use crate::event::ffxiv_event::FfxivEvent;
use crate::event::FfxivEventQueue;
use crate::live_objects::player::StatusKey;
use crate::live_objects::target::Target;
use crate::owner_tracker::OwnerTracker;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_holder::StatusHolder;
use crate::status::status_timer::StatusTimer;
use crate::status::Status;
use crate::types::TimeType;
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
                    .remove(&StatusKey::new(debuff_id, player_id));
            }
            FfxivEvent::DotTick(combat_time_millisecond) => {
                for debuff in self.debuff_list.borrow().values() {
                    if let Some(potency) = debuff.potency {
                        let snapshotted_infos = debuff.snapshotted_infos.clone();

                        self.event_queue
                            .borrow_mut()
                            .push(Reverse(FfxivEvent::Damage(
                                debuff.get_owner_id(),
                                debuff.get_damage_skill_id().unwrap(),
                                potency,
                                debuff.trait_percent.unwrap(),
                                false,
                                false,
                                snapshotted_infos,
                                debuff.damage_category.unwrap(),
                                false,
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

impl Default for FfxivTarget {
    fn default() -> Self {
        FfxivTarget {
            debuff_list: Rc::new(RefCell::new(HashMap::new())),
            event_queue: Rc::new(RefCell::new(FfxivEventQueue::new())),
            combat_time_millisecond: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FfxivTarget;
    use crate::{
        live_objects::player::StatusKey,
        status::{debuff_status::DebuffStatus, status_timer::StatusTimer},
    };

    #[test]
    fn target_debuff_update_status_time_test() {
        let mut target = FfxivTarget::default();
        let mut debuff = DebuffStatus::default();

        debuff.duration_left_millisecond = 5000;
        debuff.duration_millisecond = 5000;

        let key = StatusKey {
            player_id: debuff.owner_id,
            status_id: debuff.id,
        };

        target.debuff_list.borrow_mut().insert(key, debuff);

        target.update_status_time(1000);

        let debuff_time = target
            .debuff_list
            .borrow()
            .get(&key)
            .unwrap()
            .duration_left_millisecond;

        assert_eq!(
            debuff_time, 4000,
            "Debuff time has to be 4000, but it is {}",
            debuff_time
        );

        target.update_status_time(3999);

        let debuff_check2 = target
            .debuff_list
            .borrow()
            .get(&key)
            .unwrap()
            .duration_left_millisecond;

        assert_eq!(
            debuff_check2, 1,
            "Debuff has to have 1 second remaining, but it is {}",
            debuff_check2
        );

        target.update_status_time(1);

        let debuff_is_none = target.debuff_list.borrow().get(&key).is_none();

        assert!(debuff_is_none, "Debuff has to be removed but it is not",)
    }
}
