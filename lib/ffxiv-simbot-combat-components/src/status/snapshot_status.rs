use crate::live_objects::player::StatusKey;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::types::PlayerIdType;
use std::collections::HashMap;

pub(crate) fn snapshot_buff(
    buffs: &HashMap<StatusKey, BuffStatus>,
) -> HashMap<StatusKey, BuffStatus> {
    let mut snapshot = HashMap::new();
    for (key, buff) in buffs.iter() {
        if buff.is_damage_buff() {
            snapshot.insert(key.clone(), buff.clone());
        }
    }
    snapshot
}

pub(crate) fn snapshot_debuff(
    debuffs: &HashMap<StatusKey, DebuffStatus>,
    player_id: PlayerIdType,
) -> HashMap<StatusKey, DebuffStatus> {
    let mut snapshot = HashMap::new();
    for (key, debuff) in debuffs.iter() {
        if debuff.is_damage_debuff(player_id) {
            snapshot.insert(key.clone(), debuff.clone());
        }
    }
    snapshot
}
