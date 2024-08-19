pub(crate) mod ffxiv_combat_resources;

use crate::event::ffxiv_player_internal_event::FfxivPlayerInternalEvent;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::cooldown_timer::CooldownTimer;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::{Skill, SkillEvents};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::types::{ComboType, PlayerIdType, ResourceIdType, ResourceType, StackType};
use crate::types::{IdType, TimeType};
use log::info;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Saves all the combat related resources for the player's job
/// resources include stack, combo, cooldown of each skill
pub(crate) trait CombatResource: Clone + Sized {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill>;
    fn get_skills(&self) -> &SkillTable<AttackSkill>;
    fn get_skill(&self, skill_id: IdType) -> &AttackSkill {
        info!("get_skill: {}", skill_id);
        let skill = self.get_skills().get(&skill_id).unwrap();
        skill
    }

    fn reduce_cooldown(&mut self, skill_id: IdType, reduce_amount: TimeType) {
        let skill = self.get_skills_mut().get_mut(&skill_id).unwrap();
        skill.update_cooldown(reduce_amount);
    }

    fn get_stack(&self, skill_id: IdType) -> StackType {
        let skill = self.get_skill(skill_id);
        let skill_table = self.get_skills();

        let stack_skill = skill_table.get(&skill.stack_skill_id()).unwrap();
        stack_skill.stacks
    }

    fn handle_resource_event(
        &mut self,
        resource_events: &Vec<FfxivPlayerInternalEvent>,
        buff_list: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        debuff_list: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        player_id: PlayerIdType,
    ) {
        for resource in resource_events {
            match resource {
                FfxivPlayerInternalEvent::IncreaseResource(resource_id, resource) => {
                    self.add_resource(*resource_id, *resource);
                }
                FfxivPlayerInternalEvent::UseResource(resources, resource) => {
                    self.use_resource(*resources, *resource);
                }
                FfxivPlayerInternalEvent::RemoveBuff(buff_id) => {
                    let key = StatusKey::new(*buff_id, player_id);
                    let mut buff_list = buff_list.borrow_mut();
                    let mut delete = true;

                    if let Some(buff) = buff_list.get_mut(&key) {
                        buff.stacks -= 1;
                        delete = buff.stacks == 0;
                    }

                    if delete {
                        buff_list.remove(&key);
                    }
                }
                FfxivPlayerInternalEvent::RemoveDebuff(debuff_id) => {
                    let key = StatusKey::new(*debuff_id, player_id);
                    let mut debuff_list = debuff_list.borrow_mut();
                    let mut delete = true;

                    if let Some(debuff) = debuff_list.get_mut(&key) {
                        debuff.stacks -= 1;
                        delete = debuff.stacks == 0;
                    }

                    if delete {
                        debuff_list.remove(&key);
                    }
                }
                _ => {}
            }
        }
    }
    fn use_resource(&mut self, resource_id: ResourceIdType, resource: ResourceType) {
        self.add_resource(resource_id, -resource);
    }
    fn add_resource(&mut self, resource_id: ResourceIdType, resource_type: ResourceType);
    fn get_resource(&self, resource_id: ResourceIdType) -> ResourceType;

    fn get_current_combo(&self) -> ComboType;
    fn update_combo(&mut self, combo: &ComboType);
    fn start_cooldown(&mut self, skill_id: IdType, player: &FfxivPlayer) {
        let skill = self.get_skills_mut().get_mut(&skill_id).unwrap();
        skill.start_cooldown(player);
    }

    /// Add conditional trigger event on skill
    fn trigger_on_event(
        &mut self,
        skill_id: IdType,
        buff_list: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        debuff_list: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        current_time_millisecond: TimeType,
        player: &FfxivPlayer,
    ) -> SkillEvents;

    fn trigger_on_crit(&mut self);

    fn get_next_buff_target(&self, skill_id: IdType) -> PlayerIdType;

    fn update_cooldown(&mut self, elapsed_time: TimeType) {
        let skill_table = self.get_skills_mut();
        for skill in skill_table.values_mut() {
            skill.update_cooldown(elapsed_time);
        }
        self.update_stack_timer(elapsed_time);
    }

    fn update_stack_timer(&mut self, elapsed_time: TimeType);
}
