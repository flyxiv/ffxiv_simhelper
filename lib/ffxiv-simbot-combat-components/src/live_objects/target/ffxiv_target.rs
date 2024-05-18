use crate::event::ffxiv_event::FfxivEvent;
use crate::event::FfxivEventQueue;
use crate::id_entity::IdEntity;
use crate::live_objects::player::StatusKey;
use crate::live_objects::target::Target;
use crate::owner_tracker::OwnerTracker;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_holder::StatusHolder;
use crate::status::status_timer::StatusTimer;
use crate::TimeType;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::rc::Rc;

/// Stores the debuff list of the target
/// debuff list will be sorted in the order of debuff time left so that
/// it is easy to search which debuffs will be removed.
pub struct FfxivTarget {
    pub debuff_list: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
    pub event_queue: Rc<RefCell<FfxivEventQueue>>,
    pub combat_time_millisecond: TimeType,
}

impl Target for FfxivTarget {
    fn handle_ffxiv_event(&mut self, event: FfxivEvent) {
        match event {
            FfxivEvent::ApplyDebuff(
                player_id,
                debuff,
                duration_millisecond,
                max_duration_millisecond,
                _,
            ) => {
                self.add_status(
                    debuff,
                    duration_millisecond,
                    max_duration_millisecond,
                    player_id,
                );
            }
            FfxivEvent::RemoveDebuff(player_id, debuff_id, _) => {
                self.debuff_list
                    .borrow_mut()
                    .remove(&StatusKey::new(player_id, debuff_id));
            }
            FfxivEvent::DotTick(combat_time_millisecond) => {
                for debuff in self.debuff_list.borrow().values() {
                    if let Some(potency) = debuff.potency {
                        let snapshotted_buffs = debuff.snapshotted_buffs.clone().unwrap();
                        let snapshotted_debuffs = debuff.snapshotted_debuffs.clone().unwrap();

                        self.event_queue
                            .borrow_mut()
                            .push(Reverse(FfxivEvent::Damage(
                                debuff.get_owner_id(),
                                debuff.get_id(),
                                potency,
                                false,
                                false,
                                snapshotted_buffs,
                                snapshotted_debuffs,
                                combat_time_millisecond,
                            )));
                    }
                }
            }
            _ => {}
        }
    }
}

impl StatusHolder<DebuffStatus> for FfxivTarget {
    fn get_status_table(&self) -> Rc<RefCell<HashMap<StatusKey, DebuffStatus>>> {
        self.debuff_list.clone()
    }
}

impl StatusTimer<DebuffStatus> for FfxivTarget {}
