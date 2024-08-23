use crate::combat_resources::CombatResource;
use crate::event::ffxiv_event::FfxivEvent;
use crate::event::ffxiv_player_internal_event::FfxivPlayerInternalEvent;
use crate::event_ticker::PercentType;
use crate::id_entity::IdEntity;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::player_turn_calculator::SkillTimeInfo;
use crate::live_objects::player::StatusKey;
use crate::owner_tracker::OwnerTracker;
use crate::rotation::cooldown_timer::CooldownTimer;
use crate::skill::damage_category::DamageCategory;
use crate::skill::use_type::UseType;
use crate::skill::{
    ResourceRequirements, ResourceTable, Skill, SkillEvents, GCD_DEFAULT_DELAY_MILLISECOND,
    NON_GCD_DELAY_MILLISECOND,
};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::snapshot_status::{snapshot_buff, snapshot_debuff};
use crate::types::{ComboType, IdType, PlayerIdType, PotencyType, TimeType};
use crate::types::{ResourceType, StackType, StatusTable};
use rand::{thread_rng, Rng};
use std::cell::RefCell;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct AttackSkill {
    pub id: IdType,
    pub(crate) name: String,
    pub player_id: PlayerIdType,
    pub(crate) potency: PotencyType,
    pub(crate) trait_percent: PercentType,

    pub additional_skill_events: Vec<FfxivEvent>,
    pub proc_events: Vec<(FfxivEvent, PercentType)>,
    pub combo: ComboType,

    pub delay_millisecond: Option<TimeType>,
    pub casting_time_millisecond: TimeType,
    pub gcd_cooldown_millisecond: TimeType,
    pub charging_time_millisecond: TimeType,
    pub is_speed_buffed: bool,
    pub cooldown_reduced_by_speed: bool,

    pub resource_required: Vec<ResourceRequirements>,
    pub resource_created: ResourceTable,

    pub(crate) is_guaranteed_crit: bool,
    pub(crate) is_guaranteed_direct_hit: bool,

    pub(crate) cooldown_millisecond: TimeType,
    pub(crate) current_cooldown_millisecond: TimeType,
    pub(crate) stacks: StackType,
    pub(crate) stack_skill_id: Option<IdType>,
    pub(crate) use_type: UseType,
}

impl IdEntity for AttackSkill {
    fn get_id(&self) -> IdType {
        self.id
    }
}

impl Skill for AttackSkill {
    fn start_cooldown(&mut self, player: &FfxivPlayer) {
        if self.cooldown_millisecond == 0 {
            return;
        }

        self.stacks -= 1;

        let cooldown = if self.cooldown_reduced_by_speed {
            player.get_speed_buffed_time(self.cooldown_millisecond, false)
        } else {
            self.cooldown_millisecond
        };
        self.current_cooldown_millisecond += cooldown;
    }

    /// Generate the internal and combat events for the skill
    fn generate_skill_events(
        &self,
        buffs: StatusTable<BuffStatus>,
        debuffs: StatusTable<DebuffStatus>,
        current_combat_time_milliseconds: TimeType,
        player: &FfxivPlayer,
    ) -> SkillEvents {
        let mut internal_events = vec![];
        let resource_events = self.generate_resource_events(player);
        let cooldown_event = self.generate_cooldown_event();

        internal_events.push(cooldown_event);
        internal_events.extend(resource_events);

        let mut ffxiv_events = vec![];
        let damage_event = self.generate_damage_event(
            buffs.clone(),
            debuffs.clone(),
            current_combat_time_milliseconds,
            player,
        );
        ffxiv_events.extend(self.generate_additional_skill_events(
            player,
            buffs.clone(),
            debuffs.clone(),
            current_combat_time_milliseconds,
        ));

        if let Some(damage_event) = damage_event {
            ffxiv_events.push(damage_event);
        }

        let skill_events = player.combat_resources.borrow_mut().trigger_on_event(
            self.id,
            buffs.clone(),
            debuffs.clone(),
            current_combat_time_milliseconds,
            player,
        );

        if self.is_gcd() {
            for buffs in buffs.borrow().values() {
                ffxiv_events.extend(buffs.generate_proc_event(current_combat_time_milliseconds));
            }
        }

        ffxiv_events.extend(skill_events.0);
        internal_events.extend(skill_events.1);

        (ffxiv_events, internal_events)
    }
}

impl AttackSkill {
    pub fn get_potency(&self) -> PotencyType {
        self.potency
    }

    /// generate events that update the resource of the player.
    pub(crate) fn generate_resource_events(
        &self,
        player: &FfxivPlayer,
    ) -> Vec<FfxivPlayerInternalEvent> {
        let mut stack = 1;
        let mut events = vec![];

        for resource_requirement in self.resource_required.iter() {
            if let Some((resource_event, stacks)) =
                self.create_resource_use_event(resource_requirement, player)
            {
                stack *= stacks;
                events.push(resource_event)
            }
        }

        for (resource_id, resource_amount) in self.resource_created.iter() {
            events.push(FfxivPlayerInternalEvent::IncreaseResource(
                *resource_id,
                *resource_amount * stack,
            ));
        }

        if let Some(combo) = self.combo {
            events.push(FfxivPlayerInternalEvent::UpdateCombo(Some(combo)));
        }

        events
    }

    fn create_resource_use_event(
        &self,
        resource_requirement: &ResourceRequirements,
        player: &FfxivPlayer,
    ) -> Option<(FfxivPlayerInternalEvent, ResourceType)> {
        match resource_requirement {
            ResourceRequirements::Resource(stack_id, required_resource) => Some((
                FfxivPlayerInternalEvent::UseResource(*stack_id, *required_resource),
                1,
            )),
            ResourceRequirements::UseBuff(status_id) => {
                Some((FfxivPlayerInternalEvent::RemoveBuff(*status_id), 1))
            }
            ResourceRequirements::UseDebuff(status_id) => {
                Some((FfxivPlayerInternalEvent::RemoveDebuff(*status_id), 1))
            }
            ResourceRequirements::UseAllResource(resource_id) => {
                let resource_amount = player.combat_resources.borrow().get_resource(*resource_id);
                Some((
                    FfxivPlayerInternalEvent::UseResource(*resource_id, resource_amount),
                    resource_amount,
                ))
            }
            _ => None,
        }
    }

    fn generate_damage_event(
        &self,
        buffs: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        debuffs: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        current_combat_milliseconds: TimeType,
        player: &FfxivPlayer,
    ) -> Option<FfxivEvent> {
        if self.potency == 0 {
            return None;
        }

        let mut stack_multiplier = 1;

        for resource_required in &self.resource_required {
            match resource_required {
                ResourceRequirements::UseAllResource(resource_id) => {
                    stack_multiplier *= player.combat_resources.borrow().get_resource(*resource_id);
                }
                _ => {}
            }
        }

        let inflict_damage_time = player.get_damage_inflict_time_millisecond(self);
        Some(FfxivEvent::Damage(
            self.player_id,
            self.id,
            self.get_potency() * (stack_multiplier as PotencyType),
            self.trait_percent,
            self.is_guaranteed_crit,
            self.is_guaranteed_direct_hit,
            snapshot_buff(&buffs.borrow()),
            snapshot_debuff(&debuffs.borrow(), self.player_id),
            DamageCategory::Direct,
            current_combat_milliseconds + inflict_damage_time,
        ))
    }

    pub(crate) fn generate_additional_skill_events(
        &self,
        player: &FfxivPlayer,
        buffs: StatusTable<BuffStatus>,
        debuffs: StatusTable<DebuffStatus>,
        combat_time_millisecond: TimeType,
    ) -> Vec<FfxivEvent> {
        let mut additional_skill_events = vec![];
        let resource_table = player.combat_resources.borrow();

        for additional_skill_event in self.additional_skill_events.clone() {
            let mut event = match self.use_type {
                UseType::UseOnPartyMember => {
                    let mut event =
                        additional_skill_event.add_time_to_event(combat_time_millisecond);
                    let buff_target = resource_table.get_next_buff_target(self.get_id());
                    event.set_target(buff_target);
                    event
                }
                UseType::NoTarget => {
                    let event = additional_skill_event.add_time_to_event(combat_time_millisecond);
                    event
                }
                _ => additional_skill_event.add_time_to_event(combat_time_millisecond),
            };
            event.snapshot_status(&buffs.borrow(), &debuffs.borrow());

            for resource_required in &self.resource_required {
                match resource_required {
                    ResourceRequirements::UseAllResource(resource_id) => {
                        let resource_amount = resource_table.get_resource(*resource_id);
                        event.set_stacks(resource_amount);
                    }
                    _ => {}
                }
            }

            additional_skill_events.push(event);
        }

        additional_skill_events.extend(self.generate_proc_event(combat_time_millisecond));

        additional_skill_events
    }

    pub(crate) fn get_time_related_informations(&self, player: &FfxivPlayer) -> SkillTimeInfo {
        SkillTimeInfo {
            delay_millisecond: self.get_delay_millisecond(),
            cast_time_millisecond: player.get_cast_time(self),
            gcd_cooldown_millisecond: player.get_gcd_delay_millisecond(self),
            charge_time_millisecond: self.charging_time_millisecond,
        }
    }

    fn generate_cooldown_event(&self) -> FfxivPlayerInternalEvent {
        if let Some(stack_skill_id) = self.stack_skill_id {
            FfxivPlayerInternalEvent::StartCooldown(stack_skill_id)
        } else {
            FfxivPlayerInternalEvent::StartCooldown(self.id)
        }
    }

    /// All FFXIV Offensive Skills can be double-weaved except for Stardiver, so
    /// just give a default of 700ms, which is right for almost all skills.
    pub(crate) fn get_delay_millisecond(&self) -> TimeType {
        if let Some(delay) = self.delay_millisecond {
            delay
        } else {
            NON_GCD_DELAY_MILLISECOND
        }
    }

    #[inline]
    fn is_gcd(&self) -> bool {
        self.gcd_cooldown_millisecond > 0
    }
    pub(crate) fn get_gcd_cast_time(&self) -> TimeType {
        self.casting_time_millisecond
    }

    pub(crate) fn get_current_cooldown_millisecond(&self) -> TimeType {
        self.current_cooldown_millisecond
    }
    pub(crate) fn get_gcd_cooldown_millisecond(&self) -> TimeType {
        max(self.gcd_cooldown_millisecond, self.casting_time_millisecond)
    }

    fn is_ready(&self) -> bool {
        self.stacks >= 1
    }

    pub(crate) fn is_speed_buffed(&self) -> bool {
        self.is_speed_buffed
    }

    pub(crate) fn stack_skill_id(&self) -> IdType {
        if let Some(skill_id) = self.stack_skill_id {
            skill_id
        } else {
            self.id
        }
    }

    #[inline]
    fn is_auto_attack(&self) -> bool {
        self.id == 0
    }

    #[inline]
    fn get_name(&self) -> &String {
        &self.name
    }

    #[inline]
    fn get_combo(&self) -> ComboType {
        self.combo
    }

    #[inline]
    fn get_stack(&self) -> StackType {
        f64::ceil(self.current_cooldown_millisecond as f64 / self.cooldown_millisecond as f64)
            as StackType
    }

    pub(crate) fn generate_proc_event(
        &self,
        current_time_millisecond: TimeType,
    ) -> Vec<FfxivEvent> {
        let proc_value = thread_rng().gen_range(0..100);
        let mut proc_events = vec![];

        for (proc_event, proc_percent) in self.proc_events.iter() {
            if proc_value <= *proc_percent {
                let proc_event = proc_event.clone();
                proc_events.push(proc_event.add_time_to_event(current_time_millisecond));
            }
        }

        proc_events
    }

    pub fn new(id: IdType, name: String, player_id: PlayerIdType, potency: PotencyType) -> Self {
        Self {
            id,
            name,
            player_id,
            potency,
            trait_percent: 100,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: None,
            delay_millisecond: Some(0),
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: GCD_DEFAULT_DELAY_MILLISECOND,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            resource_required: vec![],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 0,
            stack_skill_id: None,
            use_type: UseType::UseOnTarget,
        }
    }
}

impl OwnerTracker for AttackSkill {
    fn get_owner_id(&self) -> PlayerIdType {
        self.player_id
    }
}

impl CooldownTimer for AttackSkill {
    fn update_cooldown(&mut self, elapsed_time: TimeType) {
        if self.current_cooldown_millisecond <= 0 || elapsed_time <= 0 {
            return;
        }

        let past_stack = self.get_stack();
        self.current_cooldown_millisecond =
            max(0, self.current_cooldown_millisecond - elapsed_time);

        let current_stack = self.get_stack();

        if past_stack != current_stack {
            self.stacks += 1;
        }
    }
}
