use crate::event::ffxiv_event::FfxivEvent::{ApplyBuff, ReduceSkillCooldown};
use crate::id_entity::IdEntity;
use crate::jobs_skill_data::PotionSkill;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::use_type::UseType;
use crate::skill::ResourceRequirements::UseBuff;
use crate::skill::{make_skill_table, ResourceRequirements};
use crate::status::buff_status::BuffStatus;
use crate::status::status_info::StatusInfo;
use crate::types::PlayerIdType;
use std::collections::HashMap;

pub(crate) struct WarriorDatabase {
    pub(crate) heavy_swing: AttackSkill,
    pub(crate) maim: AttackSkill,
    pub(crate) storms_eye: AttackSkill,
    pub(crate) infuriate: AttackSkill,
    pub(crate) fell_cleave: AttackSkill,
    pub(crate) onslaught: AttackSkill,
    pub(crate) upheaval: AttackSkill,
    pub(crate) inner_release: AttackSkill,
    pub(crate) primal_rend: AttackSkill,
    pub(crate) storms_path: AttackSkill,
    pub(crate) inner_chaos: AttackSkill,
    pub(crate) fell_cleave_inner: AttackSkill,
    pub(crate) primal_wrath: AttackSkill,
    pub(crate) primal_ruination: AttackSkill,

    pub(crate) surging_tempest: BuffStatus,
    pub(crate) nascent_chaos: BuffStatus,
    pub(crate) inner_release_stack: BuffStatus,

    pub(crate) potion: AttackSkill,
}

impl WarriorDatabase {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        let surging_tempest: BuffStatus = BuffStatus {
            id: 100,
            name: String::from("Surging Tempest"),
            stacks: 1,
            max_stacks: 1,
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::DamagePercent(10)],
            duration_millisecond: 30000,
            is_raidwide: false,
            trigger_proc_event_on_gcd: vec![],
        };
        let nascent_chaos: BuffStatus = BuffStatus {
            id: 101,
            name: String::from("Nascent Chaos"),
            stacks: 1,
            max_stacks: 1,
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::None],
            duration_millisecond: 30000,
            is_raidwide: false,
            trigger_proc_event_on_gcd: vec![],
        };
        let inner_release_stack: BuffStatus = BuffStatus {
            id: 102,
            name: String::from("Inner Release"),
            stacks: 3,
            max_stacks: 3,
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::None],
            duration_millisecond: 30000,
            is_raidwide: false,
            trigger_proc_event_on_gcd: vec![],
        };
        let primal_rend_ready: BuffStatus = BuffStatus {
            id: 103,
            name: String::from("Primal Rend Ready"),
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::None],
            duration_millisecond: 15000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            trigger_proc_event_on_gcd: vec![],
        };
        let primal_ruination_ready: BuffStatus = BuffStatus {
            id: 104,
            name: String::from("Primal Ruination Ready"),
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::None],
            duration_millisecond: 30000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            trigger_proc_event_on_gcd: vec![],
        };

        let heavy_swing: AttackSkill = AttackSkill {
            id: 100,
            name: String::from("Heavy Swing"),
            player_id,
            potency: 240,
            trait_percent: 100,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: Some(2),
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            cooldown_millisecond: 0,
            resource_required: vec![],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::NoTarget,
        };
        let maim: AttackSkill = AttackSkill {
            id: 101,
            name: String::from("Maim"),
            player_id,
            potency: 340,
            trait_percent: 100,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: Some(3),
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            cooldown_millisecond: 0,
            resource_required: vec![],
            resource_created: HashMap::from([(0, 10)]),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::UseOnTarget,
        };
        let storms_eye: AttackSkill = AttackSkill {
            id: 102,
            name: String::from("Storm's Eye"),
            player_id,
            potency: 500,
            trait_percent: 100,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                surging_tempest.clone(),
                30000,
                60000,
                0,
            )],
            proc_events: vec![],
            combo: Some(0),
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            cooldown_millisecond: 0,
            resource_required: vec![],
            resource_created: HashMap::from([(0, 10)]),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::UseOnTarget,
        };
        let infuriate: AttackSkill = AttackSkill {
            id: 103,
            name: String::from("Infuriate"),
            player_id,
            potency: 0,
            trait_percent: 100,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                nascent_chaos.clone(),
                30000,
                30000,
                0,
            )],
            proc_events: vec![],
            combo: Some(0),
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 0,
            charging_time_millisecond: 0,
            is_speed_buffed: false,
            cooldown_reduced_by_speed: false,
            cooldown_millisecond: 60000,
            resource_required: vec![],
            resource_created: HashMap::from([(0, 50)]),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 2,
            max_stacks: 2,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::UseOnTarget,
        };
        let fell_cleave: AttackSkill = AttackSkill {
            id: 104,
            name: String::from("Fell Cleave"),
            player_id,
            potency: 580,
            trait_percent: 100,
            additional_skill_events: vec![ReduceSkillCooldown(
                player_id,
                infuriate.get_id(),
                5000,
                0,
            )],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: false,
            cooldown_reduced_by_speed: false,
            cooldown_millisecond: 0,
            resource_required: vec![ResourceRequirements::Resource(0, 50)],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::UseOnTarget,
        };
        let onslaught: AttackSkill = AttackSkill {
            id: 105,
            name: String::from("Onslaught"),
            player_id,
            potency: 150,
            trait_percent: 100,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 0,
            charging_time_millisecond: 0,
            is_speed_buffed: false,
            cooldown_reduced_by_speed: false,
            cooldown_millisecond: 30000,
            resource_required: vec![],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 3,
            max_stacks: 3,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::UseOnTarget,
        };
        let upheaval: AttackSkill = AttackSkill {
            id: 106,
            name: String::from("Upheaval"),
            player_id,
            potency: 420,
            trait_percent: 100,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 0,
            charging_time_millisecond: 0,
            is_speed_buffed: false,
            cooldown_reduced_by_speed: false,
            cooldown_millisecond: 30000,
            resource_required: vec![],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::UseOnTarget,
        };
        let inner_release: AttackSkill = AttackSkill {
            id: 107,
            name: String::from("Inner Release"),
            player_id,
            potency: 0,
            trait_percent: 100,
            additional_skill_events: vec![
                ApplyBuff(
                    player_id,
                    player_id,
                    inner_release_stack.clone(),
                    30000,
                    30000,
                    0,
                ),
                ApplyBuff(
                    player_id,
                    player_id,
                    primal_rend_ready.clone(),
                    30000,
                    30000,
                    0,
                ),
                ApplyBuff(
                    player_id,
                    player_id,
                    surging_tempest.clone(),
                    10000,
                    60000,
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
            cooldown_millisecond: 60000,
            resource_required: vec![],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::NoTarget,
        };
        let primal_rend: AttackSkill = AttackSkill {
            id: 108,
            name: String::from("Primal Rend"),
            player_id,
            potency: 700,
            trait_percent: 100,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                primal_ruination_ready.clone(),
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
            cooldown_millisecond: 0,
            resource_required: vec![UseBuff(primal_rend_ready.get_id())],
            resource_created: Default::default(),
            is_guaranteed_crit: true,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: true,
            use_type: UseType::UseOnTarget,
        };
        let storms_path: AttackSkill = AttackSkill {
            id: 109,
            name: String::from("Storm's Path"),
            player_id,
            potency: 500,
            trait_percent: 100,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: Some(0),
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            cooldown_millisecond: 0,
            resource_required: vec![],
            resource_created: HashMap::from([(0, 20)]),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::UseOnTarget,
        };
        let inner_chaos: AttackSkill = AttackSkill {
            id: 110,
            name: String::from("Inner Chaos"),
            player_id,
            potency: 660,
            trait_percent: 100,
            additional_skill_events: vec![ReduceSkillCooldown(
                player_id,
                infuriate.get_id(),
                5000,
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
            cooldown_millisecond: 0,
            resource_required: vec![
                ResourceRequirements::Resource(0, 50),
                UseBuff(nascent_chaos.get_id()),
            ],
            resource_created: Default::default(),
            is_guaranteed_crit: true,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: true,
            use_type: UseType::UseOnTarget,
        };
        let fell_cleave_inner: AttackSkill = AttackSkill {
            id: 111,
            name: String::from("Fell Cleave"),
            player_id,
            potency: 580,
            trait_percent: 100,
            additional_skill_events: vec![ReduceSkillCooldown(
                player_id,
                infuriate.get_id(),
                5000,
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
            cooldown_millisecond: 0,
            resource_required: vec![UseBuff(inner_release_stack.get_id())],
            resource_created: HashMap::from([(1, 1)]),
            is_guaranteed_crit: true,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: true,
            use_type: UseType::UseOnTarget,
        };
        let primal_wrath: AttackSkill = AttackSkill {
            id: 112,
            name: String::from("Primal Wrath"),
            player_id,
            potency: 700,
            trait_percent: 100,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 0,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            cooldown_millisecond: 0,
            resource_required: vec![ResourceRequirements::Resource(1, 3)],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::UseOnTarget,
        };
        let primal_ruination: AttackSkill = AttackSkill {
            id: 113,
            name: String::from("Primal Ruination"),
            player_id,
            potency: 740,
            trait_percent: 100,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            cooldown_millisecond: 0,
            resource_required: vec![UseBuff(primal_ruination_ready.get_id())],
            resource_created: Default::default(),
            is_guaranteed_crit: true,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: true,
            use_type: UseType::UseOnTarget,
        };

        let potion_skill = PotionSkill::new(player_id);

        WarriorDatabase {
            heavy_swing,
            maim,
            storms_eye,
            infuriate,
            fell_cleave,
            onslaught,
            upheaval,
            inner_release,
            primal_rend,
            storms_path,
            inner_chaos,
            fell_cleave_inner,
            primal_wrath,
            primal_ruination,

            surging_tempest,
            nascent_chaos,

            inner_release_stack,

            potion: potion_skill.potion,
        }
    }
}

pub(crate) fn make_warrior_skill_list(player_id: PlayerIdType) -> SkillTable<AttackSkill> {
    let db = WarriorDatabase::new(player_id);

    let warrior_skill_list: Vec<AttackSkill> = vec![
        db.heavy_swing,
        db.maim,
        db.storms_eye,
        db.infuriate,
        db.fell_cleave,
        db.onslaught,
        db.upheaval,
        db.inner_release,
        db.primal_rend,
        db.storms_path,
        db.inner_chaos,
        db.fell_cleave_inner,
        db.primal_wrath,
        db.primal_ruination,
        db.potion,
    ];

    make_skill_table(warrior_skill_list)
}
