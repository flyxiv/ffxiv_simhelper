use crate::event::FfxivEventQueue;
use crate::id_entity::IdEntity;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::job_priorities::ffxiv_priority_table::FfxivPriorityTable;
use crate::rotation::job_priorities::ninja::NinjaPriorityTable;
use crate::rotation::job_priorities::sage::SagePriorityTable;
use crate::skill::job_abilities::ninja_abilities::get_huton_status;
use crate::{IdType, TimeType};
use ffxiv_simbot_db::ffxiv_context::FfxivContext;
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) static NINJA_START_TIME_MILLISECOND: TimeType = -2500;
pub(crate) static SAGE_START_TIME_MILLISECOND: TimeType = -1500;

impl FfxivPlayer {
    pub fn new_ninja(
        player_id: IdType,
        power: CharacterPower,
        context: &FfxivContext,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> FfxivPlayer {
        let ninja_job = context.jobs.get("NIN").unwrap();
        let huton = get_huton_status(player_id);

        Self::new(
            player_id,
            ninja_job.clone(),
            power,
            FfxivPriorityTable::Ninja(NinjaPriorityTable::new()),
            HashMap::from([(
                StatusKey::new(player_id, huton.get_id()),
                get_huton_status(player_id),
            )]),
            ffxiv_event_queue,
            NINJA_START_TIME_MILLISECOND,
        )
    }
    pub fn new_sage(
        player_id: IdType,
        power: CharacterPower,
        context: &FfxivContext,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> FfxivPlayer {
        let sage_job = context.jobs.get("SGE").unwrap();

        Self::new(
            player_id,
            sage_job.clone(),
            power,
            FfxivPriorityTable::Sage(SagePriorityTable::new()),
            Default::default(),
            ffxiv_event_queue,
            SAGE_START_TIME_MILLISECOND,
        )
    }
}
