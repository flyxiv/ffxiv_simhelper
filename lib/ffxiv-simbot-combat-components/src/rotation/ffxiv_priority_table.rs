use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::rotation::cooldown_timer::CooldownTimer;
use crate::rotation::job_priorities::SkillTable;
use crate::rotation::priority_table::PriorityTable;
use crate::rotation::{FfxivPriorityTable, SkillPriorityInfo};
use crate::skill::attack_skill::{AttackSkill, SkillInfo};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{IdType, ResourceType, StackType};
use std::cell::RefCell;
use std::rc::Rc;

impl CooldownTimer for FfxivPriorityTable {
    fn update_cooldown(&mut self, elapsed_time: i32) {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => {
                sage_priority_table.update_cooldown(elapsed_time)
            }
        }
    }
}

impl PriorityTable<AttackSkill, FfxivPlayer> for FfxivPriorityTable {
    fn get_next_skill(
        &mut self,
        buff_list: Rc<RefCell<Vec<BuffStatus>>>,
        debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
        player: &FfxivPlayer,
    ) -> Option<SkillInfo<AttackSkill>> {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => {
                sage_priority_table.get_next_skill(buff_list, debuff_list, player)
            }
        }
    }

    fn is_opener(&self, player_turn: &FfxivTurnType) -> bool {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => {
                sage_priority_table.is_opener(player_turn)
            }
        }
    }

    fn use_opener(&self, player_turn: &FfxivTurnType) -> Option<SkillInfo<AttackSkill>> {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => {
                sage_priority_table.use_opener(player_turn)
            }
        }
    }

    fn get_highest_priority_skill(
        &mut self,
        buff_list: Rc<RefCell<Vec<BuffStatus>>>,
        debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
        player: &FfxivPlayer,
        turn_type: &FfxivTurnType,
    ) -> Option<SkillInfo<AttackSkill>> {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => sage_priority_table
                .get_highest_priority_skill(buff_list, debuff_list, player, turn_type),
        }
    }

    fn get_skills_mut(&mut self) -> &mut SkillTable {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => sage_priority_table.get_skills_mut(),
        }
    }

    fn get_opener(&self) -> Option<SkillInfo<FfxivPlayer>> {
        todo!()
    }

    fn get_resource(&self) -> ResourceType {
        todo!()
    }

    fn get_skill_stack(&self, skill_id: IdType) -> StackType {
        todo!()
    }

    fn get_priority_table(&self, turn_type: &FfxivTurnType) -> &Vec<SkillPriorityInfo> {
        todo!()
    }

    fn is_guaranteed_crit(&self, skill: &FfxivPlayer) -> bool {
        todo!()
    }

    fn is_guaranteed_direct_hit(&self, skill: &FfxivPlayer) -> bool {
        todo!()
    }

    fn get_current_combo(&self) -> Option<IdType> {
        todo!()
    }
}

impl CooldownTimer for FfxivPriorityTable {
    fn update_cooldown(&mut self, elapsed_time: i32) {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => {
                sage_priority_table.update_cooldown(elapsed_time)
            }
        }
    }
}
