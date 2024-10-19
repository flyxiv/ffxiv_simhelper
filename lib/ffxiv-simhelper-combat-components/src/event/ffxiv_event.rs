use crate::event::turn_info::TurnInfo;
use crate::event_ticker::ffxiv_event_ticker::FfxivEventTicker;
use crate::event_ticker::{PercentType, TickerKey};
use crate::live_objects::player::StatusKey;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::skill::damage_category::DamageCategory;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::snapshot_status::snapshot_status_infos;
use crate::types::{
    PlayerIdType, ResourceIdType, SkillIdType, SkillStackType, SnapshotTable, TimeType,
};
use crate::types::{PotencyType, ResourceType};
use std::cmp::min;
use std::collections::HashMap;

/// All possible damage related events in a FFXIV combat.
/// the last TimeType element is always the time of the event.
///
/// The simulation is done like a CPU scheduler, where the events are scheduled in a priority queue in chronological order and executed when it is their turn.
/// Each event can generate new events and add them to the queue, which is repeated until the target combat time is reached.
#[derive(Clone)]
pub enum FfxivEvent {
    /// owner_player_id, turn, threshold limit time
    PlayerTurn(PlayerIdType, FfxivTurnType, TimeType, TimeType),
    /// player_id, target_id, skill_id
    UseSkill(PlayerIdType, Option<PlayerIdType>, SkillIdType, TimeType),

    /// owner_player_id, skill ID, potency, trait, guaranteed crit, guaranteed direct hit, snapshotted buffs and debuffs, damage category, is_gcd
    Damage(
        PlayerIdType,
        SkillIdType,
        PotencyType,
        PercentType,
        bool,
        bool,
        SnapshotTable,
        DamageCategory,
        bool,
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
    ApplyBuff(
        PlayerIdType,
        PlayerIdType,
        BuffStatus,
        TimeType,
        TimeType,
        TimeType,
    ),
    /// owner_player_id, target_id, status, status time, refresh duration or not
    ApplyBuffStack(
        PlayerIdType,
        PlayerIdType,
        BuffStatus,
        TimeType,
        bool,
        TimeType,
    ),
    /// owner_player_id, status, duration, max refresh duration
    ApplyRaidBuff(PlayerIdType, BuffStatus, TimeType, TimeType, TimeType),
    /// owner_player_id, target_id, status, duration, max refresh duration
    RefreshBuff(
        PlayerIdType,
        PlayerIdType,
        BuffStatus,
        TimeType,
        TimeType,
        TimeType,
    ),

    /// owner_player_id, time, status time, refresh duration or not
    ApplyDebuffStack(PlayerIdType, DebuffStatus, TimeType, bool, TimeType),
    /// owner_player_id, status, status time, max refresh duration
    ApplyDebuff(PlayerIdType, DebuffStatus, TimeType, TimeType, TimeType),

    /// owner_player_id, target_player_id, buff_id
    RemoveTargetBuff(PlayerIdType, PlayerIdType, SkillIdType, TimeType),
    /// owner_player_id, buff_id
    RemoveRaidBuff(PlayerIdType, SkillIdType, TimeType),
    /// owner_player_id, debuff_id
    RemoveDebuff(PlayerIdType, SkillIdType, TimeType),

    /// Raises stack of another player, for dance partners and brotherhood
    /// player_id, stack id, increase amount
    IncreasePlayerResource(PlayerIdType, ResourceIdType, ResourceType, TimeType),
    /// player_id, skill_id, reduce_amount
    ReduceSkillCooldown(PlayerIdType, SkillIdType, TimeType, TimeType),
    DotTick(TimeType),
}

impl FfxivEvent {
    pub fn get_event_time(&self) -> TimeType {
        match self {
            FfxivEvent::PlayerTurn(_, _, _, time)
            | FfxivEvent::UseSkill(_, _, _, time)
            | FfxivEvent::Damage(_, _, _, _, _, _, _, _, _, time)
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

    pub(crate) fn set_target(&mut self, target_id: PlayerIdType) {
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
        buffs: &HashMap<StatusKey, BuffStatus>,
        debuffs: &HashMap<StatusKey, DebuffStatus>,
    ) {
        match self {
            FfxivEvent::ApplyDebuff(player_id, status, _, _, _) => {
                if let Some(potency) = status.potency {
                    if potency > 0 {
                        status.snapshotted_infos =
                            snapshot_status_infos(buffs, debuffs, *player_id);
                    }
                }
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
                trait_percent,
                is_crit,
                is_direct_hit,
                snapshots,
                damage_category,
                is_gcd,
                time,
            ) => FfxivEvent::Damage(
                player_id,
                skill_id,
                potency,
                trait_percent,
                is_crit,
                is_direct_hit,
                snapshots,
                damage_category,
                is_gcd,
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
        let stacks = stacks as SkillStackType;
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

    /// 낮을수록 중요
    fn importance_of_event(&self) -> i32 {
        match self {
            FfxivEvent::UseSkill(_, _, _, _) => 1,
            FfxivEvent::Damage(_, _, _, _, _, _, _, _, _, _) => 2,
            FfxivEvent::Tick(_, _) => 3,
            FfxivEvent::AddTicker(_, _) => 4,
            FfxivEvent::RemoveTicker(_, _) => 5,
            FfxivEvent::ForceTicker(_, _) => 6,
            FfxivEvent::ApplyBuff(_, _, _, _, _, _) => 7,
            FfxivEvent::ApplyBuffStack(_, _, _, _, _, _) => 8,
            FfxivEvent::ApplyRaidBuff(_, _, _, _, _) => 9,
            FfxivEvent::RefreshBuff(_, _, _, _, _, _) => 10,
            FfxivEvent::ApplyDebuffStack(_, _, _, _, _) => 11,
            FfxivEvent::ApplyDebuff(_, _, _, _, _) => 12,
            FfxivEvent::RemoveTargetBuff(_, _, _, _) => 13,
            FfxivEvent::RemoveRaidBuff(_, _, _) => 14,
            FfxivEvent::RemoveDebuff(_, _, _) => 15,
            FfxivEvent::IncreasePlayerResource(_, _, _, _) => 16,
            FfxivEvent::ReduceSkillCooldown(_, _, _, _) => 17,
            FfxivEvent::DotTick(_) => 18,
            FfxivEvent::PlayerTurn(_, turn_type, _, _) => match turn_type {
                FfxivTurnType::Ogcd => 19,
                FfxivTurnType::Gcd => 20,
            },
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
        let first_cmp = self.get_event_time().cmp(&other.get_event_time());
        if first_cmp == std::cmp::Ordering::Equal {
            self.importance_of_event().cmp(&other.importance_of_event())
        } else {
            first_cmp
        }
    }
}

impl Eq for FfxivEvent {}

impl ToString for FfxivEvent {
    fn to_string(&self) -> String {
        match self {
            FfxivEvent::PlayerTurn(id, _, _, time) => {
                format!("Event: Player Turn {}, time: {}", id, time)
            }
            FfxivEvent::UseSkill(_, _, _, _) => String::from("Event: Use Skill"),
            FfxivEvent::Damage(_, _, _, _, _, _, _, _, _, _) => String::from("Event: Damage"),
            FfxivEvent::Tick(_, _) => String::from("Event: Tick"),
            FfxivEvent::AddTicker(_, _) => String::from("Event: Add Ticker"),
            FfxivEvent::RemoveTicker(_, _) => String::from("Event: Remove Ticker"),
            FfxivEvent::ForceTicker(_, _) => String::from("Event: Force Ticker"),
            FfxivEvent::ApplyBuff(_, _, _, _, _, _) => String::from("Event: Apply Buff"),
            FfxivEvent::ApplyBuffStack(_, _, _, _, _, _) => String::from("Event: Apply Buff Stack"),
            FfxivEvent::ApplyRaidBuff(_, _, _, _, _) => String::from("Event: Apply Raid Buff"),
            FfxivEvent::RefreshBuff(_, _, _, _, _, _) => String::from("Event: Refresh Buff"),
            FfxivEvent::ApplyDebuffStack(_, _, _, _, _) => {
                String::from("Event: Apply Debuff Stack")
            }
            FfxivEvent::ApplyDebuff(_, _, _, _, _) => String::from("Event: Apply Debuff"),
            FfxivEvent::RemoveTargetBuff(_, _, _, _) => String::from("Event: Remove Target Buff"),
            FfxivEvent::RemoveRaidBuff(_, _, _) => String::from("Event: Remove Raid Buff"),
            FfxivEvent::RemoveDebuff(_, _, _) => String::from("Event: Remove Debuff"),
            FfxivEvent::IncreasePlayerResource(_, _, _, _) => {
                String::from("Event: Increase Player Resource")
            }
            FfxivEvent::ReduceSkillCooldown(_, _, _, _) => {
                String::from("Event: Reduce Skill Cooldown")
            }
            FfxivEvent::DotTick(_) => String::from("Event: Dot Tick"),
        }
    }
}
