/// Represents what type of skill the player can use the next turn.
/// GCD: Global Cooldown Skill
/// oGCD: oGCD Skills
#[derive(Clone, Copy)]
pub enum FfxivTurnType {
    Gcd,
    Ogcd,
}
