use crate::id_entity::IdEntity;
use crate::jobs_skill_data::pictomancer::abilities::PictomancerDatabase;
use crate::jobs_skill_data::pictomancer::combat_resources::{
    BLACK_PAINT_STACK_ID, CREATURE_STACK_ID, HAMMER_READY_ID, HAMMER_STACK_ID, HARD_GCD_STACK_ID,
    HAS_CREATURE_ID, SHOT_STACK_ID, STARRY_SKY_STACK_ID,
};
use crate::rotation::priority_table::SkillPrerequisite::{
    And, Combo, HasResource, HasResourceExactly, MillisecondsBeforeBurst, Not, Or,
};
use crate::rotation::priority_table::{Opener, PriorityTable};
use crate::rotation::SkillPriorityInfo;
use crate::types::{IdType, PlayerIdType};
use std::cell::RefCell;

#[derive(Clone)]
pub(crate) struct PictomancerPriorityTable {
    turn_count: RefCell<IdType>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for PictomancerPriorityTable {
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

impl PictomancerPriorityTable {
    pub fn new(player_id: PlayerIdType) -> Self {
        let db = PictomancerDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_pictomancer_opener(&db),
            gcd_priority_table: make_pictomancer_gcd_priority_table(&db),
            ogcd_priority_table: make_pictomancer_ogcd_priority_table(&db),
        }
    }
}

pub(crate) fn make_pictomancer_opener(db: &PictomancerDatabase) -> Vec<Opener> {
    vec![
        Opener::GcdOpener(db.rainbow_drip.get_id()),
        Opener::OgcdOpener((Some(db.living_muse.get_id()), None)),
        Opener::GcdOpener(db.winged_motif.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.holy_in_white.get_id()),
        Opener::OgcdOpener((Some(db.potion.get_id()), Some(db.starry_muse.get_id()))),
        Opener::GcdOpener(db.star_prism.get_id()),
        Opener::OgcdOpener((Some(db.subtractive_pallete_proc.get_id()), None)),
        Opener::GcdOpener(db.comet_in_black_hyperphantasia.get_id()),
        Opener::OgcdOpener((
            Some(db.living_muse.get_id()),
            Some(db.mog_of_the_ages.get_id()),
        )),
        Opener::GcdOpener(db.blizzard_in_cyan_hyperphantasia.get_id()),
        Opener::OgcdOpener((Some(db.striking_muse.get_id()), None)),
        Opener::GcdOpener(db.stone_in_yellow_hyperphantasia.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.thunder_in_magenta_hyperphantasia.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.rainbow_drip_proc.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.hammer_stamp.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.hammer_brush.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.polishing_hammer.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.hammer_motif.get_id()),
        Opener::OgcdOpener((Some(db.striking_muse.get_id()), None)),
    ]
}

pub(crate) fn make_pictomancer_gcd_priority_table(
    db: &PictomancerDatabase,
) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.star_prism.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.comet_in_black_hyperphantasia.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.thunder_in_magenta_hyperphantasia.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.stone_in_yellow_hyperphantasia.get_id(),
            prerequisite: Some(Combo(Some(2))),
        },
        SkillPriorityInfo {
            skill_id: db.blizzard_in_cyan_hyperphantasia.get_id(),
            prerequisite: Some(Combo(Some(1))),
        },
        SkillPriorityInfo {
            skill_id: db.rainbow_drip_proc.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.polishing_hammer.get_id(),
            prerequisite: Some(HasResourceExactly(HAMMER_STACK_ID, 1)),
        },
        SkillPriorityInfo {
            skill_id: db.hammer_brush.get_id(),
            prerequisite: Some(HasResourceExactly(HAMMER_STACK_ID, 2)),
        },
        SkillPriorityInfo {
            skill_id: db.hammer_stamp.get_id(),
            prerequisite: Some(HasResourceExactly(HAMMER_STACK_ID, 3)),
        },
        SkillPriorityInfo {
            skill_id: db.pom_motif.get_id(),
            prerequisite: Some(And(
                Box::new(Or(
                    Box::new(HasResourceExactly(CREATURE_STACK_ID, 0)),
                    Box::new(HasResourceExactly(CREATURE_STACK_ID, 4)),
                )),
                Box::new(HasResourceExactly(HAS_CREATURE_ID, 0)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.winged_motif.get_id(),
            prerequisite: Some(And(
                Box::new(HasResourceExactly(CREATURE_STACK_ID, 1)),
                Box::new(HasResourceExactly(HAS_CREATURE_ID, 0)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.claw_motif.get_id(),
            prerequisite: Some(And(
                Box::new(HasResourceExactly(CREATURE_STACK_ID, 2)),
                Box::new(HasResourceExactly(HAS_CREATURE_ID, 0)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.maw_motif.get_id(),
            prerequisite: Some(And(
                Box::new(HasResourceExactly(CREATURE_STACK_ID, 3)),
                Box::new(HasResourceExactly(HAS_CREATURE_ID, 0)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.hammer_motif.get_id(),
            prerequisite: Some(HasResourceExactly(HAMMER_READY_ID, 0)),
        },
        SkillPriorityInfo {
            skill_id: db.starry_sky_motif.get_id(),
            prerequisite: Some(HasResourceExactly(STARRY_SKY_STACK_ID, 0)),
        },
        SkillPriorityInfo {
            skill_id: db.comet_in_black.get_id(),
            prerequisite: Some(HasResourceExactly(0, 50)),
        },
        SkillPriorityInfo {
            skill_id: db.thunder_in_magenta.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.stone_in_yellow.get_id(),
            prerequisite: Some(Combo(Some(2))),
        },
        SkillPriorityInfo {
            skill_id: db.blizzard_in_cyan.get_id(),
            prerequisite: Some(Combo(Some(1))),
        },
        SkillPriorityInfo {
            skill_id: db.water_in_blue.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.aero_in_green.get_id(),
            prerequisite: Some(Combo(Some(2))),
        },
        SkillPriorityInfo {
            skill_id: db.fire_in_red.get_id(),
            prerequisite: None,
        },
    ]
}

pub(crate) fn make_pictomancer_ogcd_priority_table(
    db: &PictomancerDatabase,
) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.starry_muse.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.subtractive_pallete_proc.get_id(),
            prerequisite: Some(And(
                Box::new(HasResourceExactly(BLACK_PAINT_STACK_ID, 0)),
                Box::new(HasResourceExactly(HARD_GCD_STACK_ID, 0)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.living_muse.get_id(),
            prerequisite: Some(Not(Box::new(HasResourceExactly(SHOT_STACK_ID, 2)))),
        },
        SkillPriorityInfo {
            skill_id: db.retribution_of_the_madeem.get_id(),
            prerequisite: Some(Or(
                Box::new(Not(Box::new(MillisecondsBeforeBurst(80000)))),
                Box::new(MillisecondsBeforeBurst(0)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.mog_of_the_ages.get_id(),
            prerequisite: Some(Or(
                Box::new(Not(Box::new(MillisecondsBeforeBurst(80000)))),
                Box::new(MillisecondsBeforeBurst(0)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.striking_muse.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.subtractive_pallete.get_id(),
            prerequisite: Some(And(
                Box::new(HasResourceExactly(BLACK_PAINT_STACK_ID, 0)),
                Box::new(HasResourceExactly(HARD_GCD_STACK_ID, 0)),
            )),
        },
    ]
}
