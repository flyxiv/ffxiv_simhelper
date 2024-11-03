pub mod ffxiv_combat_resources;

use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::cooldown_timer::CooldownTimer;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::{Skill, SkillEvents};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::types::{ComboType, PlayerIdType, ResourceIdType, ResourceType};
use crate::types::{SkillIdType, TimeType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// Saves all the combat related resources for the player's job
// resources include stack, combo, skill list
pub(crate) trait CombatResource: Clone + Sized {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill>;
    fn get_skills(&self) -> &SkillTable<AttackSkill>;
    fn get_skill(&self, skill_id: SkillIdType) -> &AttackSkill {
        let skill = self.get_skills().get(&skill_id).unwrap();
        skill
    }

    /// Some events reduce other skill's cooldown ex) WAR's fell cleave reduce infuriate's cooldown by 5s
    fn reduce_cooldown(&mut self, skill_id: SkillIdType, reduce_amount: TimeType) {
        let skill = self.get_skills_mut().get_mut(&skill_id).unwrap();
        skill.update_cooldown(reduce_amount);
    }

    fn use_resource(&mut self, resource_id: ResourceIdType, resource: ResourceType) {
        self.add_resource(resource_id, -resource);
    }
    fn add_resource(&mut self, resource_id: ResourceIdType, resource_type: ResourceType);
    fn get_resource(&self, resource_id: ResourceIdType) -> ResourceType;
    fn get_current_combo(&self) -> ComboType;
    fn update_combo(&mut self, combo: &ComboType);
    fn start_cooldown(&mut self, skill_id: SkillIdType, player: &FfxivPlayer) {
        let skill = self.get_skills_mut().get_mut(&skill_id).unwrap();
        skill.start_cooldown(player);
    }

    // Add additional event on skill
    // that are too complex to be implemented by the current combat simulation logic
    // ex) MCH's wildfire, which stacks up damage and explodes after certain amount of time
    fn trigger_on_event(
        &mut self,
        skill_id: SkillIdType,
        buff_list: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        debuff_list: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        current_time_millisecond: TimeType,
        player: &FfxivPlayer,
    ) -> SkillEvents;

    // monk gets their chakra stack every time they crit their GCD skill.
    fn trigger_on_gcd_crit(&mut self);

    fn get_next_buff_target(&self, skill_id: SkillIdType) -> PlayerIdType;

    fn update_time(&mut self, elapsed_time: TimeType) {
        let skill_table = self.get_skills_mut();
        for skill in skill_table.values_mut() {
            skill.update_cooldown(elapsed_time);
        }
        self.update_other_time_related_states(elapsed_time);
    }

    /// This is only implemented for jobs that actually have danger of not refreshing their combo
    /// when doing their "proper" rotation due to performance reason.
    fn get_combo_remaining_time(&self) -> TimeType;

    fn update_other_time_related_states(&mut self, elapsed_time: TimeType);
}
