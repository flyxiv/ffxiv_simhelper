use crate::combat_resources::CombatResource;
use crate::event::ffxiv_event::FfxivEvent::Damage;
use crate::event::ffxiv_player_internal_event::FfxivPlayerInternalEvent;
use crate::jobs_skill_data::darkknight::abilities::{
    darkknight_gcd_ids, make_darkknight_skill_list,
};
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::damage_category::DamageCategory;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::types::{ComboType, PlayerIdType, ResourceIdType, ResourceType};
use crate::types::{IdType, TimeType};
use std::cell::RefCell;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::rc::Rc;

const MANA_MAX: ResourceType = 10000;
const BLACK_BLOOD_MAX: ResourceType = 100;
const BLOOD_WEAPON_STACK_MAX: ResourceType = 5;

#[derive(Clone)]
pub(crate) struct DarkknightCombatResources {
    skills: SkillTable<AttackSkill>,
    player_id: PlayerIdType,
    current_combo: ComboType,
    mana: ResourceType,
    black_blood: ResourceType,
    blood_weapon_stack: ResourceType,
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
        if resource_id == 0 {
            self.mana = min(self.mana + amount, MANA_MAX);
        } else if resource_id == 1 {
            self.black_blood = min(self.black_blood + amount, BLACK_BLOOD_MAX);
        } else if resource_id == 2 {
            self.blood_weapon_stack = min(self.blood_weapon_stack + amount, BLOOD_WEAPON_STACK_MAX);
        }
    }

    fn get_resource(&self, resource_id: ResourceIdType) -> ResourceType {
        if resource_id == 0 {
            self.mana
        } else if resource_id == 1 {
            self.black_blood
        } else if resource_id == 2 {
            self.blood_weapon_stack
        } else {
            -1
        }
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
        skill_id: IdType,
        buff_list: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        debuff_list: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        current_time_millisecond: TimeType,
        _: &FfxivPlayer,
    ) -> SkillEvents {
        let mut ffxiv_events = vec![];
        let mut ffxiv_internal_events = vec![];

        if skill_id == 211 {
            self.living_shadow_delay = Some(5000);
        }

        if matches!(self.living_shadow_delay, Some(0)) {
            ffxiv_events.push(Damage(
                self.player_id,
                211,
                2250,
                100,
                false,
                false,
                buff_list.borrow().clone(),
                debuff_list.borrow().clone(),
                DamageCategory::Direct,
                current_time_millisecond,
            ));

            self.living_shadow_delay = None;
        }

        if darkknight_gcd_ids().contains(&skill_id) && self.blood_weapon_stack > 0 {
            ffxiv_internal_events.push(FfxivPlayerInternalEvent::IncreaseResource(0, 600));
            ffxiv_internal_events.push(FfxivPlayerInternalEvent::IncreaseResource(1, 10));
            self.blood_weapon_stack = max(0, self.blood_weapon_stack - 1);
        }

        (ffxiv_events, ffxiv_internal_events)
    }

    fn get_next_buff_target(&self, _: IdType) -> PlayerIdType {
        0
    }
    fn update_stack_timer(&mut self, elapsed_time: TimeType) {
        if let Some(delay_time) = self.living_shadow_delay {
            self.living_shadow_delay = Some(max(delay_time - elapsed_time, 0))
        }
    }
    fn trigger_on_crit(&mut self) {}
}

impl DarkknightCombatResources {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        Self {
            skills: make_darkknight_skill_list(player_id),
            player_id,
            current_combo: None,
            mana: MANA_MAX,
            black_blood: 0,
            blood_weapon_stack: 0,
            living_shadow_delay: None,
        }
    }
}
