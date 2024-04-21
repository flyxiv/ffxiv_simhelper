use crate::priority_table::PriorityTable;
use crate::skill::AttackSkill;

pub(crate) struct NinjaPriorityTable {
    cooldowns: Vec<AttackSkill>,
}
