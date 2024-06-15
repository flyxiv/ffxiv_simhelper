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
use crate::jobs_skill_data::paladin::priorities::PaladinPriorityTable;
use crate::jobs_skill_data::sage::priorities::SagePriorityTable;
use crate::jobs_skill_data::warrior::priorities::WarriorPriorityTable;
use crate::jobs_skill_data::white_mage::priorities::WhitemagePriorityTable;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::rotation::ffxiv_priority_table::FfxivPriorityTable;
use crate::skill::NON_GCD_DELAY_MILLISECOND;
use crate::status::buff_status::BuffStatus;
use crate::status::status_info::StatusInfo;
use crate::{IdType, TimeType};
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) static WARRIOR_START_TIME_MILLISECOND: TimeType = 0;
pub(crate) static PALADIN_START_TIME_MILLISECOND: TimeType = -2500;
pub(crate) static WHITEMAGE_START_TIME_MILLISECOND: TimeType = -1500;
pub(crate) static BLACKMAGE_START_TIME_MILLISECOND: TimeType = -5000;
pub(crate) static NINJA_START_TIME_MILLISECOND: TimeType = -2500;
pub(crate) static SAGE_START_TIME_MILLISECOND: TimeType = -1500 - NON_GCD_DELAY_MILLISECOND;
pub(crate) static BARD_START_TIME_MILLISECOND: TimeType = 0;
pub(crate) static DANCER_START_TIME_MILLISECOND: TimeType = -4000 - NON_GCD_DELAY_MILLISECOND;
pub(crate) static MONK_START_TIME_MILLISECOND: TimeType = 0;

pub(crate) static DRAGOON_START_TIME_MILLISECOND: TimeType = 0;

impl FfxivPlayer {
    pub fn new_ninja(
        player_id: IdType,
        power: CharacterPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> FfxivPlayer {
        let huton = get_huton_status(player_id);

        Self::new(
            player_id,
            String::from("NIN"),
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
            None,
        )
    }
    pub fn new_sage(
        player_id: IdType,
        power: CharacterPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("SGE"),
            power,
            None,
            FfxivPriorityTable::Sage(SagePriorityTable::new(player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Ogcd,
                SAGE_START_TIME_MILLISECOND,
                SAGE_START_TIME_MILLISECOND,
            ),
            Some(SAGE_START_TIME_MILLISECOND + NON_GCD_DELAY_MILLISECOND),
        )
    }

    pub fn new_bard(
        player_id: IdType,
        power: CharacterPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("BRD"),
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
            None,
        )
    }

    pub fn new_dancer(
        player_id: IdType,
        partner_player_id: IdType,
        power: CharacterPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("DNC"),
            power,
            Some(partner_player_id),
            FfxivPriorityTable::Dancer(DancerPriorityTable::new(player_id, partner_player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Ogcd,
                DANCER_START_TIME_MILLISECOND,
                DANCER_START_TIME_MILLISECOND,
            ),
            Some(DANCER_START_TIME_MILLISECOND + NON_GCD_DELAY_MILLISECOND),
        )
    }

    pub fn new_monk(
        player_id: IdType,
        power: CharacterPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> FfxivPlayer {
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
            String::from("MNK"),
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
            None,
        )
    }

    pub fn new_dragoon(
        player_id: IdType,
        partner_player_id: IdType,
        power: CharacterPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("DRG"),
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
            None,
        )
    }

    pub fn new_blackmage(
        player_id: IdType,
        power: CharacterPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> FfxivPlayer {
        let ENOCHIAN: BuffStatus = BuffStatus {
            id: 1907,
            owner_id: player_id,
            duration_left_millisecond: 999999999,
            status_info: vec![StatusInfo::DamagePercent(23)],
            duration_millisecond: 999999999,
            is_raidwide: false,
            name: "Enochian".to_string(),
            stacks: 1,
            max_stacks: 1,
            trigger_proc_event_on_gcd: vec![],
        };

        Self::new(
            player_id,
            String::from("BLM"),
            power,
            None,
            FfxivPriorityTable::Blackmage(BlackmagePriorityTable::new(player_id)),
            HashMap::from([(StatusKey::new(1707, player_id), ENOCHIAN)]),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Ogcd,
                BLACKMAGE_START_TIME_MILLISECOND,
                BLACKMAGE_START_TIME_MILLISECOND,
            ),
            Some(BLACKMAGE_START_TIME_MILLISECOND + 2 * NON_GCD_DELAY_MILLISECOND),
        )
    }

    pub fn new_whitemage(
        player_id: IdType,
        power: CharacterPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("WHM"),
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
            None,
        )
    }

    pub fn new_paladin(
        player_id: IdType,
        power: CharacterPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("PLD"),
            power,
            None,
            FfxivPriorityTable::Paladin(PaladinPriorityTable::new(player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                PALADIN_START_TIME_MILLISECOND,
                PALADIN_START_TIME_MILLISECOND,
            ),
            None,
        )
    }

    pub fn new_warrior(
        player_id: IdType,
        power: CharacterPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("WAR"),
            power,
            None,
            FfxivPriorityTable::Warrior(WarriorPriorityTable::new(player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                WARRIOR_START_TIME_MILLISECOND,
                WARRIOR_START_TIME_MILLISECOND,
            ),
            None,
        )
    }
}
