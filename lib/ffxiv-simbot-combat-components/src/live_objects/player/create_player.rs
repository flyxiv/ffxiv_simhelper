use crate::event::ffxiv_event::FfxivEvent;
use crate::event::FfxivEventQueue;
use crate::id_entity::IdEntity;
use crate::jobs_skill_data::astrologian::priorities::AstrologianPriorityTable;
use crate::jobs_skill_data::bard::priorities::BardPriorityTable;
use crate::jobs_skill_data::black_mage::priorities::BlackmagePriorityTable;
use crate::jobs_skill_data::dancer::priorities::DancerPriorityTable;
use crate::jobs_skill_data::darkknight::priorities::DarkknightPriorityTable;
use crate::jobs_skill_data::dragoon::priorities::DragoonPriorityTable;
use crate::jobs_skill_data::gunbreaker::priorities::GunbreakerPriorityTable;
use crate::jobs_skill_data::machinist::priorities::MachinistPriorityTable;
use crate::jobs_skill_data::monk::priorities::MonkPriorityTable;
use crate::jobs_skill_data::ninja::abilities::get_huton_status;
use crate::jobs_skill_data::ninja::priorities::NinjaPriorityTable;
use crate::jobs_skill_data::paladin::priorities::PaladinPriorityTable;
use crate::jobs_skill_data::pictomancer::priorities::PictomancerPriorityTable;
use crate::jobs_skill_data::reaper::priorities::ReaperPriorityTable;
use crate::jobs_skill_data::redmage::priorities::RedmagePriorityTable;
use crate::jobs_skill_data::sage::priorities::SagePriorityTable;
use crate::jobs_skill_data::samurai::priorities::SamuraiPriorityTable;
use crate::jobs_skill_data::scholar::priorities::ScholarPriorityTable;
use crate::jobs_skill_data::summoner::priorities::SummonerPriorityTable;
use crate::jobs_skill_data::viper::priorities::ViperPriorityTable;
use crate::jobs_skill_data::warrior::priorities::WarriorPriorityTable;
use crate::jobs_skill_data::white_mage::priorities::WhitemagePriorityTable;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::player_power::PlayerPower;
use crate::live_objects::player::StatusKey;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::rotation::ffxiv_priority_table::FfxivPriorityTable;
use crate::skill::NON_GCD_DELAY_MILLISECOND;
use crate::status::buff_status::BuffStatus;
use crate::status::status_info::StatusInfo;
use crate::types::{IdType, TimeType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) static PALADIN_START_TIME_MILLISECOND: TimeType = -2500;
pub(crate) static WARRIOR_START_TIME_MILLISECOND: TimeType = 0;
pub(crate) static DARKKNIGHT_START_TIME_MILLISECOND: TimeType = 0;
pub(crate) static GUNBREAKER_START_TIME_MILLISECOND: TimeType = 0;
pub(crate) static WHITEMAGE_START_TIME_MILLISECOND: TimeType = -1500;
pub(crate) static SCHOLAR_START_TIME_MILLISECOND: TimeType = -1500;
pub(crate) static ASTROLOGIAN_START_TIME_MILLISECOND: TimeType = -1500;
pub(crate) static SAGE_START_TIME_MILLISECOND: TimeType = -1500 - NON_GCD_DELAY_MILLISECOND;
pub(crate) static NINJA_START_TIME_MILLISECOND: TimeType = -2500;
pub(crate) static MONK_START_TIME_MILLISECOND: TimeType = 0;
pub(crate) static DRAGOON_START_TIME_MILLISECOND: TimeType = 0;
pub(crate) static SAMURAI_START_TIME_MILLISECOND: TimeType = -13000;
pub(crate) static REAPER_START_TIME_MILLISECOND: TimeType = 0;
pub(crate) static VIPER_START_TIME_MILLISECOND: TimeType = -700;
pub(crate) static BARD_START_TIME_MILLISECOND: TimeType = -700;
pub(crate) static DANCER_START_TIME_MILLISECOND: TimeType = -15000;
pub(crate) static MACHINIST_START_TIME_MILLISECOND: TimeType = -2000;
pub(crate) static BLACKMAGE_START_TIME_MILLISECOND: TimeType = -4000;
pub(crate) static SUMMONER_START_TIME_MILLISECOND: TimeType = -1500;
pub(crate) static REDMAGE_START_TIME_MILLISECOND: TimeType = -5500;
pub(crate) static PICTOMANCER_START_TIME_MILLISECOND: TimeType = -4000;

impl FfxivPlayer {
    pub fn new_paladin(
        player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("PLD"),
            power,
            None,
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
            player_count,
        )
    }

    pub fn new_warrior(
        player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("WAR"),
            power,
            None,
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
            player_count,
        )
    }

    pub fn new_darkknight(
        player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("DRK"),
            power,
            None,
            None,
            FfxivPriorityTable::Darkknight(DarkknightPriorityTable::new(player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                DARKKNIGHT_START_TIME_MILLISECOND,
                DARKKNIGHT_START_TIME_MILLISECOND,
            ),
            None,
            player_count,
        )
    }
    pub fn new_gunbreaker(
        player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("GNB"),
            power,
            None,
            None,
            FfxivPriorityTable::Gunbreaker(GunbreakerPriorityTable::new(player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                GUNBREAKER_START_TIME_MILLISECOND,
                GUNBREAKER_START_TIME_MILLISECOND,
            ),
            None,
            player_count,
        )
    }

    pub fn new_whitemage(
        player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("WHM"),
            power,
            None,
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
            player_count,
        )
    }

    pub fn new_scholar(
        player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("SCH"),
            power,
            None,
            None,
            FfxivPriorityTable::Scholar(ScholarPriorityTable::new(player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                SCHOLAR_START_TIME_MILLISECOND,
                SCHOLAR_START_TIME_MILLISECOND,
            ),
            None,
            player_count,
        )
    }
    pub fn new_astrologian(
        player_id: IdType,
        power: PlayerPower,
        partner_id1: IdType,
        partner_id2: IdType,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("AST"),
            power,
            Some(partner_id1),
            Some(partner_id2),
            FfxivPriorityTable::Astrologian(AstrologianPriorityTable::new(player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                ASTROLOGIAN_START_TIME_MILLISECOND,
                ASTROLOGIAN_START_TIME_MILLISECOND,
            ),
            None,
            player_count,
        )
    }

    pub fn new_sage(
        player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("SGE"),
            power,
            None,
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
            Some(SAGE_START_TIME_MILLISECOND + NON_GCD_DELAY_MILLISECOND),
            player_count,
        )
    }

    pub fn new_dragoon(
        player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("DRG"),
            power,
            None,
            None,
            FfxivPriorityTable::Dragoon(DragoonPriorityTable::new(player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                DRAGOON_START_TIME_MILLISECOND,
                DRAGOON_START_TIME_MILLISECOND,
            ),
            None,
            player_count,
        )
    }

    pub fn new_monk(
        player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
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
            None,
            FfxivPriorityTable::Monk(MonkPriorityTable::new(player_id)),
            HashMap::from([(StatusKey::new(909, player_id), GREASED_LIGHTNING_IV)]),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                MONK_START_TIME_MILLISECOND,
                MONK_START_TIME_MILLISECOND,
            ),
            None,
            player_count,
        )
    }
    pub fn new_ninja(
        player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        let huton = get_huton_status(player_id);

        Self::new(
            player_id,
            String::from("NIN"),
            power,
            None,
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
            player_count,
        )
    }

    pub fn new_samurai(
        player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("SAM"),
            power,
            None,
            None,
            FfxivPriorityTable::Samurai(SamuraiPriorityTable::new(player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Ogcd,
                0,
                SAMURAI_START_TIME_MILLISECOND,
            ),
            Some(0),
            player_count,
        )
    }

    pub fn new_reaper(
        player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("RPR"),
            power,
            None,
            None,
            FfxivPriorityTable::Reaper(ReaperPriorityTable::new(player_id, player_count)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                REAPER_START_TIME_MILLISECOND,
                REAPER_START_TIME_MILLISECOND,
            ),
            None,
            player_count,
        )
    }

    pub fn new_viper(
        player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("VPR"),
            power,
            None,
            None,
            FfxivPriorityTable::Viper(ViperPriorityTable::new(player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                VIPER_START_TIME_MILLISECOND,
                VIPER_START_TIME_MILLISECOND,
            ),
            Some(0),
            player_count,
        )
    }

    pub fn new_bard(
        player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("BRD"),
            power,
            None,
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
            player_count,
        )
    }

    pub fn new_machinist(
        player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("MCH"),
            power,
            None,
            None,
            FfxivPriorityTable::Machinist(MachinistPriorityTable::new(player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Ogcd,
                MACHINIST_START_TIME_MILLISECOND,
                MACHINIST_START_TIME_MILLISECOND,
            ),
            Some(0),
            player_count,
        )
    }
    pub fn new_dancer(
        player_id: IdType,
        partner_player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("DNC"),
            power,
            Some(partner_player_id),
            None,
            FfxivPriorityTable::Dancer(DancerPriorityTable::new(player_id, partner_player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                DANCER_START_TIME_MILLISECOND,
                DANCER_START_TIME_MILLISECOND,
            ),
            Some(DANCER_START_TIME_MILLISECOND + NON_GCD_DELAY_MILLISECOND),
            player_count,
        )
    }

    pub fn new_blackmage(
        player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        let ENOCHIAN: BuffStatus = BuffStatus {
            id: 1907,
            owner_id: player_id,
            duration_left_millisecond: 999999999,
            status_info: vec![StatusInfo::DamagePercent(33)],
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
            None,
            FfxivPriorityTable::Blackmage(BlackmagePriorityTable::new(player_id)),
            HashMap::from([(StatusKey::new(1707, player_id), ENOCHIAN)]),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                BLACKMAGE_START_TIME_MILLISECOND,
                BLACKMAGE_START_TIME_MILLISECOND,
            ),
            Some(BLACKMAGE_START_TIME_MILLISECOND + 2 * NON_GCD_DELAY_MILLISECOND),
            player_count,
        )
    }

    pub fn new_summoner(
        player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("SMN"),
            power,
            None,
            None,
            FfxivPriorityTable::Summoner(SummonerPriorityTable::new(
                player_id,
                ffxiv_event_queue.clone(),
            )),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                SUMMONER_START_TIME_MILLISECOND,
                SUMMONER_START_TIME_MILLISECOND,
            ),
            None,
            player_count,
        )
    }

    pub fn new_redmage(
        player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("RDM"),
            power,
            None,
            None,
            FfxivPriorityTable::Redmage(RedmagePriorityTable::new(player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                REDMAGE_START_TIME_MILLISECOND,
                REDMAGE_START_TIME_MILLISECOND,
            ),
            None,
            player_count,
        )
    }

    pub fn new_pictomancer(
        player_id: IdType,
        power: PlayerPower,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> FfxivPlayer {
        Self::new(
            player_id,
            String::from("PCT"),
            power,
            None,
            None,
            FfxivPriorityTable::Pictomancer(PictomancerPriorityTable::new(player_id)),
            Default::default(),
            ffxiv_event_queue,
            FfxivEvent::PlayerTurn(
                player_id,
                FfxivTurnType::Gcd,
                PICTOMANCER_START_TIME_MILLISECOND,
                PICTOMANCER_START_TIME_MILLISECOND,
            ),
            None,
            player_count,
        )
    }
}
