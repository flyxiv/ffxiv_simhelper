use crate::types::TimeType;

/// Objects that contain states which have limited duration thus should update their time
/// should implement this trait
///
/// ex) Skills have cooldowns that need to be updated
/// buffs and debuffs have durations that need to be updated
pub trait CooldownTimer {
    fn update_cooldown(&mut self, elapsed_time: TimeType);
}
