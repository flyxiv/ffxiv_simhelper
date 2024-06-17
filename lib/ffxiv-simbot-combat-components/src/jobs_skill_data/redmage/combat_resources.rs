use crate::combat_resources::CombatResource;
use crate::event::ffxiv_event::FfxivEvent;
use crate::id_entity::IdEntity;
use crate::jobs_skill_data::ninja::abilities::{
    bunshin_clone_id, bunshin_stack_id, bunshin_trigger_gcd_ids, make_ninja_skill_list,
};
use crate::jobs_skill_data::redmage::abilities::{make_redmage_skill_list, manafication_gcd_id};
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::damage_category::DamageCategory;
use crate::skill::{Skill, SkillEvents};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{ComboType, DamageType, IdType, PotencyType, ResourceType, TimeType};
use ffxiv_simbot_db::MultiplierType;
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

const MANA_MAX: ResourceType = 100;
const VERSTACK_MAX: ResourceType = 3;

#[derive(Clone)]
pub(crate) struct RedmageCombatResources {
    skills: SkillTable<AttackSkill>,
    white_mana: ResourceType,
    black_mana: ResourceType,
    verstack: ResourceType,
    current_combo: Option<IdType>,
}

impl CombatResource for RedmageCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: IdType, resource_type: ResourceType) {
        if resource_id == 0 {
            self.white_mana = min(self.white_mana + resource_type, MANA_MAX);
        } else if resource_id == 1 {
            self.black_mana = min(self.black_mana + resource_type, MANA_MAX);
        } else if resource_id == 2 {
            self.verstack = min(self.verstack + resource_type, VERSTACK_MAX);
        }
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        if resource_id == 0 {
            self.white_mana
        } else if resource_id == 1 {
            self.black_mana
        } else if resource_id == 2 {
            self.verstack
        } else {
            0
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
        let mut ffxiv_events = vec![];

        if manafication_gcd_id().contains(&skill_id) {
            if buff_list
                .borrow()
                .contains_key(&StatusKey::new(1801, player.get_id()))
            {
                let skill = self.skills.get(&skill_id).unwrap();
                ffxiv_events.push(FfxivEvent::Damage(
                    player.get_id(),
                    skill_id,
                    (skill.get_potency() as MultiplierType * 0.05) as DamageType,
                    skill.trait_percent,
                    false,
                    false,
                    buff_list.borrow().clone(),
                    debuff_list.borrow().clone(),
                    DamageCategory::Direct,
                    current_time_millisecond,
                ))
            }
        }

        return (ffxiv_events, vec![]);
    }

    fn get_next_buff_target(&self, _: IdType) -> IdType {
        0
    }

    fn update_stack_timer(&mut self, _: TimeType) {}
    fn trigger_on_crit(&mut self) {}
}

impl RedmageCombatResources {
    pub(crate) fn new(player_id: IdType) -> Self {
        Self {
            skills: make_redmage_skill_list(player_id),
            white_mana: 0,
            black_mana: 0,
            current_combo: None,
            verstack: 0,
        }
    }
}
