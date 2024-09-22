#[derive(Clone, Copy, Eq, PartialEq)]
pub enum DamageCategory {
    Direct,
    MagicalDot,
    PhysicalDot,
    AutoAttack,
}
