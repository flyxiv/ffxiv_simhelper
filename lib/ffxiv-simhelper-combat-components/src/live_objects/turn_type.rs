/// Represents what type of skill the player can use the next turn.
/// GCD: Global Cooldown Skill
/// oGCD: oGCD Skill after a GCD skill
#[derive(Clone, Copy)]
pub enum FfxivTurnType {
    Gcd,
    Ogcd,
}
