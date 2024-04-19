use crate::player::{Player, GCD_TURN_DELAY_THRESHOLD};
use crate::skill::NON_GCD_DELAY_MILLISECOND;
use crate::TimeType;

pub(crate) trait TurnType {
    fn get_next_turn<P: Player>(&self, player: &P) -> PlayerTurn;
}

/// Represents what type of skill the player can use the next turn.
/// GCD: Global Cooldown Skill
/// oGCD1: First oGCD Skill after a GCD skill
/// oGCD2: Second oGCD Skill after a GCD
pub(crate) enum FfxivTurnType {
    Gcd,
    Ogcd1,
    Ogcd2,
}

impl PartialEq for FfxivTurnType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TurnType::Gcd, TurnType::Gcd) => true,
            (TurnType::Ogcd1, TurnType::Ogcd2) => true,
            _ => false,
        }
    }
}

impl Eq for FfxivTurnType {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl FfxivTurnType {
    fn get_next_turn<P>(&self, player: &P) -> PlayerTurn
    where
        P: Player + Sized,
    {
        let current_turn_combat_time = player.get_combat_time_millisecond();

        match self {
            FfxivTurnType::Gcd => PlayerTurn {
                turn_type: FfxivTurnType::Ogcd1,
                next_turn_combat_time_millisecond: current_turn_combat_time
                    + NON_GCD_DELAY_MILLISECOND,
            },
            FfxivTurnType::Ogcd1 => {
                if player.get_delay() >= GCD_TURN_DELAY_THRESHOLD {
                    PlayerTurn {
                        turn_type: FfxivTurnType::Gcd,
                        next_turn_combat_time_millisecond: player.get_next_gcd_time_millisecond(),
                    }
                } else {
                    PlayerTurn {
                        turn_type: FfxivTurnType::Ogcd2,
                        next_turn_combat_time_millisecond: current_turn_combat_time
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
pub(crate) struct PlayerTurn {
    pub(crate) turn_type: FfxivTurnType,
    pub(crate) next_turn_combat_time_millisecond: TimeType,
}

impl Default for PlayerTurn {
    fn default() -> Self {
        PlayerTurn {
            turn_type: TurnType::Gcd,
            next_turn_combat_time_millisecond: 0,
        }
    }
}

impl PartialEq for PlayerTurn {
    fn eq(&self, other: &Self) -> bool {
        self.next_turn_combat_time_millisecond == other.next_turn_combat_time_millisecond
            && self.turn_type == other.turn_type
    }
}

impl Eq for PlayerTurn {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl PartialOrd for PlayerTurn {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.next_turn_combat_time_millisecond
            .partial_cmp(&other.next_turn_combat_time_millisecond)
    }
}

impl Ord for PlayerTurn {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.next_turn_combat_time_millisecond
            .cmp(&other.next_turn_combat_time_millisecond)
    }
}

impl TurnType for PlayerTurn {
    fn get_next_turn<P>(&self, player: &P) -> PlayerTurn
    where
        P: Player + Sized,
    {
        self.turn_type.get_next_turn(player)
    }
}
