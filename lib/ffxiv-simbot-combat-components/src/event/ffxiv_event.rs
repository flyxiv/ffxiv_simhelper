use crate::event::turn_info::TurnInfo;
use crate::event_ticker::ffxiv_event_ticker::FfxivEventTicker;
use crate::event_ticker::TickerKey;
use crate::live_objects::player::StatusKey;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{DamageType, IdType, ResourceType, StatusTable, TimeType};
use std::cmp::min;
use std::collections::HashMap;

/// All possible damage related events in a FFXIV combat.
/// the last TimeType element is always the time of the event.
#[derive(Clone)]
pub enum FfxivEvent {
    /// owner_player_id, turn, threshold limit time
    PlayerTurn(IdType, FfxivTurnType, TimeType, TimeType),
    /// player_id, target_id, skill_id
    UseSkill(IdType, Option<IdType>, IdType, TimeType),

    /// owner_player_id, skill ID, potency, guaranteed crit, guaranteed direct hit, snapshotted buffs, snapshotted debuffs,
    Damage(
        IdType,
        IdType,
        DamageType,
        bool,
        bool,
        HashMap<StatusKey, BuffStatus>,
        HashMap<StatusKey, DebuffStatus>,
        TimeType,
    ),

    /// ticker key
    Tick(TickerKey, TimeType),
    /// ticker
    AddTicker(FfxivEventTicker, TimeType),
    /// ticker key
    RemoveTicker(TickerKey, TimeType),
    /// ticker key
    ForceTicker(TickerKey, TimeType),

    /// owner_player_id, target_id, status, duration, max refresh duration
    ApplyBuff(IdType, IdType, BuffStatus, TimeType, TimeType, TimeType),
    /// owner_player_id, target_id, status, status time, refresh duration or not
    ApplyBuffStack(IdType, IdType, BuffStatus, TimeType, bool, TimeType),
    /// owner_player_id, status, duration, max refresh duration
    ApplyRaidBuff(IdType, BuffStatus, TimeType, TimeType, TimeType),
    /// owner_player_id, target_id, status, duration, max refresh duration
    RefreshBuff(IdType, IdType, BuffStatus, TimeType, TimeType, TimeType),

    /// owner_player_id, time, status time, refresh duration or not
    ApplyDebuffStack(IdType, DebuffStatus, TimeType, bool, TimeType),
    /// owner_player_id, status, status time, max refresh duration
    ApplyDebuff(IdType, DebuffStatus, TimeType, TimeType, TimeType),

    /// owner_player_id, target_player_id, buff_id
    RemoveTargetBuff(IdType, IdType, IdType, TimeType),
    /// owner_player_id, buff_id
    RemoveRaidBuff(IdType, IdType, TimeType),
    /// owner_player_id, debuff_id
    RemoveDebuff(IdType, IdType, TimeType),

    /// Raises stack of another player, for dance partners and brotherhood
    /// player_id, stack id, increase amount
    IncreasePlayerResource(IdType, IdType, ResourceType, TimeType),
    /// player_id, skill_id, reduce_amount
    ReduceSkillCooldown(IdType, IdType, TimeType, TimeType),
    DotTick(TimeType),
}

impl FfxivEvent {
    pub fn get_event_time(&self) -> TimeType {
        match self {
            FfxivEvent::PlayerTurn(_, _, _, time)
            | FfxivEvent::UseSkill(_, _, _, time)
            | FfxivEvent::Damage(_, _, _, _, _, _, _, time)
            | FfxivEvent::Tick(_, time)
            | FfxivEvent::AddTicker(_, time)
            | FfxivEvent::RemoveTicker(_, time)
            | FfxivEvent::ForceTicker(_, time)
            | FfxivEvent::ApplyBuff(_, _, _, _, _, time)
            | FfxivEvent::ApplyBuffStack(_, _, _, _, _, time)
            | FfxivEvent::ApplyRaidBuff(_, _, _, _, time)
            | FfxivEvent::RefreshBuff(_, _, _, _, _, time)
            | FfxivEvent::ApplyDebuffStack(_, _, _, _, time)
            | FfxivEvent::ApplyDebuff(_, _, _, _, time)
            | FfxivEvent::RemoveTargetBuff(_, _, _, time)
            | FfxivEvent::RemoveRaidBuff(_, _, time)
            | FfxivEvent::RemoveDebuff(_, _, time)
            | FfxivEvent::IncreasePlayerResource(_, _, _, time)
            | FfxivEvent::ReduceSkillCooldown(_, _, _, time)
            | FfxivEvent::DotTick(time) => *time,
        }
    }

    pub(crate) fn set_target(&mut self, target_id: IdType) {
        match self {
            FfxivEvent::ApplyBuff(_, target, _, _, _, _)
            | FfxivEvent::ApplyBuffStack(_, target, _, _, _, _)
            | FfxivEvent::RemoveTargetBuff(_, target, _, _)
            | FfxivEvent::RefreshBuff(_, target, _, _, _, _) => *target = target_id,
            _ => {}
        }
    }

    pub(crate) fn snapshot_status(
        &mut self,
        snapshotted_buffs: StatusTable<BuffStatus>,
        snapshotted_debuffs: StatusTable<DebuffStatus>,
    ) {
        match self {
            FfxivEvent::ApplyDebuff(_, status, _, _, _) => {
                status.snapshotted_buffs = snapshotted_buffs.borrow().clone();
                status.snapshotted_debuffs = snapshotted_debuffs.borrow().clone();
            }
            _ => {}
        }
    }

    pub fn add_time_to_event(self, elapsed_time: TimeType) -> FfxivEvent {
        match self {
            FfxivEvent::Damage(
                player_id,
                skill_id,
                potency,
                is_crit,
                is_direct_hit,
                buffs,
                debuffs,
                time,
            ) => FfxivEvent::Damage(
                player_id,
                skill_id,
                potency,
                is_crit,
                is_direct_hit,
                buffs.clone(),
                debuffs.clone(),
                elapsed_time + time,
            ),
            FfxivEvent::Tick(ticker_id, time) => FfxivEvent::Tick(ticker_id, elapsed_time + time),
            FfxivEvent::AddTicker(ticker, time) => {
                FfxivEvent::AddTicker(ticker, elapsed_time + time)
            }
            FfxivEvent::RemoveTicker(ticker_id, time) => {
                FfxivEvent::RemoveTicker(ticker_id, elapsed_time + time)
            }
            FfxivEvent::ForceTicker(ticker_key, time) => {
                FfxivEvent::ForceTicker(ticker_key, elapsed_time + time)
            }
            FfxivEvent::UseSkill(player_id, target_id, skill_id, time) => {
                FfxivEvent::UseSkill(player_id, target_id, skill_id, elapsed_time + time)
            }
            FfxivEvent::ApplyBuff(
                player_id,
                target_id,
                buff,
                duration,
                max_refresh_duration,
                time,
            ) => FfxivEvent::ApplyBuff(
                player_id,
                target_id,
                buff.clone(),
                duration,
                max_refresh_duration,
                elapsed_time + time,
            ),
            FfxivEvent::ApplyBuffStack(
                player_id,
                target_id,
                buff,
                refresh_duration,
                is_refresh,
                time,
            ) => FfxivEvent::ApplyBuffStack(
                player_id,
                target_id,
                buff.clone(),
                refresh_duration,
                is_refresh,
                elapsed_time + time,
            ),
            FfxivEvent::ApplyRaidBuff(player_id, buff, duration, max_refresh_duration, time) => {
                FfxivEvent::ApplyRaidBuff(
                    player_id,
                    buff.clone(),
                    duration,
                    max_refresh_duration,
                    elapsed_time + time,
                )
            }
            FfxivEvent::RefreshBuff(
                player_id,
                target_id,
                buff,
                duration,
                max_refresh_duration,
                time,
            ) => FfxivEvent::RefreshBuff(
                player_id,
                target_id,
                buff.clone(),
                duration,
                max_refresh_duration,
                elapsed_time + time,
            ),
            FfxivEvent::ApplyDebuffStack(player_id, debuff, refresh_duration, is_refresh, time) => {
                FfxivEvent::ApplyDebuffStack(
                    player_id,
                    debuff.clone(),
                    refresh_duration,
                    is_refresh,
                    elapsed_time + time,
                )
            }
            FfxivEvent::ApplyDebuff(player_id, debuff, duration, max_refresh_duration, time) => {
                FfxivEvent::ApplyDebuff(
                    player_id,
                    debuff.clone(),
                    duration,
                    max_refresh_duration,
                    elapsed_time + time,
                )
            }
            FfxivEvent::RemoveTargetBuff(player_id, target_id, buff_id, time) => {
                FfxivEvent::RemoveTargetBuff(player_id, target_id, buff_id, elapsed_time + time)
            }
            FfxivEvent::RemoveRaidBuff(player_id, buff_id, time) => {
                FfxivEvent::RemoveRaidBuff(player_id, buff_id, elapsed_time + time)
            }
            FfxivEvent::RemoveDebuff(player_id, debuff_id, time) => {
                FfxivEvent::RemoveDebuff(player_id, debuff_id, elapsed_time + time)
            }
            FfxivEvent::IncreasePlayerResource(player_id, stack_id, amount, time) => {
                FfxivEvent::IncreasePlayerResource(player_id, stack_id, amount, elapsed_time + time)
            }
            FfxivEvent::ReduceSkillCooldown(player_id, skill_id, reduce_amount, time) => {
                FfxivEvent::ReduceSkillCooldown(
                    player_id,
                    skill_id,
                    reduce_amount,
                    elapsed_time + time,
                )
            }
            FfxivEvent::PlayerTurn(player_id, turn_time, max_time, time) => {
                FfxivEvent::PlayerTurn(player_id, turn_time, max_time + time, elapsed_time + time)
            }
            FfxivEvent::DotTick(time) => FfxivEvent::DotTick(elapsed_time + time),
        }
    }

    pub fn to_turn_info(self) -> TurnInfo {
        match self {
            FfxivEvent::PlayerTurn(_, turn_type, max_time, time) => TurnInfo {
                turn_type,
                next_gcd_millisecond: max_time,
                lower_bound_millisecond: time,
            },
            _ => panic!("Cannot convert non-turn event to TurnInfo"),
        }
    }

    pub(crate) fn set_stacks(&mut self, stacks: ResourceType) {
        match self {
            FfxivEvent::ApplyBuff(_, _, buff, _, _, _) => {
                buff.stacks = min(stacks, buff.max_stacks);
            }
            FfxivEvent::ApplyBuffStack(_, _, buff, _, _, _) => {
                buff.stacks = min(stacks, buff.max_stacks);
            }
            FfxivEvent::ApplyRaidBuff(_, buff, _, _, _) => {
                buff.stacks = min(stacks, buff.max_stacks);
            }
            FfxivEvent::RefreshBuff(_, _, buff, _, _, _) => {
                buff.stacks = min(stacks, buff.max_stacks);
            }
            FfxivEvent::ApplyDebuff(_, debuff, _, _, _) => {
                debuff.stacks = min(stacks, debuff.max_stacks);
            }
            FfxivEvent::ApplyDebuffStack(_, debuff, _, _, _) => {
                debuff.stacks = min(stacks, debuff.max_stacks);
            }
            _ => {}
        }
    }
}

impl PartialEq<Self> for FfxivEvent {
    fn eq(&self, other: &Self) -> bool {
        self.get_event_time() == other.get_event_time()
    }
}

impl PartialOrd for FfxivEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.get_event_time().cmp(&other.get_event_time()))
    }
}
impl Ord for FfxivEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_event_time().cmp(&other.get_event_time())
    }
}

impl Eq for FfxivEvent {}
