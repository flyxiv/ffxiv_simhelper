use crate::event::ffxiv_event::FfxivEvent;
use crate::event::FfxivEventQueue;
use crate::id_entity::IdEntity;
use crate::jobs_skill_data::bard::priorities::BardPriorityTable;
use crate::jobs_skill_data::black_mage::priorities::BlackmagePriorityTable;
use crate::jobs_skill_data::dancer::priorities::DancerPriorityTable;
use crate::jobs_skill_data::dragoon::priorities::DragoonPriorityTable;
use crate::jobs_skill_data::monk::priorities::MonkPriorityTable;
use crate::jobs_skill_data::ninja::abilities::get_huton_status;
use crate::jobs_skill_data::ninja::priorities::NinjaPriorityTable;
use crate::jobs_skill_data::sage::priorities::SagePriorityTable;
use crate::jobs_skill_data::white_mage::priorities::WhitemagePriorityTable;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::rotation::ffxiv_priority_table::FfxivPriorityTable;
use crate::status::buff_status::BuffStatus;
use crate::status::status_info::StatusInfo;
use crate::{IdType, TimeType};
use ffxiv_simbot_db::ffxiv_context::FfxivContext;
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) static WHITEMAGE_START_TIME_MILLISECOND: TimeType = -1500;
pub(crate) static BLACKMAGE_START_TIME_MILLISECOND: TimeType = -5000;
pub(crate) static NINJA_START_TIME_MILLISECOND: TimeType = -2500;
pub(crate) static SAGE_START_TIME_MILLISECOND: TimeType = -1500;
pub(crate) static BARD_START_TIME_MILLISECOND: TimeType = 0;
pub(crate) static DANCER_START_TIME_MILLISECOND: TimeType = -4000;
pub(crate) static MONK_START_TIME_MILLISECOND: TimeType = 0;

pub(crate) static DRAGOON_START_TIME_MILLISECOND: TimeType = 0;

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
            None,
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
            None,
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
            None,
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
            Some(partner_player_id),
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

    pub fn new_monk(
        player_id: IdType,
        power: CharacterPower,
        context: &FfxivContext,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> FfxivPlayer {
        let monk_job = context.jobs.get("MNK").unwrap();

        let GREASED_LIGHTNING_IV: BuffStatus = BuffStatus {
            id: 909,
            owner_id: player_id,
            duration_left_millisecond: 999999999,
            status_info: vec![StatusInfo::SpeedPercent(20)],
            duration_millisecond: 999999999,
            is_raidwide: false,
            name: "Greased Lightning".to_string(),
            stacks: 1,
            max_stacks: 1,
            trigger_proc_event_on_gcd: vec![],
        };

        Self::new(
            player_id,
            monk_job.clone(),
            power,
            None,
            FfxivPriorityTable::Monk(MonkPriorityTable::new(player_id)),
            HashMap::from([(StatusKey::new(909, player_id), GREASED_LIGHTNING_IV)]),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                DANCER_START_TIME_MILLISECOND,
                DANCER_START_TIME_MILLISECOND,
            ),
        )
    }

    pub fn new_dragoon(
        player_id: IdType,
        partner_player_id: IdType,
        power: CharacterPower,
        context: &FfxivContext,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> FfxivPlayer {
        let dragoon_job = context.jobs.get("DRG").unwrap();

        Self::new(
            player_id,
            dragoon_job.clone(),
            power,
            Some(partner_player_id),
            FfxivPriorityTable::Dragoon(DragoonPriorityTable::new(player_id, partner_player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                DRAGOON_START_TIME_MILLISECOND,
                DRAGOON_START_TIME_MILLISECOND,
            ),
        )
    }

    pub fn new_blackmage(
        player_id: IdType,
        power: CharacterPower,
        context: &FfxivContext,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> FfxivPlayer {
        let black_mage_job = context.jobs.get("BLM").unwrap();

        Self::new(
            player_id,
            black_mage_job.clone(),
            power,
            None,
            FfxivPriorityTable::Blackmage(BlackmagePriorityTable::new(player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Ogcd,
                BLACKMAGE_START_TIME_MILLISECOND,
                BLACKMAGE_START_TIME_MILLISECOND,
            ),
        )
    }

    pub fn new_whitemage(
        player_id: IdType,
        power: CharacterPower,
        context: &FfxivContext,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> FfxivPlayer {
        let whitemage_job = context.jobs.get("WHM").unwrap();

        Self::new(
            player_id,
            whitemage_job.clone(),
            power,
            None,
            FfxivPriorityTable::Whitemage(WhitemagePriorityTable::new(player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                WHITEMAGE_START_TIME_MILLISECOND,
                WHITEMAGE_START_TIME_MILLISECOND,
            ),
        )
    }
}
