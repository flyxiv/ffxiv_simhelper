use crate::combat_resources::CombatResource;
use crate::event::ffxiv_event::FfxivEvent;
use crate::id_entity::IdEntity;
use crate::jobs_skill_data::ninja::abilities::{
    bunshin_clone_id, bunshin_stack_id, bunshin_trigger_gcd_ids, make_ninja_skill_list,
};
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::{Skill, SkillEvents};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{ComboType, IdType, ResourceType, TimeType};
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

const NINKI_MAX: ResourceType = 100;

#[derive(Clone)]
pub(crate) struct NinjaCombatResources {
    skills: SkillTable<AttackSkill>,
    ninki: ResourceType,
    current_combo: Option<IdType>,
}

impl CombatResource for NinjaCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: IdType, resource_type: ResourceType) {
        if resource_id == 0 {
            let ninki = self.ninki;
            self.ninki = min(NINKI_MAX, ninki + resource_type);
        }
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        if resource_id == 0 {
            self.ninki
        } else {
            -1
        }
    }

    fn get_current_combo(&self) -> ComboType {
        self.current_combo
    }

    fn update_combo(&mut self, combo: &Option<IdType>) {
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
        player: &FfxivPlayer,
    ) -> SkillEvents {
        if bunshin_trigger_gcd_ids().contains(&skill_id) {
            let key = StatusKey::new(bunshin_stack_id(), player.get_id());

            if buff_list.borrow().contains_key(&key) {
                return (
                    vec![FfxivEvent::UseSkill(
                        player.get_id(),
                        None,
                        bunshin_clone_id(),
                        current_time_millisecond,
                    )],
                    vec![],
                );
            }
        }

        return (vec![], vec![]);
    }

    fn get_next_buff_target(&self, _: IdType) -> IdType {
        0
    }

    fn update_stack_timer(&mut self, _: TimeType) {}
}

impl NinjaCombatResources {
    pub(crate) fn new(player_id: IdType) -> Self {
        Self {
            skills: make_ninja_skill_list(player_id),
            ninki: 0,
            current_combo: None,
        }
    }
}
