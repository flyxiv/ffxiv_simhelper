use crate::event::ffxiv_event::FfxivEvent;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::use_type::UseType;
use crate::skill::{make_skill_table, ResourceRequirements};
use crate::status::debuff_status::DebuffStatus;
use crate::IdType;
use std::collections::HashMap;

pub(crate) struct WhitemageDatabase {
    pub(crate) glare3: AttackSkill,
    pub(crate) dia: AttackSkill,
    pub(crate) afflatus_misery: AttackSkill,
    pub(crate) afflatus_rapture: AttackSkill,

    pub(crate) dia_dot: DebuffStatus,
}

impl WhitemageDatabase {
    pub(crate) fn new(player_id: IdType) -> Self {
        let DIA_DOT: DebuffStatus = DebuffStatus {
            id: 400,
            owner_id: player_id,
            potency: Some(65),
            damage_skill_id: Some(401),
            duration_left_millisecond: 0,
            status_info: vec![],
            duration_millisecond: 30000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            name: String::from("Dia"),
            snapshotted_buffs: Default::default(),
            snapshotted_debuffs: Default::default(),
        };

        let GLARE_III: AttackSkill = AttackSkill {
            id: 400,
            name: "Glare III".to_string(),
            player_id,
            potency: 310,
            trait_multiplier: 1.0,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 1000,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            resource_required: vec![],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            stack_skill_id: None,
            use_type: UseType::UseOnTarget,
        };
        let DIA: AttackSkill = AttackSkill {
            id: 401,
            name: "Dia".to_string(),
            player_id,
            potency: 65,
            trait_multiplier: 1.0,
            additional_skill_events: vec![FfxivEvent::ApplyDebuff(
                player_id,
                DIA_DOT.clone(),
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
            resource_required: vec![],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            stack_skill_id: None,
            use_type: UseType::UseOnTarget,
        };
        let AFFLATUS_MISERY: AttackSkill = AttackSkill {
            id: 402,
            name: "Afflatus Misery".to_string(),
            player_id,
            potency: 1240,
            trait_multiplier: 1.0,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            resource_required: vec![ResourceRequirements::Resource(0, 3)],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            stack_skill_id: None,
            use_type: UseType::UseOnTarget,
        };
        let AFFLATUS_RAPTURE: AttackSkill = AttackSkill {
            id: 403,
            name: "Afflatus Rapture".to_string(),
            player_id,
            potency: 0,
            trait_multiplier: 1.0,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            resource_required: vec![ResourceRequirements::Resource(1, 1)],
            resource_created: HashMap::from([(0, 1)]),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 0,
            stack_skill_id: None,
            use_type: UseType::NoTarget,
        };

        WhitemageDatabase {
            glare3: GLARE_III,
            dia: DIA,
            afflatus_misery: AFFLATUS_MISERY,
            afflatus_rapture: AFFLATUS_RAPTURE,
            dia_dot: DIA_DOT,
        }
    }
}

pub(crate) fn make_whitemage_skill_list(player_id: IdType) -> SkillTable<AttackSkill> {
    let db = WhitemageDatabase::new(player_id);

    let whitemage_skill_list: Vec<AttackSkill> =
        vec![db.glare3, db.dia, db.afflatus_misery, db.afflatus_rapture];

    make_skill_table(whitemage_skill_list)
}
