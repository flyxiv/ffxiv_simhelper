use ffxiv_simbot_db::MultiplierType;
use crate::live_objects::player::Player;
use crate::skill::{GCD_TURN_DELAY_PERCENTAGE_THRESHOLD, NON_GCD_DELAY_MILLISECOND};
use crate::skill::skill::NON_GCD_DELAY_MILLISECOND;
use crate::TimeType;

pub(crate) trait TurnType {
    fn get_next_turn<P: Player>(&self, player: &P, current_combat_time: TimeType) -> PlayerTurn;
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

#[inline]
fn get_gcd_turn_delay_threshold(
    next_gcd_delay_millisecond: TimeType,
) -> TimeType {
    ((next_gcd_delay_millisecond as MultiplierType) * GCD_TURN_DELAY_PERCENTAGE_THRESHOLD) as TimeType
}

impl FfxivTurnType {
    fn get_next_turn<P>(&self, player: &P, current_combat_time_millisecond: TimeType) -> PlayerTurn
    where
        P: Player + Sized,
    {
        match self {
            FfxivTurnType::Gcd => PlayerTurn {
                turn_type: FfxivTurnType::Ogcd1,
                next_turn_combat_time_millisecond: current_combat_time_millisecond
                    + NON_GCD_DELAY_MILLISECOND,
            },
            FfxivTurnType::Ogcd1 => {
                if player.get_delay()
                    >= get_gcd_turn_delay_threshold(player.get)
                    )
                {
                    PlayerTurn {
                        turn_type: FfxivTurnType::Gcd,
                        next_turn_combat_time_millisecond: player.get_next_gcd_time_millisecond(),
                    }
                } else {
                    PlayerTurn {
                        turn_type: FfxivTurnType::Ogcd2,
                        next_turn_combat_time_millisecond: current_combat_time_millisecond
                            + NON_GCD_DELAY_MILLISECOND,
                    }
                }
            }
            FfxivTurnType::Ogcd2 => PlayerTurn {
                turn_type: FfxivTurnType::Gcd,
                next_turn_combat_time_millisecond: player.get_next_gcd_time_millisecond(),
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
}

impl Default for PlayerTurn {
    fn default() -> Self {
        PlayerTurn {
            turn_type: FfxivTurnType::Gcd,
            next_turn_combat_time_millisecond: 0,
        }
    }
}

impl TurnType for PlayerTurn {
    fn get_next_turn<P>(&self, player: &P, current_combat_time_millisecond: TimeType) -> PlayerTurn
    where
        P: Player + Sized,
    {
        self.turn_type
            .get_next_turn(player, current_combat_time_millisecond)
    }
}
