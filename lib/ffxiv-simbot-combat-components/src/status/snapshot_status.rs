use crate::damage_calculator::{
    DIRECT_HIT_DAMAGE_MULTIPLIER, INCREASE_BASE, MULTIPLIER_BASE, ONE_HUNDRED_PERCENT,
};
use crate::live_objects::player::StatusKey;
use crate::owner_tracker::OwnerTracker;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_info::StatusInfo;
use crate::types::{MultiplierType, PlayerIdType, SnapshotTable};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq)]
pub struct SnapshotInfo {
    pub owner_player_id: PlayerIdType,
    pub status_infos: Vec<StatusInfo>,
}

impl SnapshotInfo {
    pub fn new(owner_player_id: PlayerIdType, status_infos: Vec<StatusInfo>) -> Self {
        Self {
            owner_player_id,
            status_infos,
        }
    }

    pub(crate) fn get_critical_strike_rate_increase(
        &self,
        is_guaranteed_crit: bool,
    ) -> MultiplierType {
        let mut critical_strike_rate_increase = 0.0;

        if is_guaranteed_crit {
            return critical_strike_rate_increase;
        }

        for status_info in &self.status_infos {
            match status_info {
                StatusInfo::CritHitRatePercent(percent) => {
                    critical_strike_rate_increase += (*percent as MultiplierType) / 100.0
                }
                _ => {}
            };
        }

        critical_strike_rate_increase
    }

    pub(crate) fn get_direct_hit_rate_increase(
        &self,
        is_guaranteed_direct_hit: bool,
    ) -> MultiplierType {
        let mut direct_hit_rate_increase = 0.0;

        if is_guaranteed_direct_hit {
            return direct_hit_rate_increase;
        }

        for status_info in &self.status_infos {
            match status_info {
                StatusInfo::DirectHitRatePercent(percent) => {
                    direct_hit_rate_increase += (*percent as MultiplierType) / 100.0
                }
                _ => {}
            };
        }

        direct_hit_rate_increase
    }

    pub(crate) fn get_damage_multiplier(
        &self,
        is_guaranteed_crit: bool,
        is_guaranteed_direct_hit: bool,
        crit_damage_rate: MultiplierType,
    ) -> MultiplierType {
        let mut total_damage_multiplier = MULTIPLIER_BASE;
        for status_info in &self.status_infos {
            match status_info {
                StatusInfo::DamagePercent(percent) => {
                    total_damage_multiplier *=
                        MULTIPLIER_BASE + (*percent as MultiplierType) / ONE_HUNDRED_PERCENT
                }
                StatusInfo::CritHitRatePercent(percent) => {
                    let damage_increase = if is_guaranteed_crit {
                        (*percent as MultiplierType) / ONE_HUNDRED_PERCENT
                            * (crit_damage_rate - MULTIPLIER_BASE)
                    } else {
                        INCREASE_BASE
                    };

                    total_damage_multiplier *= MULTIPLIER_BASE + damage_increase;
                }
                StatusInfo::DirectHitRatePercent(percent) => {
                    let damage_increase = if is_guaranteed_direct_hit {
                        (*percent as MultiplierType) / ONE_HUNDRED_PERCENT
                            * (DIRECT_HIT_DAMAGE_MULTIPLIER - MULTIPLIER_BASE)
                    } else {
                        INCREASE_BASE
                    };

                    total_damage_multiplier *= MULTIPLIER_BASE + damage_increase;
                }
                _ => {}
            };
        }

        total_damage_multiplier
    }
}
pub fn snapshot_status_infos(
    buffs: &HashMap<StatusKey, BuffStatus>,
    debuffs: &HashMap<StatusKey, DebuffStatus>,
    player_id: PlayerIdType,
) -> SnapshotTable {
    let snapshot_buffs = snapshot_buff(buffs);
    let snapshot_debuffs = snapshot_debuff(debuffs, player_id);

    let mut snapshot = HashMap::new();
    for (key, infos) in snapshot_buffs.into_iter() {
        snapshot.insert(key, infos);
    }
    for (key, infos) in snapshot_debuffs.into_iter() {
        snapshot.insert(key, infos);
    }

    snapshot
}

fn snapshot_buff(buffs: &HashMap<StatusKey, BuffStatus>) -> SnapshotTable {
    let mut snapshot = HashMap::new();
    for (&key, buff) in buffs.iter() {
        let damage_infos_of_buff = buff.get_damage_buff_infos();
        if damage_infos_of_buff.len() > 0 {
            snapshot.insert(
                key,
                SnapshotInfo::new(buff.get_owner_id(), damage_infos_of_buff),
            );
        }
    }
    snapshot
}

fn snapshot_debuff(
    debuffs: &HashMap<StatusKey, DebuffStatus>,
    player_id: PlayerIdType,
) -> SnapshotTable {
    let mut snapshot = HashMap::new();
    for (&key, debuff) in debuffs.iter() {
        let damage_infos_of_buff = debuff.get_damage_buff_infos(player_id);
        if damage_infos_of_buff.len() > 0 {
            snapshot.insert(
                key,
                SnapshotInfo::new(debuff.get_owner_id(), damage_infos_of_buff),
            );
        }
    }
    snapshot
}
