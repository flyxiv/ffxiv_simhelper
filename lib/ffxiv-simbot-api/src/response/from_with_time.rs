use ffxiv_simbot_combat_components::types::TimeType;

pub(crate) trait FromWithTime<T>: Sized {
    fn from_with_time(value: T, time_millisecond: TimeType) -> Self;
}
