use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::paladin::abilities::make_paladin_skill_list;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::types::{ComboType, PlayerIdType, ResourceIdType, ResourceType};
use crate::types::{SkillIdType, TimeType};
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

/// Paladin combat resources
///
/// # resources
/// 0: Blade of honor ready
/// 1: Blade of faith ready
/// 2: Blade of truth ready
/// 3: Blade of valor ready
///
#[derive(Clone)]
pub(crate) struct PaladinCombatResources {
    skills: SkillTable<AttackSkill>,
    current_combo: ComboType,
    resources: [ResourceType; 4],
}

impl CombatResource for PaladinCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, resource_amount: ResourceType) {
        let resource_id = resource_id as usize;
        self.resources[resource_id] = min(1, self.resources[resource_id] + resource_amount);
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
        _: SkillIdType,
        _: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        _: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        _: TimeType,
        _: &FfxivPlayer,
    ) -> SkillEvents {
        (vec![], vec![])
    }

    fn trigger_on_gcd_crit(&mut self) {}
    fn get_next_buff_target(&self, _: SkillIdType) -> PlayerIdType {
        0
    }
    fn update_other_time_related_states(&mut self, _: TimeType) {}
    fn get_combo_remaining_time(&self) -> TimeType {
        0
    }
}

impl PaladinCombatResources {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        Self {
            skills: make_paladin_skill_list(player_id),
            current_combo: None,
            resources: [0, 0, 0, 0],
        }
    }
}
