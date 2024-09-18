use crate::combat_resources::ffxiv_combat_resources::FfxivCombatResources;
use crate::event::turn_info::TurnInfo;
use crate::live_objects::player::StatusKey;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::types::{PlayerIdType, TimeType};
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub(crate) struct InformationNeededForRotationDecision<'a> {
    pub(crate) buffs: &'a HashMap<StatusKey, BuffStatus>,
    pub(crate) debuffs: &'a HashMap<StatusKey, DebuffStatus>,
    pub(crate) combat_resources: &'a FfxivCombatResources,
    pub(crate) player_id: PlayerIdType,
    pub(crate) milliseconds_before_burst: TimeType,
}

impl<'a> InformationNeededForRotationDecision<'a> {
    pub(crate) fn new(
        buffs: &'a HashMap<StatusKey, BuffStatus>,
        debuffs: &'a HashMap<StatusKey, DebuffStatus>,
        resources: &'a FfxivCombatResources,
        turn_info: &TurnInfo,
        player_id: PlayerIdType,
    ) -> InformationNeededForRotationDecision<'a> {
        InformationNeededForRotationDecision {
            buffs,
            debuffs,
            combat_resources: resources,
            player_id,
            milliseconds_before_burst: turn_info.get_next_burst_time(),
        }
    }
}
