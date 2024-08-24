use crate::event::ffxiv_event::FfxivEvent;
use crate::event::ffxiv_player_internal_event::FfxivPlayerInternalEvent;
use crate::event::FfxivEventQueue;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::types::{PlayerIdType, TimeType};
use std::cell::RefCell;
use std::cmp::{max, Reverse};
use std::rc::Rc;

pub static COMBAT_START_TIME: TimeType = -10000;

/// Stores information needed to calculate the next turn of a player.
#[derive(Clone)]
pub struct PlayerTurnCalculator {
    /// How many seconds passed after the most recent GCD. If delay is close to GCD, an oGCD will
    /// clip the player's next GCD, so it becomes a GCD turn.
    pub player_id: PlayerIdType,
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
        let next_turns = self.get_next_turn();
        if let Some((ogcdTurn, gcdTurn)) = next_turns {
            self.ffxiv_event_queue.borrow_mut().push(Reverse(gcdTurn));
            self.ffxiv_event_queue.borrow_mut().push(Reverse(ogcdTurn));
        }
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

    pub fn get_next_turn(&self) -> Option<(FfxivEvent, FfxivEvent)> {
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
                // 이미 이번 글쿨을 썼으면 다음 글쿨이 언제인지를 알 수 있으므로 미리 다음 GCD도 등록해놓는다.
                // 그래야 A 랑 B oGCD를 같은 시각에 써도 시간이 뒤엉키지 않는다.
                Some((
                    FfxivEvent::PlayerTurn(
                        self.player_id,
                        FfxivTurnType::Ogcd,
                        next_gcd_time_millisecond,
                        first_ogcd_start_time,
                    ),
                    FfxivEvent::PlayerTurn(
                        self.player_id,
                        FfxivTurnType::Gcd,
                        next_gcd_time_millisecond,
                        next_gcd_time_millisecond,
                    ),
                ))
            }
            FfxivTurnType::Ogcd => None,
        }
    }

    pub(crate) fn new(
        player_id: PlayerIdType,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        gcd_start_time_millisecond: Option<TimeType>,
    ) -> Self {
        let gcd_start_time_millisecond =
            if let Some(gcd_start_time_millisecond) = gcd_start_time_millisecond {
                gcd_start_time_millisecond
            } else {
                COMBAT_START_TIME
            };

        PlayerTurnCalculator {
            player_id,
            total_delay_millisecond: 0,
            last_gcd_time_millisecond: gcd_start_time_millisecond,
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
