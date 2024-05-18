use crate::combat_resources::CombatResource;
use crate::event::ffxiv_event::FfxivEvent;
use crate::event::ffxiv_player_internal_event::FfxivPlayerInternalEvent;
use crate::id_entity::IdEntity;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::player_turn_calculator::SkillTimeInfo;
use crate::live_objects::player::StatusKey;
use crate::owner_tracker::OwnerTracker;
use crate::rotation::cooldown_timer::CooldownTimer;
use crate::skill::{
    ResourceRequirements, ResourceTable, Skill, SkillEvents, GCD_DEFAULT_DELAY_MILLISECOND,
    NON_GCD_DELAY_MILLISECOND,
};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{DamageType, IdType, ResourceType, StackType, StatusTable, TimeType};
use ffxiv_simbot_db::MultiplierType;
use std::cell::RefCell;
use std::cmp::max;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct AttackSkill {
    pub id: IdType,
    pub(crate) name: String,
    pub player_id: IdType,
    pub(crate) potency: DamageType,
    pub(crate) trait_multiplier: MultiplierType,

    pub buff_events: Vec<FfxivEvent>,
    pub debuff_events: Vec<FfxivEvent>,
    pub combo: Option<IdType>,

    pub delay_millisecond: Option<TimeType>,
    pub casting_time_millisecond: TimeType,
    pub gcd_cooldown_millisecond: TimeType,
    pub charging_time_millisecond: TimeType,
    pub is_speed_buffed: bool,

    pub resource_required: Vec<ResourceRequirements>,
    pub resource_created: ResourceTable,

    pub(crate) is_guaranteed_crit: bool,
    pub(crate) is_guaranteed_direct_hit: bool,

    pub(crate) cooldown_millisecond: TimeType,
    pub(crate) current_cooldown_millisecond: TimeType,
    pub(crate) stacks: StackType,
    pub(crate) stack_skill_id: Option<IdType>,
}

impl IdEntity for AttackSkill {
    fn get_id(&self) -> IdType {
        self.id
    }
}

impl Skill for AttackSkill {
    fn start_cooldown(&mut self) {
        if self.cooldown_millisecond == 0 {
            return;
        }

        self.stacks -= 1;
        self.current_cooldown_millisecond += self.cooldown_millisecond;
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
        let resource_events = self.generate_resource_events();
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

        if let Some(damage_event) = damage_event {
            ffxiv_events.push(damage_event);
        }

        let skill_events_vec = player.combat_resources.borrow().trigger_on_event(
            self.id,
            buffs.clone(),
            debuffs.clone(),
            current_combat_time_milliseconds,
            player,
        );

        for skill_events in skill_events_vec {
            ffxiv_events.extend(skill_events.0);
            internal_events.extend(skill_events.1);
        }

        (ffxiv_events, internal_events)
    }
}

impl AttackSkill {
    pub fn get_potency(&self) -> DamageType {
        (self.potency as MultiplierType * self.trait_multiplier) as DamageType
    }

    /// generate events that update the resource of the player.
    pub(crate) fn generate_resource_events(&self) -> Vec<FfxivPlayerInternalEvent> {
        let mut events = vec![];

        for resource_requirement in self.resource_required.iter() {
            if let Some(resource_event) = self.create_resource_use_event(resource_requirement) {
                events.push(resource_event)
            }
        }

        for (resource_id, resource_amount) in self.resource_created.iter() {
            events.push(FfxivPlayerInternalEvent::IncreaseResource(
                *resource_id,
                *resource_amount,
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
    ) -> Option<FfxivPlayerInternalEvent> {
        match resource_requirement {
            ResourceRequirements::Resource(stack_id, required_resource) => Some(
                FfxivPlayerInternalEvent::UseResource(*stack_id, *required_resource),
            ),
            ResourceRequirements::UseBuff(status_id) => {
                Some(FfxivPlayerInternalEvent::RemoveBuff(*status_id))
            }
            ResourceRequirements::UseDebuff(status_id) => {
                Some(FfxivPlayerInternalEvent::RemoveDebuff(*status_id))
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

        let inflict_damage_time = player.get_damage_inflict_time_millisecond(self);
        Some(FfxivEvent::Damage(
            self.player_id,
            self.id,
            self.get_potency(),
            self.is_guaranteed_crit,
            self.is_guaranteed_direct_hit,
            buffs.clone(),
            debuffs.clone(),
            current_combat_milliseconds + inflict_damage_time,
        ))
    }

    pub(crate) fn get_time_related_informations(&self, player: &FfxivPlayer) -> SkillTimeInfo {
        SkillTimeInfo {
            delay_millisecond: self.get_delay_millisecond(),
            cast_time_millisecond: player.get_cast_time(self),
            gcd_cooldown_millisecond: player.get_gcd(self),
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

    fn get_gcd_time_millisecond(&self) -> TimeType {
        self.gcd_cooldown_millisecond
    }
    pub(crate) fn get_current_cooldown_millisecond(&self) -> TimeType {
        self.current_cooldown_millisecond
    }
    fn get_gcd_cooldown_millisecond(&self) -> TimeType {
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
    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        *self.resource_created.get(&resource_id).unwrap()
    }

    #[inline]
    fn get_combo(&self) -> Option<IdType> {
        self.combo
    }

    fn get_stack(&self) -> StackType {
        f64::ceil(self.current_cooldown_millisecond as f64 / self.cooldown_millisecond as f64)
            as StackType
    }

    pub fn new(id: IdType, name: String, player_id: IdType, potency: DamageType) -> Self {
        Self {
            id,
            name,
            player_id,
            potency,
            trait_multiplier: 1.0,
            buff_events: vec![],
            debuff_events: vec![],
            combo: None,
            delay_millisecond: Some(0),
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: GCD_DEFAULT_DELAY_MILLISECOND,
            charging_time_millisecond: 0,
            is_speed_buffed: false,
            resource_required: vec![],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 0,
            stack_skill_id: None,
        }
    }
}

impl OwnerTracker for AttackSkill {
    fn get_owner_id(&self) -> IdType {
        self.player_id
    }
}

impl CooldownTimer for AttackSkill {
    fn update_cooldown(&mut self, elapsed_time: TimeType) {
        let past_stack = self.get_stack();
        self.current_cooldown_millisecond =
            max(0, self.current_cooldown_millisecond - elapsed_time);

        let current_stack = self.get_stack();

        if past_stack != current_stack {
            self.stacks += 1;
        }
    }
}
