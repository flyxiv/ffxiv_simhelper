use crate::live_objects::player::Player;
use crate::skill::NON_GCD_DELAY_MILLISECOND;
use crate::TimeType;
use std::fmt::Display;

static INFINITE_DELAY: TimeType = 5000;

pub(crate) trait TurnType {
    fn get_next_turn<P: Player>(
        &self,
        player: &P,
        skill_delay: TimeType,
        charging_time_millisecond: TimeType,
        current_combat_time: TimeType,
    ) -> PlayerTurn;
}

/// Represents what type of skill the player can use the next turn.
/// GCD: Global Cooldown Skill
/// oGCD1: First oGCD Skill after a GCD skill
/// oGCD2: Second oGCD Skill after a GCD
#[derive(Clone)]
pub enum FfxivTurnType {
    Gcd,
    Ogcd1,
    Ogcd2,
}

impl FfxivTurnType {
    fn get_next_turn<P>(
        &self,
        player: &P,
        skill_delay: TimeType,
        charging_time_millisecond: TimeType,
        current_combat_time_millisecond: TimeType,
    ) -> PlayerTurn
    where
        P: Player + Sized,
    {
        let current_delay = player.get_delay();
        let next_delay = skill_delay + charging_time_millisecond;
        let last_gcd_time = player.get_last_gcd_time_millisecond();
        let next_gcd_time = player.get_next_gcd_time_millisecond();

        let total_delay = last_gcd_time + current_delay + next_delay;
        let delay_left = next_gcd_time - total_delay;

        match self {
            FfxivTurnType::Gcd => PlayerTurn {
                turn_type: FfxivTurnType::Ogcd1,
                next_turn_combat_time_millisecond: current_combat_time_millisecond + next_delay,
                delay_left,
            },
            FfxivTurnType::Ogcd1 => {
                if delay_left < NON_GCD_DELAY_MILLISECOND {
                    PlayerTurn {
                        turn_type: FfxivTurnType::Gcd,
                        next_turn_combat_time_millisecond: player.get_next_gcd_time_millisecond(),
                        delay_left: INFINITE_DELAY,
                    }
                } else {
                    {
                        PlayerTurn {
                            turn_type: FfxivTurnType::Ogcd2,
                            next_turn_combat_time_millisecond: current_combat_time_millisecond
                                + next_delay,
                            delay_left,
                        }
                    }
                }
            }
            FfxivTurnType::Ogcd2 => PlayerTurn {
                turn_type: FfxivTurnType::Gcd,
                next_turn_combat_time_millisecond: player.get_next_gcd_time_millisecond(),
                delay_left: INFINITE_DELAY,
            },
        }
    }
}

/// https://github.com/flyxiv/ffxiv_simbot/issues/6#issuecomment-2057989693
/// Ffxiv players have a 0.7 second delay after using an oGCD skill, and GCD seconds delay for each
/// Consecutive GCD skill, so FFXIV's combat can be seen as players taking turns, with the player
/// getting the earliest turn going first.
pub struct PlayerTurn {
    pub turn_type: FfxivTurnType,
    pub next_turn_combat_time_millisecond: TimeType,
    pub delay_left: TimeType,
}

impl Display for PlayerTurn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.turn_type {
            FfxivTurnType::Gcd => write!(f, "GCD, {}", self.next_turn_combat_time_millisecond),
            FfxivTurnType::Ogcd1 => write!(f, "oGCD1, {}", self.next_turn_combat_time_millisecond),
            FfxivTurnType::Ogcd2 => write!(f, "oGCD2, {}", self.next_turn_combat_time_millisecond),
        }
    }
}

impl Default for PlayerTurn {
    fn default() -> Self {
        PlayerTurn {
            turn_type: FfxivTurnType::Gcd,
            next_turn_combat_time_millisecond: 0,
            delay_left: INFINITE_DELAY,
        }
    }
}

impl TurnType for PlayerTurn {
    fn get_next_turn<P>(
        &self,
        player: &P,
        skill_delay: TimeType,
        charging_time_millisecond: TimeType,
        current_combat_time_millisecond: TimeType,
    ) -> PlayerTurn
    where
        P: Player + Sized,
    {
        self.turn_type.get_next_turn(
            player,
            skill_delay,
            charging_time_millisecond,
            current_combat_time_millisecond,
        )
    }
}

impl PlayerTurn {
    pub fn new(start_time_millisecond: TimeType) -> Self {
        Self {
            turn_type: FfxivTurnType::Gcd,
            next_turn_combat_time_millisecond: start_time_millisecond,
            delay_left: INFINITE_DELAY,
        }
    }
}
