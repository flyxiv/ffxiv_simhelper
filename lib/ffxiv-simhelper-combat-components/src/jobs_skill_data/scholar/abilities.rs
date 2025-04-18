use crate::event::ffxiv_event::FfxivEvent;
use crate::id_entity::IdEntity;
use crate::jobs_skill_data::PotionSkill;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::damage_category::DamageCategory;
use crate::skill::use_type::UseType;
use crate::skill::ResourceRequirements::UseBuff;
use crate::skill::{make_skill_table, ResourceRequirements};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_info::StatusInfo;
use crate::types::PlayerIdType;
use std::collections::HashMap;

pub(crate) struct ScholarDatabase {
    pub(crate) broil_iv: AttackSkill,
    pub(crate) biolysis: AttackSkill,
    pub(crate) aetherflow: AttackSkill,
    pub(crate) energy_drain: AttackSkill,
    pub(crate) dissipation: AttackSkill,
    pub(crate) chain_stratagem: AttackSkill,
    pub(crate) baneful_impaction: AttackSkill,

    pub(crate) biolysis_dot: DebuffStatus,

    pub(crate) potion: AttackSkill,
}

impl ScholarDatabase {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        let biolysis_dot: DebuffStatus = DebuffStatus {
            id: 600,
            owner_id: player_id,
            potency: Some(80),
            trait_percent: Some(130),
            damage_category: Some(DamageCategory::MagicalDot),
            damage_skill_id: Some(607),
            duration_left_millisecond: 0,
            status_info: vec![],
            duration_millisecond: 30000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            name: String::from("Biolysis"),
            snapshotted_infos: Default::default(),
        };

        let chain_stratagem_buff: DebuffStatus = DebuffStatus {
            id: 601,
            owner_id: player_id,
            potency: None,
            trait_percent: None,
            damage_category: None,
            damage_skill_id: None,
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::CritHitRatePercent(10)],
            duration_millisecond: 21000,
            is_raidwide: true,
            stacks: 1,
            max_stacks: 1,
            name: String::from("Chain Stratagem"),
            snapshotted_infos: Default::default(),
        };
        let impact_imminent: BuffStatus = BuffStatus {
            id: 602,
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![],
            duration_millisecond: 30000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            name: String::from("Impact Imminent"),
            trigger_proc_event_on_gcd: vec![],
        };
        let baneful_impaction_dot: DebuffStatus = DebuffStatus {
            id: 603,
            owner_id: player_id,
            potency: Some(140),
            trait_percent: Some(130),
            damage_category: Some(DamageCategory::MagicalDot),
            damage_skill_id: Some(608),
            duration_left_millisecond: 0,
            status_info: vec![],
            duration_millisecond: 15000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            name: String::from("Baneful Impaction"),
            snapshotted_infos: Default::default(),
        };

        let broil_iv: AttackSkill = AttackSkill {
            id: 600,
            name: "BROIL_IV".to_string(),
            player_id,
            potency: 310,
            trait_percent: 130,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 1200,
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
        let biolysis: AttackSkill = AttackSkill {
            id: 601,
            name: "Biolysis".to_string(),
            player_id,
            potency: 0,
            trait_percent: 130,
            additional_skill_events: vec![FfxivEvent::ApplyDebuff(
                player_id,
                biolysis_dot.clone(),
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
        let aetherflow: AttackSkill = AttackSkill {
            id: 602,
            name: "Aetherflow".to_string(),
            player_id,
            potency: 0,
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
            resource_created: HashMap::from([(0, 3)]),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 60000,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::UseOnTarget,
        };
        let energy_drain: AttackSkill = AttackSkill {
            id: 603,
            name: "Energy Drain".to_string(),
            player_id,
            potency: 100,
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
            resource_required: vec![ResourceRequirements::Resource(0, 1)],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::NoTarget,
        };

        let dissipation: AttackSkill = AttackSkill {
            id: 604,
            name: "Dissipation".to_string(),
            player_id,
            potency: 0,
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
            resource_created: HashMap::from([(0, 3)]),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 180000,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::UseOnTarget,
        };

        let chain_stratagem: AttackSkill = AttackSkill {
            id: 605,
            name: "Chain Stratagem".to_string(),
            player_id,
            potency: 0,
            trait_percent: 130,
            additional_skill_events: vec![
                FfxivEvent::ApplyDebuff(player_id, chain_stratagem_buff.clone(), 21000, 21000, 0),
                FfxivEvent::ApplyBuff(
                    player_id,
                    player_id,
                    impact_imminent.clone(),
                    30000,
                    30000,
                    0,
                ),
            ],
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
            cooldown_millisecond: 120000,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::UseOnTarget,
        };
        let baneful_impaction: AttackSkill = AttackSkill {
            id: 606,
            name: "Baneful Impaction".to_string(),
            player_id,
            potency: 0,
            trait_percent: 130,
            additional_skill_events: vec![FfxivEvent::ApplyDebuff(
                player_id,
                baneful_impaction_dot.clone(),
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
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            resource_required: vec![UseBuff(impact_imminent.get_id())],
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

        ScholarDatabase {
            broil_iv,
            biolysis,
            aetherflow,
            energy_drain,
            dissipation,
            chain_stratagem,
            baneful_impaction,

            biolysis_dot,

            potion: potion_skill.potion,
        }
    }
}

pub(crate) fn make_scholar_skill_list(player_id: PlayerIdType) -> SkillTable<AttackSkill> {
    let db = ScholarDatabase::new(player_id);

    let scholar_skill_list: Vec<AttackSkill> = vec![
        db.broil_iv,
        db.biolysis,
        db.aetherflow,
        db.energy_drain,
        db.dissipation,
        db.chain_stratagem,
        db.baneful_impaction,
        db.potion,
    ];

    make_skill_table(scholar_skill_list)
}
