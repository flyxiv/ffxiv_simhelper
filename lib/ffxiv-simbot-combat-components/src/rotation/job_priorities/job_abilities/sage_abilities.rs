use crate::rotation::job_priorities::sage::SagePriorityTable;
use crate::rotation::priority_table::SkillPrerequisite;
use crate::rotation::SkillPriorityInfo;
use crate::skill::attack_skill::AttackSkill;
use lazy_static::lazy_static;

lazy_static! {
    static ref DOT: AttackSkill = AttackSkill {
        id: 0,
        name: String::from("Eukrasian Dosis III"),
        player_id: 0,
        potency: 750,
        trait_multiplier: 1.3,
        buff: None,
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 2500,
        gcd_cooldown_millisecond: 2500,
        charging_time_millisecond: 0,
        is_speed_buffed: true,
        cooldown_millisecond: 30000,
        resource_required: vec![],
        current_cooldown_millisecond: 0,
        stacks: 1,
    };
    static ref GCD: AttackSkill = AttackSkill {
        id: 1,
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
        current_cooldown_millisecond: 0,
        stacks: 1,
    };
    static ref PHLEGMA: AttackSkill = AttackSkill {
        id: 2,
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
        current_cooldown_millisecond: 0,
        stacks: 2,
    };
    static ref SAGE_OPENER: Vec<Option<AttackSkill>> = vec![
        Some(GCD.clone()),
        None,
        None,
        Some(DOT.clone()),
        None,
        None,
        Some(GCD.clone()),
        None,
        None,
        Some(GCD.clone()),
    ];
    static ref SAGE_PRIORITY_LIST: Vec<SkillPriorityInfo> = vec![
        SkillPriorityInfo {
            skill: DOT.clone(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill: PHLEGMA.clone(),
            prerequisite: Some(SkillPrerequisite::Or(
                SkillPrerequisite::HasStacks(2),
                SkillPrerequisite::MillisecondsBeforeBurst(0)
            )),
        },
        SkillPriorityInfo {
            skill: GCD.clone(),
            prerequisite: None,
        },
    ];
}
