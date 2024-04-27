use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::rotation::cooldown_timer::CooldownTimer;
use crate::rotation::priority_table::PriorityTable;
use crate::rotation::FfxivPriorityTable;
use crate::skill::attack_skill::{AttackSkill, SkillInfo};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
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

    fn use_highest_priority_skill(
        &mut self,
        buff_list: Rc<RefCell<Vec<BuffStatus>>>,
        debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
        player: &FfxivPlayer,
    ) -> Option<SkillInfo<AttackSkill>> {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => {
                sage_priority_table.use_highest_priority_skill(buff_list, debuff_list, player)
            }
        }
    }

    fn get_skills_mut(&mut self) -> &mut Vec<AttackSkill> {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => sage_priority_table.get_skills_mut(),
        }
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
