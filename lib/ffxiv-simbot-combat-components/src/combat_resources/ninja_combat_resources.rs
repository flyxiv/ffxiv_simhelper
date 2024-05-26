use crate::combat_resources::CombatResource;
use crate::id_entity::IdEntity;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::job_priorities::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::job_abilities::ninja_abilities::{
    bunshin_clone_id, bunshin_stack_id, bunshin_trigger_gcd_ids, make_ninja_skill_list,
};
use crate::skill::skill_target::SkillTarget;
use crate::skill::{Skill, SkillEvents};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{ComboType, IdType, ResourceType, TimeType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct NinjaCombatResources {
    skills: SkillTable<AttackSkill>,
    ninki: RefCell<ResourceType>,
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
            *self.ninki.borrow_mut() += resource_type;
        }
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        if resource_id == 0 {
            *self.ninki.borrow()
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
        } else {
            self.current_combo = None;
        }
    }

    fn trigger_on_event(
        &self,
        skill_id: IdType,
        buff_list: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        debuff_list: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        current_time_millisecond: TimeType,
        player: &FfxivPlayer,
    ) -> Vec<SkillEvents> {
        let mut triggered_skills = vec![];

        if bunshin_trigger_gcd_ids().contains(&skill_id) {
            let key = StatusKey::new(bunshin_stack_id(), player.get_id());

            if buff_list.borrow().contains_key(&key) {
                let bunshin_skill = self.skills.get(&bunshin_clone_id()).unwrap();
                triggered_skills.push(bunshin_skill.generate_skill_events(
                    buff_list.clone(),
                    debuff_list.clone(),
                    current_time_millisecond,
                    player,
                ));
            }
        }

        triggered_skills
    }

    fn get_next_buff_target(&self, _: IdType) -> IdType {
        0
    }
}

impl NinjaCombatResources {
    pub(crate) fn new(player_id: IdType) -> Self {
        Self {
            skills: make_ninja_skill_list(player_id),
            ninki: RefCell::new(0),
            current_combo: None,
        }
    }
}
