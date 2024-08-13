use crate::combat_resources::CombatResource;
use crate::event::ffxiv_event::FfxivEvent::Damage;
use crate::jobs_skill_data::samurai::abilities::make_samurai_skill_list;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::damage_category::DamageCategory;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::types::{ComboType, ResourceType};
use crate::types::{IdType, TimeType};
use std::cell::RefCell;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::rc::Rc;

const LUNAR_STACK_MAX: ResourceType = 1;
const STELLAR_TIMER: TimeType = 20000;

#[derive(Clone)]
pub(crate) struct AstrologianCombatResources {
    skills: SkillTable<AttackSkill>,
    player_id: IdType,
    current_combo: ComboType,
    lunar_stack: ResourceType,
    stellar_timer: Option<TimeType>,
    melee_partner: IdType,
    ranged_partner: IdType,
}

impl CombatResource for AstrologianCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: IdType, resource_amount: ResourceType) {
        if resource_id == 0 {
            self.lunar_stack = min(LUNAR_STACK_MAX, self.lunar_stack + resource_amount);
        }
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        if resource_id == 0 {
            self.lunar_stack
        } else {
            -1
        }
    }

    fn get_current_combo(&self) -> ComboType {
        self.current_combo
    }

    fn update_combo(&mut self, combo: &Option<IdType>) {
        if let Some(combo) = combo {
            self.current_combo = Some(*combo);
        }
    }

    fn trigger_on_event(
        &mut self,
        skill_id: IdType,
        buff_list: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        debuff_list: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        combat_time_millisecond: TimeType,
        _: &FfxivPlayer,
    ) -> SkillEvents {
        let mut ffxiv_event = vec![];
        if let Some(timer) = self.stellar_timer {
            if timer <= 0 {
                self.lunar_stack = 0;
                self.stellar_timer = None;
                ffxiv_event.push(Damage(
                    self.player_id,
                    self.player_id,
                    310,
                    130,
                    false,
                    false,
                    buff_list.borrow().clone(),
                    debuff_list.borrow().clone(),
                    DamageCategory::Direct,
                    combat_time_millisecond,
                ))
            }
        }

        if skill_id == 502 {
            self.stellar_timer = Some(STELLAR_TIMER);
        }

        (ffxiv_event, vec![])
    }

    fn trigger_on_crit(&mut self) {}

    fn get_next_buff_target(&self, skill_id: IdType) -> IdType {
        if skill_id == 505 {
            self.melee_partner
        } else if skill_id == 506 {
            self.ranged_partner
        } else {
            0
        }
    }
    fn update_stack_timer(&mut self, elapsed_time_millisecond: TimeType) {
        if let Some(timer) = self.stellar_timer {
            self.stellar_timer = Some(max(timer - elapsed_time_millisecond, 0));
        }
    }
}

impl AstrologianCombatResources {
    pub(crate) fn new(player_id: IdType, melee_partner: IdType, ranged_partner: IdType) -> Self {
        Self {
            skills: make_samurai_skill_list(player_id),
            player_id,
            current_combo: None,
            lunar_stack: 0,
            stellar_timer: None,
            melee_partner,
            ranged_partner,
        }
    }
}
