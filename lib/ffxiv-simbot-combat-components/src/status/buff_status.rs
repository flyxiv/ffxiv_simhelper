use crate::event::ffxiv_event::FfxivEvent;
use crate::event_ticker::PercentType;
use crate::id_entity::IdEntity;
use crate::owner_tracker::OwnerTracker;
use crate::status::status_info::StatusInfo;
use crate::status::Status;
use crate::types::ResourceType;
use crate::{IdType, TimeType};
use rand::{thread_rng, Rng};
use std::cmp::min;

#[derive(PartialEq, Eq, Clone)]
pub struct BuffStatus {
    pub(crate) id: IdType,
    pub(crate) owner_id: IdType,
    pub(crate) duration_left_millisecond: TimeType,
    pub(crate) status_info: Vec<StatusInfo>,
    pub(crate) duration_millisecond: TimeType,
    pub is_raidwide: bool,
    pub(crate) name: String,
    pub(crate) stacks: ResourceType,
    pub(crate) max_stacks: ResourceType,
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

    fn get_status_info(&self) -> &Vec<StatusInfo> {
        &self.status_info
    }

    fn get_duration_millisecond(&self) -> i32 {
        self.duration_millisecond
    }

    fn is_raidwide(&self) -> bool {
        self.is_raidwide
    }
    fn add_stack(&mut self, stack: ResourceType) {
        self.stacks = min(self.stacks + stack, self.max_stacks);
    }

    fn get_stack(&self) -> ResourceType {
        self.stacks
    }

    fn get_damage_skill_id(&self) -> Option<IdType> {
        None
    }
}

impl BuffStatus {
    pub(crate) fn generate_proc_event(
        &self,
        current_time_millisecond: TimeType,
    ) -> Vec<FfxivEvent> {
        let proc_value = thread_rng().gen_range(0..100);
        let mut proc_events = vec![];

        for (proc_event, proc_percent) in self.trigger_proc_event_on_gcd.iter() {
            if proc_value <= *proc_percent {
                let proc_event = proc_event.clone();
                proc_events.push(proc_event.add_time_to_event(current_time_millisecond));
            }
        }

        proc_events
    }

    pub fn is_damage_buff(&self) -> bool {
        self.status_info
            .iter()
            .any(|status_info| match status_info {
                StatusInfo::DirectHitRatePercent(_)
                | StatusInfo::CritHitRatePercent(_)
                | StatusInfo::DamagePercent(_) => true,
                _ => false,
            })
    }
}

impl IdEntity for BuffStatus {
    fn get_id(&self) -> IdType {
        self.id
    }
}

impl OwnerTracker for BuffStatus {
    fn get_owner_id(&self) -> IdType {
        self.owner_id
    }
}
