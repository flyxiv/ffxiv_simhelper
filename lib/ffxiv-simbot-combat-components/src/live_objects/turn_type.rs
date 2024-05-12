use crate::live_objects::player::Player;
use crate::skill::NON_GCD_DELAY_MILLISECOND;
use crate::TimeType;
use std::fmt::Display;

static INFINITE_DELAY: TimeType = 5000;

/// Represents what type of skill the player can use the next turn.
/// GCD: Global Cooldown Skill
/// oGCD1: First oGCD Skill after a GCD skill
/// oGCD2: Second oGCD Skill after a GCD
#[derive(Clone)]
pub enum FfxivTurnType {
    Gcd,
    Ogcd,
}
