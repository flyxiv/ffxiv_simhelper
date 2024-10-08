use crate::combat_resources::CombatResource;
use crate::event::ffxiv_event::FfxivEvent::Damage;
use crate::jobs_skill_data::darkknight::abilities::make_darkknight_skill_list;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::damage_category::DamageCategory;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::snapshot_status::snapshot_status_infos;
use crate::types::{ComboType, PlayerIdType, PotencyType, ResourceIdType, ResourceType};
use crate::types::{SkillIdType, TimeType};
use std::cell::RefCell;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::rc::Rc;

const DARK_KNIGHT_STACKS_COUNT: usize = 2;

const MANA_MAX: ResourceType = 10000;
const BLACK_BLOOD_MAX: ResourceType = 100;

const LIVING_SHADOW_DELAY: TimeType = 5000;
const LIVING_SHADOW_POTENCY: PotencyType = 2250;
const LIVING_SHADOW_ID: SkillIdType = 211;

const DARK_KNIGHT_MAX_STACKS: [ResourceType; DARK_KNIGHT_STACKS_COUNT] =
    [MANA_MAX, BLACK_BLOOD_MAX];

#[derive(Clone)]
pub(crate) struct DarkknightCombatResources {
    skills: SkillTable<AttackSkill>,
    player_id: PlayerIdType,
    current_combo: ComboType,
    resources: [ResourceType; DARK_KNIGHT_STACKS_COUNT],
    living_shadow_delay: Option<TimeType>,
}

impl CombatResource for DarkknightCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, amount: ResourceType) {
        let resource_id = resource_id as usize;
        self.resources[resource_id] = min(
            DARK_KNIGHT_MAX_STACKS[resource_id],
            self.resources[resource_id] + amount,
        );
    }

    fn get_resource(&self, resource_id: ResourceIdType) -> ResourceType {
        self.resources[resource_id as usize]
    }

    fn get_current_combo(&self) -> ComboType {
        self.current_combo
    }

    fn update_combo(&mut self, combo: &ComboType) {
        if let Some(combo_id) = combo {
            self.current_combo = Some(*combo_id);
        }
    }

    fn trigger_on_event(
        &mut self,
        skill_id: SkillIdType,
        buff_list: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        debuff_list: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        current_time_millisecond: TimeType,
        _: &FfxivPlayer,
    ) -> SkillEvents {
        let mut ffxiv_events = vec![];
        let ffxiv_internal_events = vec![];

        if skill_id == 211 {
            self.living_shadow_delay = Some(LIVING_SHADOW_DELAY);
        }

        if matches!(self.living_shadow_delay, Some(0)) {
            ffxiv_events.push(Damage(
                self.player_id,
                LIVING_SHADOW_ID,
                LIVING_SHADOW_POTENCY,
                100,
                false,
                false,
                snapshot_status_infos(&buff_list.borrow(), &debuff_list.borrow(), self.player_id),
                DamageCategory::Direct,
                false,
                current_time_millisecond,
            ));

            self.living_shadow_delay = None;
        }

        (ffxiv_events, ffxiv_internal_events)
    }

    fn trigger_on_crit(&mut self) {}
    fn get_next_buff_target(&self, _: SkillIdType) -> PlayerIdType {
        0
    }
    fn update_stack_timer(&mut self, elapsed_time: TimeType) {
        if let Some(delay_time) = self.living_shadow_delay {
            self.living_shadow_delay = Some(max(delay_time - elapsed_time, 0))
        }
    }
}

impl DarkknightCombatResources {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        Self {
            skills: make_darkknight_skill_list(player_id),
            player_id,
            current_combo: None,
            resources: [MANA_MAX + 2000, 0],
            living_shadow_delay: None,
        }
    }
}
