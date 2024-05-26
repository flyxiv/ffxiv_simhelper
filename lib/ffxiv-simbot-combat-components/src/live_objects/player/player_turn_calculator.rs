use crate::event::ffxiv_event::FfxivEvent;
use crate::event::ffxiv_player_internal_event::FfxivPlayerInternalEvent;
use crate::event::FfxivEventQueue;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::{IdType, TimeType, COMBAT_START_TIME};
use std::cell::RefCell;
use std::cmp::{max, Reverse};
use std::rc::Rc;

/// Stores information needed to calculate the next turn of a player.
#[derive(Clone)]
pub struct PlayerTurnCalculator {
    /// How many seconds passed after the most recent GCD. If delay is close to GCD, an oGCD will
    /// clip the player's next GCD, so it becomes a GCD turn.
    pub player_id: IdType,
    pub total_delay_millisecond: TimeType,
    pub last_gcd_time_millisecond: TimeType,

    latest_turn_type: FfxivTurnType,
    last_gcd_skill_time_info: SkillTimeInfo,
    ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
}

#[derive(Clone)]
pub(crate) struct SkillTimeInfo {
    pub(crate) charge_time_millisecond: TimeType,
    pub(crate) cast_time_millisecond: TimeType,
    pub(crate) gcd_cooldown_millisecond: TimeType,
    pub(crate) delay_millisecond: TimeType,
}

impl PlayerTurnCalculator {
    pub(crate) fn produce_event_to_queue(&self) {
        let next_turn = self.get_next_turn();
        self.ffxiv_event_queue.borrow_mut().push(Reverse(next_turn));
    }

    pub(crate) fn update_internal_status(&mut self, event: &FfxivPlayerInternalEvent) {
        match event {
            FfxivPlayerInternalEvent::UpdateTurn(
                turn_type,
                combat_time_millisecond,
                charge_time_millisecond,
                cast_time_millisecond,
                gcd_cooldown_millisecond,
                delay_millisecond,
            ) => {
                if matches!(turn_type, FfxivTurnType::Gcd) {
                    self.last_gcd_time_millisecond = *combat_time_millisecond;
                    self.last_gcd_skill_time_info = SkillTimeInfo {
                        charge_time_millisecond: *charge_time_millisecond,
                        cast_time_millisecond: *cast_time_millisecond,
                        gcd_cooldown_millisecond: *gcd_cooldown_millisecond,
                        delay_millisecond: *delay_millisecond,
                    };
                }

                self.latest_turn_type = turn_type.clone();
            }
            _ => {}
        }
    }

    pub fn get_next_turn(&self) -> FfxivEvent {
        let gcd_cooldown = self.last_gcd_skill_time_info.gcd_cooldown_millisecond;
        let next_gcd_time_millisecond = self.last_gcd_time_millisecond + gcd_cooldown;

        match self.latest_turn_type {
            FfxivTurnType::Gcd => {
                let delay = max(
                    self.last_gcd_skill_time_info.delay_millisecond,
                    self.last_gcd_skill_time_info.cast_time_millisecond,
                );
                let first_ogcd_start_time = self.last_gcd_time_millisecond
                    + self.last_gcd_skill_time_info.charge_time_millisecond
                    + delay;

                // oGCD turn: 시작/끝 시간 안에 가장 잘 맞는 두 oGCD쌍을 한번에 찾아서 등록(둘 중 highest priority로 랭킹).
                FfxivEvent::PlayerTurn(
                    self.player_id,
                    FfxivTurnType::Ogcd,
                    next_gcd_time_millisecond,
                    first_ogcd_start_time,
                )
            }
            FfxivTurnType::Ogcd => FfxivEvent::PlayerTurn(
                self.player_id,
                FfxivTurnType::Gcd,
                next_gcd_time_millisecond,
                next_gcd_time_millisecond,
            ),
        }
    }

    pub(crate) fn new(player_id: IdType, ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>) -> Self {
        PlayerTurnCalculator {
            player_id,
            total_delay_millisecond: 0,
            last_gcd_time_millisecond: COMBAT_START_TIME,
            latest_turn_type: FfxivTurnType::Gcd,
            last_gcd_skill_time_info: SkillTimeInfo {
                charge_time_millisecond: 0,
                cast_time_millisecond: 0,
                gcd_cooldown_millisecond: 0,
                delay_millisecond: 0,
            },
            ffxiv_event_queue,
        }
    }
}
