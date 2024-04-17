use crate::{BuffTable, DamageType};

/// Saves the RDPS profile of the current inflicted damage.
/// Includes the total multiplier and how much each buff/debuff contributed to the total damage.
pub(crate) struct DamageRdpsProfile {
    pub(crate) raw_damage: DamageType,
    pub(crate) buff_table: BuffTable,
}

impl DamageRdpsProfile {
    pub(crate) fn get_buffs_contribution_rdps(&self, total_buff_table: ) -> BuffTable {
        &self.buff_table
    }
}
