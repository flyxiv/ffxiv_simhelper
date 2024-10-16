use crate::event::ffxiv_event::FfxivEvent;
use crate::event_ticker::PercentType;
use crate::id_entity::IdEntity;
use crate::owner_tracker::OwnerTracker;
use crate::status::status_info::StatusInfo;
use crate::status::Status;
use crate::types::{BuffIncreasePercentType, PlayerIdType};
use crate::types::{SkillIdType, SkillStackType, TimeType};
use rand::{thread_rng, Rng};
use std::cmp::min;

#[derive(PartialEq, Eq, Clone)]
pub struct BuffStatus {
    pub(crate) id: SkillIdType,
    pub(crate) owner_id: PlayerIdType,
    pub(crate) duration_left_millisecond: TimeType,
    pub(crate) status_info: Vec<StatusInfo>,
    pub(crate) duration_millisecond: TimeType,
    pub is_raidwide: bool,
    pub(crate) name: String,
    pub(crate) stacks: SkillStackType,
    pub(crate) max_stacks: SkillStackType,
    pub(crate) trigger_proc_event_on_gcd: Vec<(FfxivEvent, PercentType)>,
}

impl Status for BuffStatus {
    fn get_duration_left_millisecond(&self) -> TimeType {
        self.duration_left_millisecond
    }
    fn set_duration_left_millisecond(&mut self, duration: TimeType) {
        self.duration_left_millisecond = duration;
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_status_info(&self) -> &[StatusInfo] {
        &self.status_info
    }

    fn get_duration_millisecond(&self) -> i32 {
        self.duration_millisecond
    }

    fn is_raidwide(&self) -> bool {
        self.is_raidwide
    }
    fn add_stack(&mut self, stack: SkillStackType) {
        self.stacks = min(self.stacks + stack, self.max_stacks);
    }

    fn get_stack(&self) -> SkillStackType {
        self.stacks
    }

    fn get_damage_skill_id(&self) -> Option<SkillIdType> {
        None
    }
}

impl BuffStatus {
    pub(crate) fn generate_proc_event(
        &self,
        current_time_millisecond: TimeType,
    ) -> Vec<FfxivEvent> {
        let mut proc_events = vec![];
        proc_events.reserve(self.trigger_proc_event_on_gcd.len());

        for (proc_event, proc_percent) in self.trigger_proc_event_on_gcd.iter() {
            let proc_value = thread_rng().gen_range(0..100);
            if proc_value <= *proc_percent {
                let proc_event = proc_event.clone();
                proc_events.push(proc_event.add_time_to_event(current_time_millisecond));
            }
        }

        proc_events
    }

    pub fn get_damage_buff_infos(&self) -> Vec<StatusInfo> {
        self.status_info
            .clone()
            .into_iter()
            .filter_map(|status_info| match status_info {
                StatusInfo::DirectHitRatePercent(_)
                | StatusInfo::CritHitRatePercent(_)
                | StatusInfo::IncreaseMainStat(_, _) => Some(status_info),
                StatusInfo::DamagePercent(increase) => Some(StatusInfo::DamagePercent(
                    increase * self.get_stack() as BuffIncreasePercentType,
                )),
                _ => None,
            })
            .collect()
    }
}

impl IdEntity for BuffStatus {
    fn get_id(&self) -> SkillIdType {
        self.id
    }
}

impl OwnerTracker for BuffStatus {
    fn get_owner_id(&self) -> PlayerIdType {
        self.owner_id
    }
}

impl Default for BuffStatus {
    fn default() -> Self {
        BuffStatus {
            id: 0,
            stacks: 0,
            duration_millisecond: 0,
            duration_left_millisecond: 0,
            status_info: vec![],
            name: "".to_string(),
            max_stacks: 0,
            trigger_proc_event_on_gcd: vec![],
            owner_id: 0,
            is_raidwide: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::status::status_info::StatusInfo;

    use super::BuffStatus;

    #[test]
    fn generate_proc_event_test() {
        let mut buff_status = BuffStatus::default();

        let proc_chance = 0.50;

        buff_status.trigger_proc_event_on_gcd.push((
            FfxivEvent::Damage(0, 0, 0, 0, false, false, vec![], None, false, 0),
            proc_chance * 100 as PercentType,
        ));

        let iteration_count = 1000;

        let mut events = vec![];

        for _ in 0..iteration_count {
            let proc_events = buff_status.generate_proc_event(i);

            events.extend(proc_events);
        }

        let proc_lower_bound = (proc_chance - 0.05) * iteration_count as f64;
        let proc_upper_bound = (proc_chance + 0.05) * iteration_count as f64;

        assert!(
            events.len() >= proc_lower_bound as usize && events.len() <= proc_upper_bound as usize,
            "Proc event count is not in the range of {} to {}: it is {}",
            proc_lower_bound,
            proc_upper_bound,
            events.len()
        );
    }

    #[test]
    fn get_damage_buff_infos_test() {
        let mut buff_status = BuffStatus::default();

        buff_status.status_info = vec![
            StatusInfo::DamagePercent(10),
            StatusInfo::CritHitRatePercent(10),
            StatusInfo::DirectHitRatePercent(10),
            StatusInfo::IncreaseMainStat(10, 800),
            StatusInfo::SpeedByStack(vec![1, 2, 4, 10]),
            StatusInfo::SpeedOnlyAutoAttack(10),
            StatusInfo::SpeedPercent(10),
        ];

        let damage_buffs = buff_status.get_damage_buff_infos();
        let answer = vec![
            StatusInfo::DamagePercent(10),
            StatusInfo::CritHitRatePercent(10),
            StatusInfo::DirectHitRatePercent(10),
            StatusInfo::IncreaseMainStat(10, 800),
        ];

        assert_eq!(damage_buffs, answer, "{}", damage_buffs.len());
    }
}
