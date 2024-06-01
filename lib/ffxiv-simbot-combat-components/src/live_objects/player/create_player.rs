use crate::event::ffxiv_event::FfxivEvent;
use crate::event::FfxivEventQueue;
use crate::id_entity::IdEntity;
use crate::jobs_skill_data::bard::priorities::BardPriorityTable;
use crate::jobs_skill_data::dancer::priorities::DancerPriorityTable;
use crate::jobs_skill_data::ninja::abilities::get_huton_status;
use crate::jobs_skill_data::ninja::priorities::NinjaPriorityTable;
use crate::jobs_skill_data::sage::priorities::SagePriorityTable;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::rotation::ffxiv_priority_table::FfxivPriorityTable;
use crate::{IdType, TimeType};
use ffxiv_simbot_db::ffxiv_context::FfxivContext;
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) static NINJA_START_TIME_MILLISECOND: TimeType = -2500;
pub(crate) static SAGE_START_TIME_MILLISECOND: TimeType = -1500;
pub(crate) static BARD_START_TIME_MILLISECOND: TimeType = 0;
pub(crate) static DANCER_START_TIME_MILLISECOND: TimeType = -4000;

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
            FfxivPriorityTable::Ninja(NinjaPriorityTable::new(player_id)),
            HashMap::from([(
                StatusKey::new(huton.get_id(), player_id),
                get_huton_status(player_id),
            )]),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                NINJA_START_TIME_MILLISECOND,
                NINJA_START_TIME_MILLISECOND,
            ),
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
            FfxivPriorityTable::Sage(SagePriorityTable::new(player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                SAGE_START_TIME_MILLISECOND,
                SAGE_START_TIME_MILLISECOND,
            ),
        )
    }

    pub fn new_bard(
        player_id: IdType,
        power: CharacterPower,
        context: &FfxivContext,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> FfxivPlayer {
        let bard_job = context.jobs.get("BRD").unwrap();

        Self::new(
            player_id,
            bard_job.clone(),
            power,
            FfxivPriorityTable::Bard(BardPriorityTable::new(player_id, ffxiv_event_queue.clone())),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                BARD_START_TIME_MILLISECOND,
                BARD_START_TIME_MILLISECOND,
            ),
        )
    }

    pub fn new_dancer(
        player_id: IdType,
        partner_player_id: IdType,
        power: CharacterPower,
        context: &FfxivContext,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> FfxivPlayer {
        let dancer_job = context.jobs.get("DNC").unwrap();

        Self::new(
            player_id,
            dancer_job.clone(),
            power,
            FfxivPriorityTable::Dancer(DancerPriorityTable::new(player_id, partner_player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                DANCER_START_TIME_MILLISECOND,
                DANCER_START_TIME_MILLISECOND,
            ),
        )
    }
}
