use crate::event::ffxiv_event::FfxivEvent;
use crate::id_entity::IdEntity;
use crate::rotation::job_priorities::priority_table::{Opener, SkillPrerequisite};
use crate::rotation::job_priorities::SkillTable;
use crate::rotation::SkillPriorityInfo;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::job_abilities::make_skill_table;
use crate::skill::ResourceTable;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_info::StatusInfo;
use crate::IdType;

struct SageDatabase {
    dot_status: DebuffStatus,
    dot: AttackSkill,
    gcd: AttackSkill,
    phlegma: AttackSkill,
}

impl SageDatabase {
    fn new() -> Self {
        let DOT_STATUS: DebuffStatus = DebuffStatus {
            id: 800,
            owner_id: 0,
            potency: Some(70),
            duration_left_millisecond: 0,
            status_info: StatusInfo::None,
            duration_millisecond: 30000,
            is_raidwide: false,
            stacks: 1,
            name: String::from("Eukrasian Dosis III"),
            snapshotted_buffs: None,
            snapshotted_debuffs: None,
        };
        let DOT: AttackSkill = AttackSkill {
            id: 700,
            name: String::from("Eukrasian Dosis III"),
            player_id: 0,
            potency: 750,
            trait_multiplier: 1.3,
            buff_events: vec![],
            debuff_events: vec![FfxivEvent::ApplyDebuff(
                0,
                DOT_STATUS.clone(),
                30000,
                30000,
                0,
            )],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 1500,
            charging_time_millisecond: 1000,
            is_speed_buffed: true,
            cooldown_millisecond: 30000,
            resource_required: vec![],
            resource_created: ResourceTable::new(),
            current_cooldown_millisecond: 0,
            stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            is_guaranteed_crit: false,
        };
        let GCD: AttackSkill = AttackSkill {
            id: 701,
            name: String::from("Dosis III"),
            player_id: 0,
            potency: 330,
            trait_multiplier: 1.3,
            buff_events: vec![],
            debuff_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 2500,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_millisecond: 0,
            resource_required: vec![],
            resource_created: ResourceTable::new(),
            current_cooldown_millisecond: 0,
            stacks: 1,
            stack_skill_id: None,
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
        };
        let PHLEGMA: AttackSkill = AttackSkill {
            id: 702,
            name: String::from("Phlegma III"),
            player_id: 0,
            potency: 600,
            trait_multiplier: 1.3,
            buff_events: vec![],
            debuff_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: false,
            cooldown_millisecond: 40000,
            resource_required: vec![],
            resource_created: ResourceTable::new(),
            current_cooldown_millisecond: 0,
            stacks: 2,
            stack_skill_id: None,
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
        };

        Self {
            dot_status: DOT_STATUS,
            dot: DOT,
            gcd: GCD,
            phlegma: PHLEGMA,
        }
    }
}

pub(crate) fn make_sage_gcd_priority_table() -> Vec<SkillPriorityInfo> {
    let priority_table = SageDatabase::new();

    let sage_priority_list: Vec<SkillPriorityInfo> = vec![
        SkillPriorityInfo {
            skill_id: priority_table.phlegma.get_id(),
            prerequisite: Some(SkillPrerequisite::Or(
                Box::new(SkillPrerequisite::HasStacks(702, 2)),
                Box::new(SkillPrerequisite::MillisecondsBeforeBurst(0)),
            )),
        },
        SkillPriorityInfo {
            skill_id: priority_table.dot.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: priority_table.gcd.get_id(),
            prerequisite: None,
        },
    ];

    sage_priority_list
}

pub(crate) fn make_sage_opener() -> Vec<Opener> {
    let table = SageDatabase::new();

    let sage_opener: Vec<Opener> = vec![
        Opener::GcdOpener(table.gcd.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(table.dot.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(table.gcd.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(table.gcd.get_id()),
    ];

    sage_opener
}

pub(crate) fn make_sage_skills(player_id: IdType) -> SkillTable<AttackSkill> {
    let table = SageDatabase::new();

    let all_skills = vec![table.dot.clone(), table.gcd.clone(), table.phlegma.clone()];

    make_skill_table(player_id, all_skills)
}
