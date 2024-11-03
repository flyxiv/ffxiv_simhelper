use crate::event::FfxivEventQueue;
use crate::rotation::priority_table::{Opener, PriorityTable};
use crate::rotation::SkillPriorityInfo;
use crate::types::{PlayerIdType, SkillIdType};
use std::cell::RefCell;
use std::rc::Rc;

use crate::id_entity::IdEntity;
use crate::jobs_skill_data::summoner::abilities::SummonerDatabase;
use crate::rotation::priority_table::Opener::{GcdOpener, OgcdOpener};
use crate::rotation::priority_table::SkillPrerequisite::{
    BufforDebuffLessThan, Combo, HasBufforDebuff, HasResource, MillisecondsBeforeBurst, Not,
};

#[derive(Clone)]
pub struct SummonerPriorityTable {
    turn_count: RefCell<SkillIdType>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for SummonerPriorityTable {
    fn get_opener_len(&self) -> usize {
        self.opener.len()
    }

    fn get_opener_at(&self, index: usize) -> Opener {
        self.opener[index].clone()
    }

    fn get_gcd_priority_table(&self) -> &[SkillPriorityInfo] {
        &self.gcd_priority_table
    }

    fn get_ogcd_priority_table(&self) -> &[SkillPriorityInfo] {
        &self.ogcd_priority_table
    }

    fn increment_turn(&self) {
        *self.turn_count.borrow_mut() += 1;
    }

    fn get_turn_count(&self) -> SkillIdType {
        *self.turn_count.borrow()
    }
}

impl SummonerPriorityTable {
    pub fn new(
        player_id: PlayerIdType,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        use_pots: bool,
    ) -> Self {
        let db = SummonerDatabase::new(player_id, ffxiv_event_queue);
        Self {
            turn_count: RefCell::new(0),
            opener: make_summoner_opener(&db, use_pots),
            gcd_priority_table: make_summoner_gcd_priority_table(&db),
            ogcd_priority_table: make_summoner_ogcd_priority_table(&db, use_pots),
        }
    }
}

pub(crate) fn make_summoner_opener(db: &SummonerDatabase, use_pots: bool) -> Vec<Opener> {
    let mut openers = if use_pots {
        vec![
            GcdOpener(db.ruin_iii.get_id()),
            OgcdOpener((None, None)),
            GcdOpener(db.summon_solar_bahamut.get_id()),
            OgcdOpener((Some(db.potion.get_id()), None)),
        ]
    } else {
        vec![
            GcdOpener(db.ruin_iii.get_id()),
            OgcdOpener((None, None)),
            GcdOpener(db.summon_solar_bahamut.get_id()),
            OgcdOpener((None, None)),
        ]
    };

    openers.extend(vec![
        GcdOpener(db.umbral_impulse.get_id()),
        OgcdOpener((Some(db.searing_light.get_id()), None)),
        GcdOpener(db.umbral_impulse.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.umbral_impulse.get_id()),
        OgcdOpener((Some(db.energy_drain.get_id()), None)),
        GcdOpener(db.umbral_impulse.get_id()),
        OgcdOpener((
            Some(db.enkindle_solar_bahamut.get_id()),
            Some(db.necrotize.get_id()),
        )),
        GcdOpener(db.umbral_impulse.get_id()),
        OgcdOpener((Some(db.sunflare.get_id()), Some(db.necrotize.get_id()))),
    ]);

    openers
}

pub(crate) fn make_summoner_gcd_priority_table(db: &SummonerDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.summon_solar_bahamut.get_id(),
            prerequisite: Some(HasBufforDebuff(db.searing_light_buff.get_id())),
        },
        SkillPriorityInfo {
            skill_id: db.summon_phoenix.get_id(),
            prerequisite: Some(Not(Box::new(HasResource(5, 1)))),
        },
        SkillPriorityInfo {
            skill_id: db.summon_bahamut.get_id(),
            prerequisite: Some(Not(Box::new(HasResource(5, 1)))),
        },
        SkillPriorityInfo {
            skill_id: db.umbral_impulse.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.fountain_of_fire.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.astral_impulse.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.summon_titan_ii.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.topaz_rite.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.summon_garuda_ii.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.slipstream.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.emerald_rite.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.summon_ifrit_ii.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.crimson_strike.get_id(),
            prerequisite: Some(Combo(Some(1))),
        },
        SkillPriorityInfo {
            skill_id: db.crimson_cyclone.get_id(),
            prerequisite: Some(Combo(Some(0))),
        },
        SkillPriorityInfo {
            skill_id: db.ruby_rite.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.ruin_iv.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.ruin_iii.get_id(),
            prerequisite: None,
        },
    ]
}

pub(crate) fn make_summoner_ogcd_priority_table(
    db: &SummonerDatabase,
    use_pots: bool,
) -> Vec<SkillPriorityInfo> {
    let mut ogcd_priorities = if use_pots {
        vec![SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: Some(MillisecondsBeforeBurst(9000)),
        }]
    } else {
        vec![]
    };

    ogcd_priorities.extend(vec![
        SkillPriorityInfo {
            skill_id: db.swiftcast.get_id(),
            prerequisite: Some(HasBufforDebuff(db.garuda_state.get_id())),
        },
        SkillPriorityInfo {
            skill_id: db.searing_light.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.mountain_buster.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.energy_drain.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.necrotize.get_id(),
            prerequisite: Some(MillisecondsBeforeBurst(0)),
        },
        SkillPriorityInfo {
            skill_id: db.enkindle_solar_bahamut.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.enkindle_phoenix.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.enkindle_bahamut.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.searing_flash.get_id(),
            prerequisite: Some(BufforDebuffLessThan(db.searing_light_buff.get_id(), 15000)),
        },
        SkillPriorityInfo {
            skill_id: db.sunflare.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.deathflare.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.necrotize.get_id(),
            prerequisite: None,
        },
    ]);

    ogcd_priorities
}
