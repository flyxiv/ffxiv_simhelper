use crate::id_entity::IdEntity;
use crate::rotation::job_priorities::job_abilities::{make_opener, make_skill_table};
use crate::rotation::job_priorities::SkillTable;
use crate::rotation::priority_table::SkillPrerequisite;
use crate::rotation::SkillPriorityInfo;
use crate::skill::attack_skill::AttackSkill;
use crate::IdType;
use lazy_static::lazy_static;

lazy_static! {
    static ref DOT: AttackSkill = AttackSkill {
        id: 700,
        name: String::from("Eukrasian Dosis III"),
        player_id: 0,
        potency: 750,
        trait_multiplier: 1.3,
        buff: None,
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 1500,
        charging_time_millisecond: 1000,
        is_speed_buffed: true,
        cooldown_millisecond: 30000,
        resource_required: vec![],
        resource1_created: 0,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref GCD: AttackSkill = AttackSkill {
        id: 701,
        name: String::from("Dosis III"),
        player_id: 0,
        potency: 330,
        trait_multiplier: 1.3,
        buff: None,
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 2500,
        gcd_cooldown_millisecond: 2500,
        charging_time_millisecond: 0,
        is_speed_buffed: true,
        cooldown_millisecond: 0,
        resource_required: vec![],
        resource1_created: 0,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None
    };
    static ref PHLEGMA: AttackSkill = AttackSkill {
        id: 702,
        name: String::from("Phlegma III"),
        player_id: 0,
        potency: 600,
        trait_multiplier: 1.3,
        buff: None,
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 2500,
        charging_time_millisecond: 0,
        is_speed_buffed: false,
        cooldown_millisecond: 40000,
        resource_required: vec![],
        resource1_created: 0,
        resource2_created: 1,
        current_cooldown_millisecond: 0,
        stacks: 2,
        stack_skill_id: None,
    };
}

pub(crate) fn make_sage_gcd_priority_table() -> Vec<SkillPriorityInfo> {
    let mut sage_priority_list: Vec<SkillPriorityInfo> = vec![
        SkillPriorityInfo {
            skill_id: PHLEGMA.get_id(),
            prerequisite: Some(SkillPrerequisite::Or(
                Box::new(SkillPrerequisite::HasStacks(702, 2)),
                Box::new(SkillPrerequisite::MillisecondsBeforeBurst(0)),
            )),
        },
        SkillPriorityInfo {
            skill_id: DOT.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: GCD.get_id(),
            prerequisite: None,
        },
    ];

    sage_priority_list
}

pub(crate) fn make_sage_opener(player_id: IdType) -> Vec<Option<AttackSkill>> {
    let mut sage_opener: Vec<Option<AttackSkill>> = vec![
        Some(GCD.clone()),
        None,
        None,
        Some(DOT.clone()),
        None,
        Some(GCD.clone()),
        None,
        None,
        Some(GCD.clone()),
    ];

    make_opener(player_id, sage_opener)
}

pub(crate) fn make_sage_skills(player_id: IdType) -> SkillTable<AttackSkill> {
    let mut all_skills = vec![DOT.clone(), GCD.clone(), PHLEGMA.clone()];

    make_skill_table(player_id, all_skills)
}
