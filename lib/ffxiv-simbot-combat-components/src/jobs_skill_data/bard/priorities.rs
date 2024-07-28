use crate::event::FfxivEventQueue;
use crate::id_entity::IdEntity;
use crate::jobs_skill_data::bard::abilities::BardDatabase;
use crate::rotation::priority_table::SkillPrerequisite::HasSkillStacks;
use crate::rotation::priority_table::{Opener, PriorityTable, SkillPrerequisite};
use crate::rotation::SkillPriorityInfo;
use crate::{IdType, TurnCount};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct BardPriorityTable {
    turn_count: RefCell<TurnCount>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for BardPriorityTable {
    fn get_opener_len(&self) -> usize {
        self.opener.len()
    }

    fn get_opener_at(&self, index: usize) -> Opener {
        self.opener[index].clone()
    }

    fn get_gcd_priority_table(&self) -> &Vec<SkillPriorityInfo> {
        &self.gcd_priority_table
    }

    fn get_ogcd_priority_table(&self) -> &Vec<SkillPriorityInfo> {
        &self.ogcd_priority_table
    }

    fn increment_turn(&self) {
        *self.turn_count.borrow_mut() += 1;
    }

    fn get_turn_count(&self) -> IdType {
        *self.turn_count.borrow()
    }
}

impl BardPriorityTable {
    pub fn new(player_id: IdType, ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>) -> Self {
        let db = BardDatabase::new(player_id, ffxiv_event_queue);
        Self {
            turn_count: RefCell::new(0),
            opener: make_bard_opener(&db),
            gcd_priority_table: make_bard_gcd_priority_table(&db),
            ogcd_priority_table: make_bard_ogcd_priority_table(&db),
        }
    }
}

pub(crate) fn make_bard_opener(db: &BardDatabase) -> Vec<Opener> {
    let bard_opener: Vec<Opener> = vec![
        Opener::GcdOpener(db.caustic_bite.get_id()),
        Opener::OgcdOpener((Some(db.wanderers_minuet.get_id()), Some(db.potion.get_id()))),
        Opener::GcdOpener(db.storm_bite.get_id()),
        Opener::OgcdOpener((
            Some(db.battle_voice.get_id()),
            Some(db.radiant_finale.get_id()),
        )),
        Opener::GcdOpener(db.burst_shot.get_id()),
        Opener::OgcdOpener((
            Some(db.raging_strike.get_id()),
            Some(db.empyreal_arrow.get_id()),
        )),
        Opener::GcdOpener(db.radiant_encore1.get_id()),
        Opener::OgcdOpener((Some(db.barrage.get_id()), Some(db.heartbreak_shot.get_id()))),
    ];

    bard_opener
}

pub(crate) fn make_bard_gcd_priority_table(db: &BardDatabase) -> Vec<SkillPriorityInfo> {
    let bard_gcd_priority_table: Vec<SkillPriorityInfo> = vec![
        SkillPriorityInfo {
            skill_id: db.iron_jaws.get_id(),
            prerequisite: Some(SkillPrerequisite::Or(
                Box::new(SkillPrerequisite::BufforDebuffLessThan(1300, 3000)),
                Box::new(SkillPrerequisite::BufforDebuffLessThan(1301, 3000)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.blast_arrow.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.refulgent_arrow_barrage.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.apex_arrow.get_id(),
            prerequisite: Some(SkillPrerequisite::HasResource(0, 16)),
        },
        SkillPriorityInfo {
            skill_id: db.iron_jaws.get_id(),
            prerequisite: Some(SkillPrerequisite::Or(
                Box::new(SkillPrerequisite::BufforDebuffLessThan(1300, 6000)),
                Box::new(SkillPrerequisite::BufforDebuffLessThan(1301, 6000)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.apex_arrow.get_id(),
            prerequisite: Some(SkillPrerequisite::HasResource(0, 80)),
        },
        SkillPriorityInfo {
            skill_id: db.resonant_arrow.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.radiant_encore3.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.refulgent_arrow.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.burst_shot.get_id(),
            prerequisite: None,
        },
    ];

    bard_gcd_priority_table
}

pub(crate) fn make_bard_ogcd_priority_table(db: &BardDatabase) -> Vec<SkillPriorityInfo> {
    let bard_ogcd_table: Vec<SkillPriorityInfo> = vec![
        SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.wanderers_minuet.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.armys_paeon.get_id(),
            prerequisite: Some(SkillPrerequisite::BufforDebuffLessThan(
                db.mages_ballad_status.get_id(),
                3000,
            )),
        },
        SkillPriorityInfo {
            skill_id: db.mages_ballad.get_id(),
            prerequisite: Some(SkillPrerequisite::BufforDebuffLessThan(
                db.wanderers_minuet_status.get_id(),
                3000,
            )),
        },
        SkillPriorityInfo {
            skill_id: db.pitch_perfect3.get_id(),
            prerequisite: Some(SkillPrerequisite::And(
                Box::new(SkillPrerequisite::HasResource(1, 3)),
                Box::new(SkillPrerequisite::BufforDebuffLessThan(1303, 3000)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.pitch_perfect2.get_id(),
            prerequisite: Some(SkillPrerequisite::And(
                Box::new(SkillPrerequisite::HasResource(1, 2)),
                Box::new(SkillPrerequisite::BufforDebuffLessThan(1303, 3000)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.pitch_perfect1.get_id(),
            prerequisite: Some(SkillPrerequisite::And(
                Box::new(SkillPrerequisite::HasResource(1, 1)),
                Box::new(SkillPrerequisite::BufforDebuffLessThan(1303, 3000)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.battle_voice.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.radiant_finale.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.pitch_perfect3.get_id(),
            prerequisite: Some(SkillPrerequisite::HasResource(1, 3)),
        },
        SkillPriorityInfo {
            skill_id: db.empyreal_arrow.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.raging_strike.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.barrage.get_id(),
            prerequisite: Some(SkillPrerequisite::MillisecondsBeforeBurst(0)),
        },
        SkillPriorityInfo {
            skill_id: db.heartbreak_shot.get_id(),
            prerequisite: Some(SkillPrerequisite::MillisecondsBeforeBurst(0)),
        },
        SkillPriorityInfo {
            skill_id: db.side_winder.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.heartbreak_shot.get_id(),
            prerequisite: Some(HasSkillStacks(1303, 2)),
        },
    ];

    bard_ogcd_table
}
