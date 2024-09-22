use crate::event::ffxiv_event::FfxivEvent;
use crate::jobs_skill_data::PotionSkill;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::damage_category::DamageCategory;
use crate::skill::use_type::UseType;
use crate::skill::{make_skill_table, ResourceRequirements};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_info::StatusInfo;
use crate::types::PlayerIdType;
use std::collections::HashMap;

pub(crate) struct WhitemageDatabase {
    pub(crate) glare3: AttackSkill,
    pub(crate) dia: AttackSkill,
    pub(crate) afflatus_misery: AttackSkill,
    pub(crate) afflatus_rapture: AttackSkill,
    pub(crate) assize: AttackSkill,
    pub(crate) presence_of_mind: AttackSkill,
    pub(crate) glare4: AttackSkill,

    pub(crate) dia_dot: DebuffStatus,

    pub(crate) potion: AttackSkill,
}

impl WhitemageDatabase {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        let dia_dot: DebuffStatus = DebuffStatus {
            id: 400,
            owner_id: player_id,
            potency: Some(75),
            trait_percent: Some(130),
            damage_category: Some(DamageCategory::MagicalDot),
            damage_skill_id: Some(401),
            duration_left_millisecond: 0,
            status_info: vec![],
            duration_millisecond: 30000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            name: String::from("Dia"),
            snapshotted_infos: Default::default(),
        };

        let presence_of_mind_buff: BuffStatus = BuffStatus {
            id: 401,
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::SpeedPercent(20)],
            duration_millisecond: 15000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            name: String::from("Presence of Mind"),
            trigger_proc_event_on_gcd: vec![],
        };

        let glare_iii: AttackSkill = AttackSkill {
            id: 400,
            name: "Glare III".to_string(),
            player_id,
            potency: 330,
            trait_percent: 130,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 1000,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            resource_required: vec![],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::UseOnTarget,
        };
        let dia: AttackSkill = AttackSkill {
            id: 401,
            name: "Dia".to_string(),
            player_id,
            potency: 75,
            trait_percent: 130,
            additional_skill_events: vec![FfxivEvent::ApplyDebuff(
                player_id,
                dia_dot.clone(),
                30000,
                30000,
                0,
            )],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            resource_required: vec![],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::UseOnTarget,
        };
        let afflatus_misery: AttackSkill = AttackSkill {
            id: 402,
            name: "Afflatus Misery".to_string(),
            player_id,
            potency: 1320,
            trait_percent: 130,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            resource_required: vec![ResourceRequirements::Resource(0, 3)],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::UseOnTarget,
        };
        let afflatus_rapture: AttackSkill = AttackSkill {
            id: 403,
            name: "Afflatus Rapture".to_string(),
            player_id,
            potency: 0,
            trait_percent: 130,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            resource_required: vec![ResourceRequirements::Resource(1, 1)],
            resource_created: HashMap::from([(0, 1)]),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::NoTarget,
        };
        let assize: AttackSkill = AttackSkill {
            id: 404,
            name: "Assize".to_string(),
            player_id,
            potency: 400,
            trait_percent: 130,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 0,
            charging_time_millisecond: 0,
            is_speed_buffed: false,
            cooldown_reduced_by_speed: false,
            resource_required: vec![],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 40000,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::NoTarget,
        };
        let presence_of_mind: AttackSkill = AttackSkill {
            id: 405,
            name: String::from("Presence of Mind"),
            player_id,
            potency: 0,
            trait_percent: 130,
            additional_skill_events: vec![FfxivEvent::ApplyBuff(
                player_id,
                player_id,
                presence_of_mind_buff.clone(),
                15000,
                15000,
                0,
            )],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 0,
            charging_time_millisecond: 0,
            is_speed_buffed: false,
            cooldown_reduced_by_speed: false,
            resource_required: vec![],
            resource_created: HashMap::from([(2, 3)]),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 120000,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::NoTarget,
        };
        let glare_iv: AttackSkill = AttackSkill {
            id: 406,
            name: "Glare IV".to_string(),
            player_id,
            potency: 640,
            trait_percent: 130,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            resource_required: vec![ResourceRequirements::Resource(2, 1)],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::UseOnTarget,
        };

        let potion_skill = PotionSkill::new(player_id);

        WhitemageDatabase {
            glare3: glare_iii,
            dia,
            afflatus_misery,
            afflatus_rapture,
            assize,
            presence_of_mind,
            glare4: glare_iv,
            dia_dot,
            potion: potion_skill.potion,
        }
    }
}

pub(crate) fn make_whitemage_skill_list(player_id: PlayerIdType) -> SkillTable<AttackSkill> {
    let db = WhitemageDatabase::new(player_id);

    let whitemage_skill_list: Vec<AttackSkill> = vec![
        db.glare3,
        db.dia,
        db.afflatus_misery,
        db.afflatus_rapture,
        db.assize,
        db.presence_of_mind,
        db.glare4,
        db.potion,
    ];

    make_skill_table(whitemage_skill_list)
}
