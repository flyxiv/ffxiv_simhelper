use crate::event::ffxiv_event::FfxivEvent;
use crate::event::ffxiv_event::FfxivEvent::{ApplyBuff, ApplyRaidBuff};
use crate::id_entity::IdEntity;
use crate::jobs_skill_data::PotionSkill;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::damage_category::DamageCategory;
use crate::skill::use_type::UseType;
use crate::skill::ResourceRequirements::{Resource, UseBuff};
use crate::skill::{make_skill_table, ResourceRequirements};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_info::StatusInfo;
use crate::types::PlayerIdType;
use std::collections::HashMap;

pub(crate) struct DragoonDatabase {
    pub(crate) life_surge: AttackSkill,
    pub(crate) true_thrust: AttackSkill,
    pub(crate) raiden_thrust: AttackSkill,
    pub(crate) lance_barrage: AttackSkill,
    pub(crate) spiral_blow: AttackSkill,
    pub(crate) heavens_thrust: AttackSkill,
    pub(crate) chaotic_spring: AttackSkill,
    pub(crate) fang_and_claw: AttackSkill,
    pub(crate) wheeling_thrust: AttackSkill,
    pub(crate) drakesbane: AttackSkill,
    pub(crate) rise_of_the_dragon: AttackSkill,
    pub(crate) lance_charge: AttackSkill,
    pub(crate) heavens_thrust_surge: AttackSkill,
    pub(crate) drakesbane_surge: AttackSkill,
    pub(crate) starcross: AttackSkill,
    pub(crate) high_jump: AttackSkill,
    pub(crate) mirage_dive: AttackSkill,
    pub(crate) dragonfire_dive: AttackSkill,
    pub(crate) battle_litany: AttackSkill,
    pub(crate) geirskogul: AttackSkill,
    pub(crate) nastrond: AttackSkill,
    pub(crate) stardiver: AttackSkill,
    pub(crate) wyrmwind_thrust: AttackSkill,

    pub(crate) life_surge_buff: BuffStatus,
    pub(crate) draconian_fire: BuffStatus,
    pub(crate) lance_charge_buff: BuffStatus,
    pub(crate) battle_litany_buff: BuffStatus,

    pub(crate) potion: AttackSkill,
}

impl DragoonDatabase {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        let life_surge_status: BuffStatus = BuffStatus {
            id: 800,
            name: String::from("Life Surge"),
            stacks: 1,
            max_stacks: 1,
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::None],
            duration_millisecond: 5000,
            is_raidwide: false,
            trigger_proc_event_on_gcd: vec![],
        };
        let power_surge: BuffStatus = BuffStatus {
            id: 801,
            name: String::from("Power Surge"),
            stacks: 1,
            max_stacks: 1,
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::DamagePercent(10)],
            duration_millisecond: 30000,
            is_raidwide: false,
            trigger_proc_event_on_gcd: vec![],
        };
        let dragons_flight: BuffStatus = BuffStatus {
            id: 802,
            name: String::from("Dragon's Flight"),
            stacks: 1,
            max_stacks: 1,
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::None],
            duration_millisecond: 30000,
            is_raidwide: false,
            trigger_proc_event_on_gcd: vec![],
        };
        let draconian_fire: BuffStatus = BuffStatus {
            id: 803,
            name: String::from("Draconian Fire"),
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::None],
            duration_millisecond: 30000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            trigger_proc_event_on_gcd: vec![],
        };
        let lance_charge_status: BuffStatus = BuffStatus {
            id: 804,
            name: String::from("Lance Charge"),
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::DamagePercent(10)],
            duration_millisecond: 20000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            trigger_proc_event_on_gcd: vec![],
        };
        let dive_ready: BuffStatus = BuffStatus {
            id: 805,
            name: String::from("Dive Ready"),
            stacks: 1,
            max_stacks: 1,
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::None],
            duration_millisecond: 15000,
            is_raidwide: false,
            trigger_proc_event_on_gcd: vec![],
        };
        let starcross_ready: BuffStatus = BuffStatus {
            id: 806,
            name: String::from("Starcross Ready"),
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![],
            duration_millisecond: 20000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            trigger_proc_event_on_gcd: vec![],
        };
        let battle_litany_status: BuffStatus = BuffStatus {
            id: 807,
            name: String::from("Battle Litany"),
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::CritHitRatePercent(10)],
            duration_millisecond: 21000,
            is_raidwide: true,
            stacks: 1,
            max_stacks: 1,
            trigger_proc_event_on_gcd: vec![],
        };
        let life_of_the_dragon: BuffStatus = BuffStatus {
            id: 808,
            name: String::from("Life of the Dragon"),
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::DamagePercent(15)],
            duration_millisecond: 20000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            trigger_proc_event_on_gcd: vec![],
        };

        let chaotic_spring_dot: DebuffStatus = DebuffStatus {
            id: 810,
            name: String::from("Chaotic Spring"),
            owner_id: player_id,
            damage_skill_id: Some(826),
            trait_percent: Some(100),
            potency: Some(45),
            damage_category: Some(DamageCategory::PhysicalDot),
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::None],
            duration_millisecond: 24000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            snapshotted_infos: HashMap::new(),
        };

        let life_surge: AttackSkill = AttackSkill {
            id: 800,
            name: String::from("Life Surge"),
            player_id,
            potency: 0,
            trait_percent: 100,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                life_surge_status.clone(),
                5000,
                5000,
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
            cooldown_millisecond: 40000,
            resource_required: vec![],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 2,
            max_stacks: 2,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::NoTarget,
        };
        let true_thrust: AttackSkill = AttackSkill {
            id: 801,
            name: String::from("True Thrust"),
            player_id,
            potency: 230,
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
            use_type: UseType::UseOnTarget,
        };
        let raiden_thrust: AttackSkill = AttackSkill {
            id: 802,
            name: String::from("Raiden Thrust"),
            player_id,
            potency: 320,
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
            resource_required: vec![UseBuff(draconian_fire.id)],
            resource_created: HashMap::from([(1, 1)]),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::UseOnTarget,
        };
        let lance_barrage: AttackSkill = AttackSkill {
            id: 803,
            name: String::from("Lance Barrage"),
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
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::UseOnTarget,
        };
        let spiral_blow: AttackSkill = AttackSkill {
            id: 804,
            name: String::from("Spiral Blow"),
            player_id,
            potency: 300,
            trait_percent: 100,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                power_surge.clone(),
                30000,
                30000,
                0,
            )],
            proc_events: vec![],
            combo: Some(4),
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
            use_type: UseType::UseOnTarget,
        };
        let heavens_thrust: AttackSkill = AttackSkill {
            id: 805,
            name: String::from("Heaven's Thrust"),
            player_id,
            potency: 460,
            trait_percent: 100,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: Some(6),
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
            use_type: UseType::UseOnTarget,
        };
        let chaotic_spring: AttackSkill = AttackSkill {
            id: 806,
            name: String::from("Chaotic Spring"),
            player_id,
            potency: 340,
            trait_percent: 100,
            additional_skill_events: vec![FfxivEvent::ApplyDebuff(
                player_id,
                chaotic_spring_dot.clone(),
                24000,
                24000,
                0,
            )],
            proc_events: vec![],
            combo: Some(5),
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
            use_type: UseType::UseOnTarget,
        };
        let fang_and_claw: AttackSkill = AttackSkill {
            id: 807,
            name: String::from("Fang and Claw"),
            player_id,
            potency: 340,
            trait_percent: 100,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: Some(7),
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
            use_type: UseType::UseOnTarget,
        };
        let wheeling_thrust: AttackSkill = AttackSkill {
            id: 808,
            name: String::from("Wheeling Thrust"),
            player_id,
            potency: 340,
            trait_percent: 100,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: Some(7),
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
            use_type: UseType::UseOnTarget,
        };
        let drakesbane: AttackSkill = AttackSkill {
            id: 809,
            name: String::from("Drakesbane"),
            player_id,
            potency: 460,
            trait_percent: 100,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                draconian_fire.clone(),
                30000,
                30000,
                0,
            )],
            proc_events: vec![],
            combo: Some(1),
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
            use_type: UseType::UseOnTarget,
        };
        let rise_of_the_dragon: AttackSkill = AttackSkill {
            id: 810,
            name: String::from("Rise of the Dragon"),
            player_id,
            potency: 550,
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
            cooldown_millisecond: 0,
            resource_required: vec![UseBuff(dragons_flight.id)],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::NoTarget,
        };
        let lance_charge: AttackSkill = AttackSkill {
            id: 811,
            name: String::from("Lance Charge"),
            player_id,
            potency: 0,
            trait_percent: 100,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                lance_charge_status.clone(),
                20000,
                20000,
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
        let heavens_thrust_surge: AttackSkill = AttackSkill {
            id: 812,
            name: String::from("Heaven's Thrust"),
            player_id,
            potency: 460,
            trait_percent: 100,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: Some(6),
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            cooldown_millisecond: 0,
            resource_required: vec![UseBuff(life_surge.id)],
            resource_created: Default::default(),
            is_guaranteed_crit: true,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::UseOnTarget,
        };
        let drakesbane_surge: AttackSkill = AttackSkill {
            id: 813,
            name: String::from("Drakesbane"),
            player_id,
            potency: 440,
            trait_percent: 100,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                draconian_fire.clone(),
                30000,
                30000,
                0,
            )],
            proc_events: vec![],
            combo: Some(1),
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            cooldown_millisecond: 0,
            resource_required: vec![UseBuff(life_surge.id)],
            resource_created: Default::default(),
            is_guaranteed_crit: true,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::UseOnTarget,
        };
        let starcross: AttackSkill = AttackSkill {
            id: 814,
            name: String::from("Starcross"),
            player_id,
            potency: 1000,
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
            cooldown_millisecond: 0,
            resource_required: vec![UseBuff(starcross_ready.id)],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::UseOnTarget,
        };
        let high_jump: AttackSkill = AttackSkill {
            id: 815,
            name: String::from("High Jump"),
            player_id,
            potency: 400,
            trait_percent: 100,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                dive_ready.clone(),
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
        let mirage_dive: AttackSkill = AttackSkill {
            id: 816,
            name: String::from("Mirage Dive"),
            player_id,
            potency: 380,
            trait_percent: 100,
            additional_skill_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 0,
            charging_time_millisecond: 0,
            is_speed_buffed: false,
            cooldown_reduced_by_speed: false,
            cooldown_millisecond: 0,
            resource_required: vec![UseBuff(dive_ready.get_id())],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            proc_events: vec![],
            is_guaranteed_direct_hit: false,
            use_type: UseType::UseOnTarget,
        };
        let dragonfire_dive: AttackSkill = AttackSkill {
            id: 819,
            name: String::from("Dragonfire Dive"),
            player_id,
            potency: 500,
            trait_percent: 100,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                dragons_flight.clone(),
                30000,
                30000,
                0,
            )],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 0,
            charging_time_millisecond: 0,
            is_speed_buffed: false,
            cooldown_reduced_by_speed: false,
            cooldown_millisecond: 120000,
            resource_required: vec![],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            proc_events: vec![],
            is_guaranteed_direct_hit: false,
            use_type: UseType::UseOnTarget,
        };
        let battle_litany: AttackSkill = AttackSkill {
            id: 820,
            name: String::from("Battle Litany"),
            player_id,
            potency: 0,
            trait_percent: 100,
            additional_skill_events: vec![ApplyRaidBuff(
                player_id,
                battle_litany_status.clone(),
                21000,
                21000,
                1000,
            )],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 0,
            charging_time_millisecond: 0,
            is_speed_buffed: false,
            cooldown_reduced_by_speed: false,
            cooldown_millisecond: 120000,
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
        let geirskogul: AttackSkill = AttackSkill {
            id: 821,
            name: String::from("Geirskogul"),
            player_id,
            potency: 280,
            trait_percent: 100,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                life_of_the_dragon.clone(),
                20000,
                20000,
                0,
            )],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 0,
            charging_time_millisecond: 0,
            is_speed_buffed: false,
            cooldown_reduced_by_speed: false,
            cooldown_millisecond: 60000,
            resource_required: vec![],
            resource_created: HashMap::from([(0, 1)]),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            proc_events: vec![],
            is_guaranteed_direct_hit: false,
            use_type: UseType::UseOnTarget,
        };
        let nastrond: AttackSkill = AttackSkill {
            id: 823,
            name: String::from("Nastrond"),
            player_id,
            potency: 720,
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
            cooldown_millisecond: 2000,
            resource_required: vec![Resource(0, 1)],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::UseOnTarget,
        };
        let stardiver: AttackSkill = AttackSkill {
            id: 824,
            name: String::from("Stardiver"),
            player_id,
            potency: 840,
            trait_percent: 100,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                starcross_ready.clone(),
                20000,
                20000,
                0,
            )],
            proc_events: vec![],
            combo: None,
            delay_millisecond: Some(1500),
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 0,
            charging_time_millisecond: 0,
            is_speed_buffed: false,
            cooldown_reduced_by_speed: false,
            cooldown_millisecond: 30000,
            resource_required: vec![ResourceRequirements::CheckStatus(life_of_the_dragon.id)],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::UseOnTarget,
        };
        let wyrmwind_thrust: AttackSkill = AttackSkill {
            id: 825,
            name: String::from("Wyrmwind Thrust"),
            player_id,
            potency: 440,
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
            cooldown_millisecond: 20000,
            resource_required: vec![Resource(1, 2)],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            use_type: UseType::UseOnTarget,
        };

        let potion_skill = PotionSkill::new(player_id);

        DragoonDatabase {
            life_surge,
            true_thrust,
            raiden_thrust,
            lance_barrage,
            spiral_blow,
            heavens_thrust,
            chaotic_spring,
            fang_and_claw,
            wheeling_thrust,
            drakesbane,
            rise_of_the_dragon,
            lance_charge,
            heavens_thrust_surge,
            drakesbane_surge,
            starcross,
            high_jump,
            mirage_dive,
            dragonfire_dive,
            battle_litany,
            geirskogul,
            nastrond,
            stardiver,
            wyrmwind_thrust,

            life_surge_buff: life_surge_status,
            draconian_fire,
            lance_charge_buff: lance_charge_status,
            battle_litany_buff: battle_litany_status,

            potion: potion_skill.potion,
        }
    }
}

pub(crate) fn make_dragoon_skill_list(player_id: PlayerIdType) -> SkillTable<AttackSkill> {
    let db = DragoonDatabase::new(player_id);

    let dragoon_skill_list: Vec<AttackSkill> = vec![
        db.life_surge,
        db.true_thrust,
        db.raiden_thrust,
        db.lance_barrage,
        db.spiral_blow,
        db.heavens_thrust,
        db.chaotic_spring,
        db.fang_and_claw,
        db.wheeling_thrust,
        db.drakesbane,
        db.rise_of_the_dragon,
        db.lance_charge,
        db.heavens_thrust_surge,
        db.drakesbane_surge,
        db.high_jump,
        db.mirage_dive,
        db.dragonfire_dive,
        db.battle_litany,
        db.geirskogul,
        db.nastrond,
        db.stardiver,
        db.wyrmwind_thrust,
        db.starcross,
        db.potion,
    ];

    make_skill_table(dragoon_skill_list)
}
