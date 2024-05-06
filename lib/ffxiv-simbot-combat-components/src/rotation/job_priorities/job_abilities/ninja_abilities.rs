use crate::id_entity::IdEntity;
use crate::rotation::job_priorities::job_abilities::{make_opener, make_skill_table};
use crate::rotation::job_priorities::SkillTable;
use crate::rotation::priority_table::SkillPrerequisite;
use crate::rotation::SkillPriorityInfo;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::ResourceRequirements;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_apply::{ApplyInfo, StatusApply};
use crate::status::status_event::StatusApplyType;
use crate::status::status_info::StatusInfo;
use crate::IdType;
use lazy_static::lazy_static;

lazy_static! {
    static ref HUTON_STATUS: BuffStatus = {
        BuffStatus {
            id: 1000,
            name: String::from("Huton"),
            owner_id: 0,
            duration_left_millisecond: 0,
            status_info: StatusInfo::SpeedPercent(15),
            duration_millisecond: 60000,
            is_raidwide: false,
        }
    };
    static ref RAIJUREADY: BuffStatus = BuffStatus {
        id: 1001,
        name: String::from("Raiju Ready"),
        owner_id: 0,
        duration_left_millisecond: 0,
        status_info: StatusInfo::None,
        duration_millisecond: 30000,
        is_raidwide: false,
    };
    static ref SUITON_STATUS: BuffStatus = BuffStatus {
        id: 1002,
        name: String::from("Suiton"),
        owner_id: 0,
        duration_left_millisecond: 0,
        status_info: StatusInfo::None,
        duration_millisecond: 20000,
        is_raidwide: false,
    };
    static ref MUG_STATUS: DebuffStatus = DebuffStatus {
        id: 1003,
        name: String::from("Mug"),
        owner_id: 0,
        duration_left_millisecond: 0,
        status_info: StatusInfo::DamagePercent(5),
        duration_millisecond: 20000,
        is_raidwide: true,
    };
    static ref TRICK_ATTACK_STATUS: DebuffStatus = DebuffStatus {
        id: 1004,
        name: String::from("Trick Attack"),
        owner_id: 0,
        duration_left_millisecond: 0,
        status_info: StatusInfo::DamagePercent(10),
        duration_millisecond: 15000,
        is_raidwide: false,
    };
    static ref KASSATSU_STATUS: BuffStatus = BuffStatus {
        id: 1005,
        name: String::from("Kassatsu"),
        owner_id: 0,
        duration_left_millisecond: 0,
        status_info: StatusInfo::None,
        duration_millisecond: 15000,
        is_raidwide: false,
    };
    static ref TENCHIJIN_STATUS: BuffStatus = BuffStatus {
        id: 1006,
        name: String::from("Ten Chi Jin"),
        owner_id: 0,
        duration_left_millisecond: 0,
        status_info: StatusInfo::None,
        duration_millisecond: 6000,
        is_raidwide: false,
    };
    static ref BUNSHIN_STATUS: BuffStatus = BuffStatus {
        id: 1007,
        name: String::from("Bunshin"),
        owner_id: 0,
        duration_left_millisecond: 0,
        status_info: StatusInfo::None,
        duration_millisecond: 45000,
        is_raidwide: false,
    };
    static ref MEISUI_STATUS: BuffStatus = BuffStatus {
        id: 1008,
        name: String::fomr("Meisui"),
        owner_id: 0,
        duration_left_millisecond: 0,
        status_info: StatusInfo::None,
        duration_millisecond: 30000,
        is_raidwide: false,
    };
    static ref HUTON: AttackSkill = AttackSkill {
        id: 1000,
        name: String::from("Huton"),
        player_id: 0,
        potency: 0,
        trait_multiplier: 1.0,
        buff: None,
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 1500,
        charging_time_millisecond: 1500,
        is_speed_buffed: false,
        cooldown_millisecond: 0,
        resource_required: vec![],
        resource1_created: 0,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref RAITON: AttackSkill = AttackSkill {
        id: 1001,
        name: String::from("Raiton"),
        player_id: 0,
        potency: 650,
        trait_multiplier: 1.0,
        buff: Some(StatusApply::new(RAIJUREADY.clone(), 1, StatusApplyType::SelfBuff)),
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 1500,
        charging_time_millisecond: 1000,
        is_speed_buffed: false,
        cooldown_millisecond: 0,
        resource_required: vec![],
        resource1_created: 0,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: Some(1023),
    };
    static ref RAIJU: AttackSkill = AttackSkill {
        id: 1002,
        name: String::from("Fleeting Raiju"),
        player_id: 0,
        potency: 560,
        trait_multiplier: 1.0,
        buff: None,
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 2500,
        charging_time_millisecond: 0,
        is_speed_buffed: false,
        cooldown_millisecond: 0,
        resource_required: vec![ResourceRequirements::UseStatus(1001)],
        resource1_created: 5,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref HYOSHO: AttackSkill = AttackSkill {
        id: 1003,
        name: String::from("Hyosho Ranryu"),
        player_id: 0,
        potency: 1300,
        trait_multiplier: 1.3,
        buff: None,
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 1500,
        charging_time_millisecond: 1000,
        is_speed_buffed: false,
        cooldown_millisecond: 0,
        resource_required: vec![ResourceRequirements::UseStatus(1005)],
        resource1_created: 0,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref SUITON: AttackSkill = AttackSkill {
        id: 1004,
        name: String::from("Suiton"),
        player_id: 0,
        potency: 500,
        trait_multiplier: 1.0,
        buff: Some(StatusApply::new(SUITON_STATUS.clone(), 0, StatusApplyType::SelfBuff))
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 1500,
        charging_time_millisecond: 1500,
        is_speed_buffed: false,
        cooldown_millisecond: 0,
        resource_required: vec![],
        resource1_created: 0,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: Some(1023),
    };
    static ref SPINNING_EDGE: AttackSkill = AttackSkill {
        id: 1005,
        name: String::from("Spinning Edge"),
        player_id: 0,
        potency: 220,
        trait_multiplier: 1.0,
        buff: None,
        debuff: None,
        combo: Some(1),
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 2500,
        charging_time_millisecond: 0,
        is_speed_buffed: true,
        cooldown_millisecond: 0,
        resource_required: vec![],
        resource1_created: 5,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref GUST_SLASH: AttackSkill = AttackSkill {
        id: 1006,
        name: String::from("Gust Slash"),
        player_id: 0,
        potency: 320,
        trait_multiplier: 1.0,
        buff: None,
        debuff: None,
        combo: Some(2),
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 2500,
        charging_time_millisecond: 0,
        is_speed_buffed: true,
        cooldown_millisecond: 0,
        resource_required: vec![],
        resource1_created: 5,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref AEOLIAN_EDGE: AttackSkill = AttackSkill {
        id: 1007,
        name: String::from("Aeolian Edge"),
        player_id: 0,
        potency: 440,
        trait_multiplier: 1.0,
        buff: None,
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 2500,
        charging_time_millisecond: 0,
        is_speed_buffed: true,
        cooldown_millisecond: 0,
        resource_required: vec![],
        resource1_created: 15,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref ARMOR_CRUSH: AttackSkill = AttackSkill {
        id: 1008,
        name: String::from("Armor Crush"),
        player_id: 0,
        potency: 420,
        trait_multiplier: 1.0,
        buff: Some(StatusApply::new_with_duration(HUTON_STATUS.clone(), 30000, 60000, 0, StatusApplyType::SelfBuff)),
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 2500,
        charging_time_millisecond: 0,
        is_speed_buffed: true,
        cooldown_millisecond: 0,
        resource_required: vec![],
        resource1_created: 15,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref MUG: AttackSkill = AttackSkill {
        id: 1009,
        name: String::from("Mug"),
        player_id: 0,
        potency: 150,
        trait_multiplier: 1.0,
        buff: None
        debuff: Some(StatusApply::new(MUG_STATUS.clone(), 0, StatusApplyType::TargetDebuff)),
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 0,
        charging_time_millisecond: 0,
        is_speed_buffed: false,
        cooldown_millisecond: 120000,
        resource_required: vec![],
        resource1_created: 40,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref TRICK_ATTACK: AttackSkill = AttackSkill {
        id: 1010,
        name: String::from("Trick Attack"),
        player_id: 0,
        potency: 400,
        trait_multiplier: 1.0,
        buff: None,
        debuff: Some(StatusApply::new(TRICK_ATTACK_STATUS.clone(), 0, StatusApplyType::TargetDebuff)),
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 0,
        charging_time_millisecond: 0,
        is_speed_buffed: false,
        cooldown_millisecond: 60000,
        resource_required: vec![ResourceRequirements::UseStatus(1002)],
        resource1_created: 0,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref KASSATSU: AttackSkill = AttackSkill {
        id: 1011,
        name: String::from("Kassatsu"),
        player_id: 0,
        potency: 0,
        trait_multiplier: 1.0,
        buff: Some(StatusApply::new(KASSATSU_STATUS.clone(), 0, StatusApplyType::SelfBuff)),
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 0,
        charging_time_millisecond: 0,
        is_speed_buffed: false,
        cooldown_millisecond: 60000,
        resource_required: vec![],
        resource1_created: 0,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref BHAVACAKRA: AttackSkill = AttackSkill {
        id: 1012,
        name: String::from("Bhavakacra"),
        player_id: 0,
        potency: 350,
        trait_multiplier: 1.0,
        buff: None,
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 0,
        charging_time_millisecond: 0,
        is_speed_buffed: false,
        cooldown_millisecond: 0,
        resource_required: vec![ResourceRequirements::StackResource1(50)],
        resource1_created: 0,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref TENCHIJIN: AttackSkill = AttackSkill {
        id: 1013,
        name: String::from("Ten Chi Jin"),
        player_id: 0,
        potency: 0,
        trait_multiplier: 1.0,
        buff: Some(StatusApply::new(TENCHIJIN_STATUS.clone(), 0, StatusApplyType::SelfBuff)),
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 0,
        charging_time_millisecond: 0,
        is_speed_buffed: false,
        cooldown_millisecond: 120000,
        resource_required: vec![],
        resource1_created: 0,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref FUMA_TENCHIJIN: AttackSkill = AttackSkill {
        id: 1014,
        name: String::from("Fuma Shuriken-TCJ"),
        player_id: 0,
        potency: 450,
        trait_multiplier: 1.0,
        buff: None,
        debuff: None,
        combo: Some(3),
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 1000,
        charging_time_millisecond: 0,
        is_speed_buffed: false,
        cooldown_millisecond: 0,
        resource_required: vec![ResourceRequirements::UseStatus(1006)],
        resource1_created: 0,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref RAITON_TENCHIJIN: AttackSkill = AttackSkill {
        id: 1015,
        name: String::from("Raiton-TCJ"),
        player_id: 0,
        potency: 560,
        trait_multiplier: 1.0,
        buff: None,
        debuff: None,
        combo: Some(4),
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 1000,
        charging_time_millisecond: 0,
        is_speed_buffed: false,
        cooldown_millisecond: 0,
        resource_required: vec![],
        resource1_created: 0,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref SUITON_TENCHIJIN: AttackSkill = AttackSkill {
        id: 1016,
        name: String::from("Suiton-TCJ"),
        player_id: 0,
        potency: 500,
        trait_multiplier: 1.0,
        buff: Some(StatusApply::AddOrRefreshStatus(ApplyInfo {
            status: SUITON_STATUS.clone(),
            refresh_duration: 20000,
            max_duration: 20000,
        }, StatusApplyType::SelfBuff)),
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 1000,
        charging_time_millisecond: 0,
        is_speed_buffed: false,
        cooldown_millisecond: 0,
        resource_required: vec![],
        resource1_created: 0,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref BUNSHIN: AttackSkill = AttackSkill {
        id: 1017,
        name: String::from("Bunshin"),
        player_id: 0,
        potency: 0,
        trait_multiplier: 1.0,
        buff: Some(StatusApply::new(BUNSHIN_STATUS.clone(), 0, StatusApplyType::SelfBuff)),
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 0,
        charging_time_millisecond: 0,
        is_speed_buffed: false,
        cooldown_millisecond: 90000,
        resource_required: vec![ResourceRequirements::StackResource1(50)],
        resource1_created: 0,
        resource2_created: 5,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref DREAM: AttackSkill = AttackSkill {
        id: 1018,
        name: String::from("Dream Within a Dream"),
        player_id: 0,
        potency: 450,
        trait_multiplier: 1.0,
        buff: None,
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 0,
        charging_time_millisecond: 0,
        is_speed_buffed: false,
        cooldown_millisecond: 60000,
        resource_required: vec![],
        resource1_created: 0,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref PHANTOM_KAMAITACHI: AttackSkill = AttackSkill {
        id: 1019,
        name: String::from("Phantom Kamaitachi"),
        player_id: 0,
        potency: 600,
        trait_multiplier: 1.0,
        buff: None,
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 2500,
        charging_time_millisecond: 0,
        is_speed_buffed: false,
        cooldown_millisecond: 0,
        resource_required: vec![ResourceRequirements::UseStatus(1007)],
        resource1_created: 10,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref MEISUI: AttackSkill = AttackSkill {
        id: 1020,
        name: String::from("Meisui"),
        player_id: 0,
        potency: 0,
        trait_multiplier: 1.0,
        buff: Some(StatusApply::new(MEISUI_STATUS.clone(), 0, StatusApplyType::SelfBuff)),
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 0,
        charging_time_millisecond: 0,
        is_speed_buffed: false,
        cooldown_millisecond: 0,
        resource_required: vec![ResourceRequirements::UseStatus(1002)],
        resource1_created: 50,
        resource2_created: 0,
        current_cooldown_millisecond: 120000,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref BHAVACAKRA_MEISUI: AttackSkill = AttackSkill {
        id: 1021,
        name: String::from("Bhavakacra-Meisui"),
        player_id: 0,
        potency: 500,
        trait_multiplier: 1.0,
        buff: None,
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 0,
        charging_time_millisecond: 0,
        is_speed_buffed: false,
        cooldown_millisecond: 0,
        resource_required: vec![
            ResourceRequirements::StackResource1(50),
            ResourceRequirements::UseStatus(1008)
        ],
        resource1_created: 0,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref BUNSHIN_STACK: AttackSkill = AttackSkill {
        id: 1022,
        name: String::from("Bunshin-Stack"),
        player_id: 0,
        potency: 150,
        trait_multiplier: 1.0,
        buff: None,
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 0,
        charging_time_millisecond: 0,
        is_speed_buffed: false,
        cooldown_millisecond: 0,
        resource_required: vec![],
        resource1_created: 0,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 1,
        stack_skill_id: None,
    };
    static ref MUDRA: AttackSkill = AttackSkill {
        id: 1023,
        name: String::from("Mudra"),
        player_id: 0,
        potency: 0,
        trait_multiplier: 1.0,
        buff: None,
        debuff: None,
        combo: None,
        delay_millisecond: None,
        casting_time_millisecond: 0,
        gcd_cooldown_millisecond: 0,
        charging_time_millisecond: 0,
        is_speed_buffed: false,
        cooldown_millisecond: 20000,
        resource_required: vec![],
        resource1_created: 0,
        resource2_created: 0,
        current_cooldown_millisecond: 0,
        stacks: 2,
        stack_skill_id: None,
    };
}

pub(crate) fn make_ninja_gcd_table() -> Vec<SkillPriorityInfo> {
    let ninja_gcd_priority_table: Vec<SkillPriorityInfo> = vec![
        SkillPriorityInfo {
            skill_id: FUMA_TENCHIJIN.get_id(),
            prerequisite: Some(SkillPrerequisite::HasBufforDebuff(1006)),
        },
        SkillPriorityInfo {
            skill_id: RAITON_TENCHIJIN.get_id(),
            prerequisite: Some(SkillPrerequisite::Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: SUITON_TENCHIJIN.get_id(),
            prerequisite: Some(SkillPrerequisite::Combo(Some(4))),
        },
        SkillPriorityInfo {
            skill_id: SUITON.get_id(),
            prerequisite: Some(SkillPrerequisite::And(
                Box::new(SkillPrerequisite::RelatedSkillCooldownLessThan(1010, 17000)),
                Box::new(SkillPrerequisite::Not(Box::new(
                    SkillPrerequisite::HasBufforDebuff(1002),
                ))),
            )),
        },
        SkillPriorityInfo {
            skill_id: ARMOR_CRUSH.get_id(),
            prerequisite: Some(SkillPrerequisite::And(
                Box::new(SkillPrerequisite::BufforDebuffLessThan(1000, 8000)),
                Box::new(SkillPrerequisite::Combo(Some(2))),
            )),
        },
        SkillPriorityInfo {
            skill_id: PHANTOM_KAMAITACHI.get_id(),
            prerequisite: Some(SkillPrerequisite::BufforDebuffLessThan(1007, 3000)),
        },
        SkillPriorityInfo {
            skill_id: HYOSHO.get_id(),
            prerequisite: Some(SkillPrerequisite::And(
                Box::new(SkillPrerequisite::HasBufforDebuff(1004)),
                Box::new(SkillPrerequisite::HasBufforDebuff(1005)),
            )),
        },
        SkillPriorityInfo {
            skill_id: RAITON.get_id(),
            prerequisite: Some(SkillPrerequisite::HasStacks(1023, 2)),
        },
        SkillPriorityInfo {
            skill_id: AEOLIAN_EDGE.get_id(),
            prerequisite: Some(SkillPrerequisite::And(
                Box::new(SkillPrerequisite::HasBufforDebuff(1004)),
                Box::new(SkillPrerequisite::And(
                    Box::new(SkillPrerequisite::Combo(Some(2))),
                    Box::new(SkillPrerequisite::And(
                        Box::new(SkillPrerequisite::HasResource2(1)),
                        Box::new(SkillPrerequisite::Not(Box::new(
                            SkillPrerequisite::HasBufforDebuff(1001),
                        ))),
                    )),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: PHANTOM_KAMAITACHI.get_id(),
            prerequisite: Some(SkillPrerequisite::And(
                Box::new(SkillPrerequisite::MillisecondsBeforeBurst(0)),
                Box::new(SkillPrerequisite::HasBufforDebuff(1007)),
            )),
        },
        SkillPriorityInfo {
            skill_id: RAIJU.get_id(),
            prerequisite: Some(SkillPrerequisite::HasBufforDebuff(1001)),
        },
        SkillPriorityInfo {
            skill_id: RAITON.get_id(),
            prerequisite: Some(SkillPrerequisite::Or(
                Box::new(SkillPrerequisite::HasBufforDebuff(1004)),
                Box::new(SkillPrerequisite::HasBufforDebuff(1003)),
            )),
        },
        SkillPriorityInfo {
            skill_id: RAITON.get_id(),
            prerequisite: Some(SkillPrerequisite::Or(
                Box::new(SkillPrerequisite::HasBufforDebuff(1004)),
                Box::new(SkillPrerequisite::HasBufforDebuff(1003)),
            )),
        },
        SkillPriorityInfo {
            skill_id: ARMOR_CRUSH.get_id(),
            prerequisite: Some(SkillPrerequisite::And(
                Box::new(SkillPrerequisite::Not(Box::new(
                    SkillPrerequisite::MillisecondsBeforeBurst(0),
                ))),
                Box::new(SkillPrerequisite::And(
                    Box::new(SkillPrerequisite::BufforDebuffLessThan(1000, 30000)),
                    Box::new(SkillPrerequisite::Combo(Some(2))),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: AEOLIAN_EDGE.get_id(),
            prerequisite: Some(SkillPrerequisite::Combo(Some(2))),
        },
        SkillPriorityInfo {
            skill_id: GUST_SLASH.get_id(),
            prerequisite: Some(SkillPrerequisite::Combo(Some(1))),
        },
        SkillPriorityInfo {
            skill_id: SPINNING_EDGE.get_id(),
            prerequisite: None,
        },
    ];

    ninja_gcd_priority_table
}

pub(crate) fn make_ninja_ogcd_table() -> Vec<SkillPriorityInfo> {
    // TODO: calculate future ninki
    let mut ninja_ogcd_table: Vec<SkillPriorityInfo> = vec![
        SkillPriorityInfo {
            skill_id: BUNSHIN.get_id(),
            prerequisite: Some(SkillPrerequisite::HasResource1(50)),
        },
        SkillPriorityInfo {
            skill_id: BHAVACAKRA.get_id(),
            prerequisite: Some(SkillPrerequisite::Or(
                Box::new(SkillPrerequisite::HasBufforDebuff(1008)),
                Box::new(SkillPrerequisite::And(
                    Box::new(SkillPrerequisite::MillisecondsBeforeBurst(0)),
                    Box::new(SkillPrerequisite::HasResource1(50)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: BHAVACAKRA.get_id(),
            prerequisite: Some(SkillPrerequisite::HasResource1(80)),
        },
        SkillPriorityInfo {
            skill_id: MUG.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: TRICK_ATTACK.get_id(),
            prerequisite: Some(SkillPrerequisite::HasBufforDebuff(1002)),
        },
        SkillPriorityInfo {
            skill_id: KASSATSU.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: DREAM.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: TENCHIJIN.get_id(),
            prerequisite: None,
        },
    ];

    ninja_ogcd_table
}

pub(crate) fn make_ninja_opener(player_id: IdType) -> Vec<Option<AttackSkill>> {
    let mut ninja_opener: Vec<Option<AttackSkill>> = vec![
        Some(SUITON.clone()),
        Some(KASSATSU.clone()),
        Some(SPINNING_EDGE.clone()),
        // TODO: Potion
        None,
        None,
        Some(GUST_SLASH.clone()),
        Some(MUG.clone()),
        Some(BUNSHIN.clone()),
        Some(PHANTOM_KAMAITACHI.clone()),
        Some(TRICK_ATTACK.clone()),
        Some(DREAM.clone()),
        Some(HYOSHO.clone()),
        Some(TENCHIJIN.clone()),
        Some(FUMA_TENCHIJIN.clone()),
        Some(RAITON_TENCHIJIN.clone()),
        Some(SUITON_TENCHIJIN.clone()),
        Some(MEISUI.clone()),
        Some(RAITON.clone()),
        Some(BHAVACAKRA_MEISUI.clone()),
        Some(RAIJU.clone()),
        None,
        None,
        Some(RAIJU.clone()),
        None,
        None,
        Some(AEOLIAN_EDGE.clone()),
        Some(BHAVACAKRA.clone()),
        None,
        Some(RAITON.clone()),
    ];

    make_opener(player_id, ninja_opener)
}

pub(crate) fn make_ninja_skill_list(player_id: IdType) -> SkillTable<AttackSkill> {
    let mut ninja_skill_list: Vec<AttackSkill> = vec![
        HUTON.clone(),
        RAITON.clone(),
        RAIJU.clone(),
        HYOSHO.clone(),
        SUITON.clone(),
        SPINNING_EDGE.clone(),
        GUST_SLASH.clone(),
        AEOLIAN_EDGE.clone(),
        ARMOR_CRUSH.clone(),
        MUG.clone(),
        TRICK_ATTACK.clone(),
        MUDRA.clone(),
        KASSATSU.clone(),
        BHAVACAKRA.clone(),
        TENCHIJIN.clone(),
        FUMA_TENCHIJIN.clone(),
        RAITON_TENCHIJIN.clone(),
        SUITON_TENCHIJIN.clone(),
        BUNSHIN.clone(),
        DREAM.clone(),
        PHANTOM_KAMAITACHI.clone(),
        MEISUI.clone(),
        BHAVACAKRA_MEISUI.clone(),
        BUNSHIN_STACK.clone(),
    ];

    make_skill_table(player_id, ninja_skill_list)
}

#[inline]
pub(crate) fn bunshin_gcd_ids() -> Vec<IdType> {
    vec![
        AEOLIAN_EDGE.id,
        GUST_SLASH.id,
        SPINNING_EDGE.id,
        ARMOR_CRUSH.id,
    ]
}

pub(crate) fn get_bunshin_stack(player_id: IdType) -> AttackSkill {
    let mut bunshin_stack = BUNSHIN_STACK.clone();
    bunshin_stack.player_id = player_id;
    bunshin_stack
}

pub(crate) fn get_huton_status(player_id: IdType) -> BuffStatus {
    let mut huton = HUTON_STATUS.clone();
    huton.owner_id = player_id;
    huton.duration_left_millisecond = 55000;

    huton
}
