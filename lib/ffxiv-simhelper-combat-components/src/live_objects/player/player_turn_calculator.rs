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
///
/// # FFXIV Player's Turn System
/// In FFXIV, a player's turn is divided into two types: GCD(Global CoolDown) turn and oGCD(off-GCD) turn.
///
/// ## 1. GCD Turn
/// - GCD turn when the player's GCD cooldown is back. The player can use one of the skills with a GCD cooldown.
/// ex) PLD's Fast Blade, DRG's True Thrust, WHM's glare IV
///
/// ## 2. oGCD Turn
/// Between the player's GCD turns, which are typically 2.5s long, the player can fit in multiple oGCD skills.
///
/// 1) Each FFXIV skill has a delay, and almost all skills' delay is 0.7s.
/// 2) After 0.7s delay of the GCD skill, the player can use an oGCD skill, a skill that has its own cooldown instead of the Global CoolDown.
/// 3) Even after the first oGCD skill's delay, it is **still 1.4s after the last GCD skill. Using another oGCD skill will create a delay until 2.1s**
///     * since most GCD skills are 2.5s long, player's GCD doesn't get bothered even if the player uses another oGCD skill.
///
/// thus, **in normal occasions, a player can "double-weave" two oGCD skills in between a GCD.**
/// the tricky part is, **this isn't always true**; for skills that have casts and fast GCD cooldown(ex) ninja's mudra skills), only one oGCD skill can be used between GCDs.
/// also for long GCD skills suck as Pictomancer's painting skills, 3 oGCD skills can be used between GCDs.
/// So it is a very tricky turn system to simulate.
///
/// # Implementation
/// **FFXIV Simhelper only allows 2 oGCD MAX between GCDs**
///    * 3 oGCD cases are very rare and exists only for new jobs like PCT and VPR
///    * However restricting these cases to 2 oGCDs doesn't hinder the job's rotation much, since these jobs aren't oGCD heavy enough to be weaving oGCDs in every possible slots.
#[derive(Clone)]
pub struct PlayerTurnCalculator {
    pub player_id: PlayerIdType,

    /// Keeps track of the last time the player's GCD was casted.
    /// last GCD cast time + the last cast GCD skill's GCD cooldown = next GCD start time.
    pub last_gcd_time_millisecond: TimeType,

    /// Keeps track of the player's last turn type (GCD turn or oGCD turn).
    latest_turn_type: FfxivTurnType,

    /// GCD cooldown can change dynamically based on skill's GCD cooldown(some skills are not 2.5s), speed buffs(like ley line).
    /// Thus we need to snapshot player's GCD cooldown status at the time the last GCD was casted to figure out when the next GCD will be.
    last_gcd_skill_time_info: SkillTimeInfo,

    /// The main event queue of the simulation board. PlayerTurnCalculator calculates the player's next turn and inserts it into the event queue directly.
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
        if let Some((ogcd_turn, gcd_turn)) = next_turns {
            self.ffxiv_event_queue.borrow_mut().push(Reverse(gcd_turn));
            self.ffxiv_event_queue.borrow_mut().push(Reverse(ogcd_turn));
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

                // At GCD turn you can already figure out
                // 1) When the next turn GCD will be
                // 2) When the first oGCD can be used
                //
                // Thus, you can produce two events at once.
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

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        event::{ffxiv_event::FfxivEvent, ffxiv_player_internal_event::FfxivPlayerInternalEvent},
        live_objects::turn_type::FfxivTurnType,
    };

    use super::PlayerTurnCalculator;

    #[test]
    fn get_next_turn_test() {
        // When: skill used at 10s: 1500ms charge time, 1000ms cast time, 1500ms gcd delay
        let charge_time_millisecond = 1500;
        let gcd_cooldown_millisecond = 3000;
        let cast_time_millisecond = 1000;
        let delay_millisecond = 670;
        let start_time_millisecond = 10000;

        let last_gcd_turn = FfxivPlayerInternalEvent::UpdateTurn(
            FfxivTurnType::Gcd,
            start_time_millisecond,
            charge_time_millisecond,
            cast_time_millisecond,
            gcd_cooldown_millisecond,
            delay_millisecond,
        );

        let mut player_turn_calculator =
            PlayerTurnCalculator::new(0, Rc::new(RefCell::new(Default::default())), None);

        player_turn_calculator.update_internal_status(&last_gcd_turn);
        let next_turn = player_turn_calculator.get_next_turn();

        assert!(next_turn.is_some());
        let (next_turn_ogcd, next_turn_gcd) = next_turn.unwrap();

        match next_turn_gcd {
            // cast time < gcd cooldown, so the end_time is charge_time(1500) + gcd_cooldown(1500)
            FfxivEvent::PlayerTurn(_, turn_type, end_time, start_time) => {
                assert!(matches!(turn_type, FfxivTurnType::Gcd));
                assert_eq!(start_time, 13000);
                assert_eq!(end_time, 13000);
            }
            _ => panic!("Expected GCD turn event"),
        }

        match next_turn_ogcd {
            FfxivEvent::PlayerTurn(_, turn_type, end_time, start_time) => {
                assert!(matches!(turn_type, FfxivTurnType::Ogcd));

                // cast time is longer than delay, so the oGCD start time offset is charge_time(1500) + delay(670)
                assert_eq!(
                    start_time,
                    start_time_millisecond + charge_time_millisecond + cast_time_millisecond
                );
                assert_eq!(end_time, start_time_millisecond + gcd_cooldown_millisecond);
            }
            _ => panic!("Expected oGCD turn event"),
        }
    }
}
