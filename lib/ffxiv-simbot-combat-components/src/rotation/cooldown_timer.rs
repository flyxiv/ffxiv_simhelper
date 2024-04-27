use crate::TimeType;

pub trait CooldownTimer {
    fn update_cooldown(&mut self, elapsed_time: TimeType);
}
