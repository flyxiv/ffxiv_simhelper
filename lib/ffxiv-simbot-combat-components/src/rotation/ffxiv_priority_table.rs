use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::rotation::cooldown_timer::CooldownTimer;
use crate::rotation::job_priorities::SkillTable;
use crate::rotation::priority_table::PriorityTable;
use crate::rotation::{FfxivPriorityTable, SkillPriorityInfo};
use crate::skill::attack_skill::{AttackSkill, SkillInfo};
use crate::{IdType, ResourceType, StackType, TimeType};

impl PriorityTable<FfxivPlayer, AttackSkill> for FfxivPriorityTable {
    fn add_resource1(&self, resource: ResourceType) {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => {
                sage_priority_table.add_resource1(resource)
            }
            FfxivPriorityTable::Ninja(ninja_priority_table) => {
                ninja_priority_table.add_resource1(resource)
            }
        }
    }

    fn add_resource2(&self, resource: ResourceType) {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => {
                sage_priority_table.add_resource2(resource)
            }
            FfxivPriorityTable::Ninja(ninja_priority_table) => {
                ninja_priority_table.add_resource2(resource)
            }
        }
    }

    fn update_combo(&mut self, combo_id: Option<IdType>) {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => {
                sage_priority_table.update_combo(combo_id)
            }
            FfxivPriorityTable::Ninja(ninja_priority_table) => {
                ninja_priority_table.update_combo(combo_id)
            }
        }
    }

    fn get_opener_len(&self) -> usize {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => sage_priority_table.get_opener_len(),
            FfxivPriorityTable::Ninja(ninja_priority_table) => {
                ninja_priority_table.get_opener_len()
            }
        }
    }

    fn get_opener_at(&self, index: usize) -> &Option<AttackSkill> {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => {
                sage_priority_table.get_opener_at(index)
            }
            FfxivPriorityTable::Ninja(ninja_priority_table) => {
                ninja_priority_table.get_opener_at(index)
            }
        }
    }

    fn get_turn_count(&self) -> IdType {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => sage_priority_table.get_turn_count(),
            FfxivPriorityTable::Ninja(ninja_priority_table) => {
                ninja_priority_table.get_turn_count()
            }
        }
    }

    fn increment_turn(&mut self) {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => sage_priority_table.increment_turn(),
            FfxivPriorityTable::Ninja(ninja_priority_table) => {
                ninja_priority_table.increment_turn()
            }
        }
    }

    fn add_additional_skills(
        &self,
        skill: &Vec<SkillInfo<AttackSkill>>,
        player: &FfxivPlayer,
    ) -> Vec<SkillInfo<AttackSkill>> {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => {
                sage_priority_table.add_additional_skills(skill, player)
            }
            FfxivPriorityTable::Ninja(ninja_priority_table) => {
                ninja_priority_table.add_additional_skills(skill, player)
            }
        }
    }

    fn get_skills_mut(&mut self) -> &mut SkillTable {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => sage_priority_table.get_skills_mut(),
            FfxivPriorityTable::Ninja(ninja_priority_table) => {
                ninja_priority_table.get_skills_mut()
            }
        }
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => {
                sage_priority_table.get_resource(resource_id)
            }
            FfxivPriorityTable::Ninja(ninja_priority_table) => {
                ninja_priority_table.get_resource(resource_id)
            }
        }
    }

    fn get_skill_stack(&self, skill_id: IdType) -> StackType {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => {
                sage_priority_table.get_skill_stack(skill_id)
            }
            FfxivPriorityTable::Ninja(ninja_priority_table) => {
                ninja_priority_table.get_skill_stack(skill_id)
            }
        }
    }

    fn get_priority_table(
        &self,
        turn_type: &FfxivTurnType,
    ) -> &Vec<SkillPriorityInfo<AttackSkill>> {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => {
                sage_priority_table.get_priority_table(turn_type)
            }
            FfxivPriorityTable::Ninja(ninja_priority_table) => {
                ninja_priority_table.get_priority_table(turn_type)
            }
        }
    }

    fn is_guaranteed_crit(&self, skill: &AttackSkill) -> bool {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => {
                sage_priority_table.is_guaranteed_crit(skill)
            }
            FfxivPriorityTable::Ninja(ninja_priority_table) => {
                ninja_priority_table.is_guaranteed_crit(skill)
            }
        }
    }

    fn is_guaranteed_direct_hit(&self, skill: &AttackSkill) -> bool {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => {
                sage_priority_table.is_guaranteed_direct_hit(skill)
            }
            FfxivPriorityTable::Ninja(ninja_priority_table) => {
                ninja_priority_table.is_guaranteed_direct_hit(skill)
            }
        }
    }

    fn get_current_combo(&self) -> Option<IdType> {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => {
                sage_priority_table.get_current_combo()
            }
            FfxivPriorityTable::Ninja(ninja_priority_table) => {
                ninja_priority_table.get_current_combo()
            }
        }
    }
}

impl CooldownTimer for FfxivPriorityTable {
    fn update_cooldown(&mut self, elapsed_time: TimeType) {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => {
                sage_priority_table.update_cooldown(elapsed_time)
            }
            FfxivPriorityTable::Ninja(ninja_priority_table) => {
                ninja_priority_table.update_cooldown(elapsed_time)
            }
        }
    }
}
