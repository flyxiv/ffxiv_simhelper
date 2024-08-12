use crate::event::ffxiv_event::FfxivEvent;
use crate::jobs_skill_data::PotionSkill;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::damage_category::DamageCategory;
use crate::skill::use_type::UseType;
use crate::skill::{make_skill_table, ResourceTable};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_info::StatusInfo;
use crate::IdType;

pub(crate) struct SageDatabase {
    pub(crate) dot_status: DebuffStatus,
    pub(crate) eukrasian_dyskrasia_dot: DebuffStatus,
    pub(crate) dot: AttackSkill,
    pub(crate) gcd: AttackSkill,
    pub(crate) phlegma: AttackSkill,
    pub(crate) eukrasian_dyskrasia: AttackSkill,

    pub(crate) potion: AttackSkill,
    pub(crate) potion_buff: BuffStatus,
}

impl SageDatabase {
    pub(crate) fn new(player_id: IdType) -> Self {
        let DOT_STATUS: DebuffStatus = DebuffStatus {
            id: 700,
            owner_id: player_id,
            potency: Some(75),
            damage_category: Some(DamageCategory::MagicalDot),
            trait_percent: Some(130),
            damage_skill_id: Some(700),
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::None],
            duration_millisecond: 30000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            name: String::from("Eukrasian Dosis III"),
            snapshotted_buffs: Default::default(),
            snapshotted_debuffs: Default::default(),
        };
        let EUKRASIAN_DYSKRASIA_DOT: DebuffStatus = DebuffStatus {
            id: 701,
            owner_id: player_id,
            potency: Some(40),
            damage_category: Some(DamageCategory::MagicalDot),
            trait_percent: Some(130),
            damage_skill_id: Some(703),
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::None],
            duration_millisecond: 30000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            name: String::from("Eukrasian Dyskrasia"),
            snapshotted_buffs: Default::default(),
            snapshotted_debuffs: Default::default(),
        };
        let DOT: AttackSkill = AttackSkill {
            id: 700,
            name: String::from("Eukrasian Dosis III"),
            player_id,
            potency: 0,
            use_type: UseType::UseOnTarget,
            trait_percent: 130,
            additional_skill_events: vec![FfxivEvent::ApplyDebuff(
                0,
                DOT_STATUS.clone(),
                30000,
                30000,
                1000,
            )],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 1500,
            charging_time_millisecond: 1000,
            is_speed_buffed: true,
            cooldown_millisecond: 0,
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
            player_id,
            potency: 360,
            use_type: UseType::UseOnTarget,
            trait_percent: 130,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 1500,
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
            player_id,
            use_type: UseType::UseOnTarget,
            potency: 600,
            trait_percent: 130,
            additional_skill_events: vec![],
            proc_events: vec![],
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
        let EUKRASIAN_DYSKRASIA: AttackSkill = AttackSkill {
            id: 703,
            name: String::from("Eukrasian Dyskrasia"),
            player_id,
            potency: 0,
            use_type: UseType::UseOnTarget,
            trait_percent: 130,
            additional_skill_events: vec![FfxivEvent::ApplyDebuff(
                0,
                EUKRASIAN_DYSKRASIA_DOT.clone(),
                30000,
                30000,
                1000,
            )],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 1500,
            charging_time_millisecond: 1000,
            is_speed_buffed: true,
            cooldown_millisecond: 0,
            resource_required: vec![],
            resource_created: ResourceTable::new(),
            current_cooldown_millisecond: 0,
            stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            is_guaranteed_crit: false,
        };

        let potion_skill = PotionSkill::new(player_id);

        Self {
            dot_status: DOT_STATUS,
            eukrasian_dyskrasia_dot: EUKRASIAN_DYSKRASIA_DOT,
            dot: DOT,
            gcd: GCD,
            phlegma: PHLEGMA,
            eukrasian_dyskrasia: EUKRASIAN_DYSKRASIA,

            potion: potion_skill.potion,
            potion_buff: potion_skill.potion_buff,
        }
    }
}

pub(crate) fn make_sage_skills(player_id: IdType) -> SkillTable<AttackSkill> {
    let table = SageDatabase::new(player_id);

    let skills = vec![
        table.dot.clone(),
        table.gcd.clone(),
        table.phlegma.clone(),
        table.eukrasian_dyskrasia.clone(),
        table.potion,
    ];
    make_skill_table(skills)
}
