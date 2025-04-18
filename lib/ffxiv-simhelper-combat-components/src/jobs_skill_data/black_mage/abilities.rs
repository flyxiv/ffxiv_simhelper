use crate::event::ffxiv_event::FfxivEvent;
use crate::event::ffxiv_event::FfxivEvent::{ApplyBuff, RefreshBuff};
use crate::id_entity::IdEntity;
use crate::jobs_skill_data::{CasterGlobalSkill, PotionSkill};
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::damage_category::DamageCategory::MagicalDot;
use crate::skill::make_skill_table;
use crate::skill::use_type::UseType;
use crate::skill::ResourceRequirements::{Resource, UseBuff};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_info::StatusInfo;
use crate::types::PlayerIdType;
use std::collections::HashMap;

pub(crate) struct BlackmageDatabase {
    pub(crate) transpose_ice_to_fire: AttackSkill,
    pub(crate) high_thunder: AttackSkill,
    pub(crate) transpose_fire_to_ice: AttackSkill,
    pub(crate) fire4: AttackSkill,
    pub(crate) fire4_triplecast: AttackSkill,
    pub(crate) fire3_ice: AttackSkill,
    pub(crate) fire3_f1: AttackSkill,
    pub(crate) despair: AttackSkill,
    pub(crate) xenoglossy: AttackSkill,
    pub(crate) paradox: AttackSkill,
    pub(crate) blizzard3: AttackSkill,
    pub(crate) blizzard4: AttackSkill,
    pub(crate) triplecast: AttackSkill,
    pub(crate) leylines: AttackSkill,
    pub(crate) manafont: AttackSkill,
    pub(crate) amplifier: AttackSkill,
    pub(crate) fire3_opener: AttackSkill,
    pub(crate) fire4_swiftcast: AttackSkill,
    pub(crate) flare_star: AttackSkill,
    pub(crate) swiftcast: AttackSkill,
    pub(crate) blizzard3_transpose_swift: AttackSkill,
    pub(crate) flare_star_triplecast: AttackSkill,
    pub(crate) blizzard3_transpose_triplecast: AttackSkill,
    pub(crate) fire_iii_proc: AttackSkill,

    pub(crate) triplecast_buff: BuffStatus,
    pub(crate) high_thunder_dot: DebuffStatus,
    pub(crate) astral_fire3: BuffStatus,
    pub(crate) umbral_ice1: BuffStatus,
    pub(crate) astral_fire1: BuffStatus,
    pub(crate) leylines_buff: BuffStatus,

    pub(crate) potion: AttackSkill,
}
impl BlackmageDatabase {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        let caster_skills = CasterGlobalSkill::new(player_id);

        let triplecast_buff: BuffStatus = {
            BuffStatus {
                id: 1700,
                name: String::from("Triplecast"),
                owner_id: player_id,
                duration_left_millisecond: 0,
                status_info: vec![StatusInfo::None],
                duration_millisecond: 15000,
                is_raidwide: false,
                stacks: 3,
                max_stacks: 3,
                trigger_proc_event_on_gcd: vec![],
            }
        };
        let high_thunder_dot: DebuffStatus = DebuffStatus {
            id: 1701,
            name: String::from("High Thunder"),
            owner_id: player_id,
            potency: Some(60),
            trait_percent: Some(130),
            damage_category: Some(MagicalDot),
            damage_skill_id: Some(1725),
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::None],
            duration_millisecond: 30000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            snapshotted_infos: Default::default(),
        };
        let thunderhead: BuffStatus = BuffStatus {
            id: 1702,
            name: String::from("Thunderhead"),
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![],
            duration_millisecond: 120000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            trigger_proc_event_on_gcd: vec![],
        };
        let leylines_buff: BuffStatus = BuffStatus {
            id: 1703,
            name: String::from("Ley Lines"),
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::SpeedPercent(15)],
            duration_millisecond: 20000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            trigger_proc_event_on_gcd: vec![],
        };
        let astral_fire_i: BuffStatus = BuffStatus {
            id: 1704,
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![],
            duration_millisecond: 15000,
            is_raidwide: false,
            name: "Astral Fire I".to_string(),
            stacks: 1,
            max_stacks: 1,
            trigger_proc_event_on_gcd: vec![],
        };
        let umbral_ice_i: BuffStatus = BuffStatus {
            id: 1705,
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![],
            duration_millisecond: 15000,
            is_raidwide: false,
            name: "Umbral Ice 1".to_string(),
            stacks: 1,
            max_stacks: 1,
            trigger_proc_event_on_gcd: vec![],
        };
        let astral_fire_iii: BuffStatus = BuffStatus {
            id: 1706,
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![],
            duration_millisecond: 60000,
            is_raidwide: false,
            name: "Astral Fire III".to_string(),
            stacks: 1,
            max_stacks: 1,
            trigger_proc_event_on_gcd: vec![],
        };
        let firestarter: BuffStatus = BuffStatus {
            id: 1708,
            owner_id: player_id,
            duration_left_millisecond: 0,
            status_info: vec![],
            duration_millisecond: 120000,
            is_raidwide: false,
            name: "Firestarter".to_string(),
            stacks: 1,
            max_stacks: 1,
            trigger_proc_event_on_gcd: vec![],
        };

        let transpose_ice_to_fire: AttackSkill = AttackSkill {
            id: 1700,
            name: "Transpose".to_string(),
            player_id,
            potency: 0,
            trait_percent: 130,
            additional_skill_events: vec![
                ApplyBuff(player_id, player_id, astral_fire_i.clone(), 15000, 15000, 0),
                ApplyBuff(player_id, player_id, thunderhead.clone(), 120000, 120000, 0),
            ],
            proc_events: vec![],
            combo: Some(0),
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 0,
            charging_time_millisecond: 0,
            is_speed_buffed: false,
            cooldown_reduced_by_speed: false,
            resource_required: vec![],
            resource_created: HashMap::from([(1, 1)]),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 5000,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::NoTarget,
        };

        let high_thunder: AttackSkill = AttackSkill {
            id: 1701,
            name: "High Thunder".to_string(),
            player_id,
            potency: 150,
            trait_percent: 130,
            additional_skill_events: vec![FfxivEvent::ApplyDebuff(
                player_id,
                high_thunder_dot.clone(),
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
            resource_required: vec![UseBuff(thunderhead.get_id())],
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

        let transpose_fire_to_ice: AttackSkill = AttackSkill {
            id: 1702,
            name: "Transpose".to_string(),
            player_id,
            potency: 0,
            trait_percent: 130,
            additional_skill_events: vec![
                ApplyBuff(player_id, player_id, umbral_ice_i.clone(), 15000, 15000, 0),
                ApplyBuff(player_id, player_id, thunderhead.clone(), 120000, 120000, 0),
            ],
            proc_events: vec![],
            combo: Some(1),
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 0,
            charging_time_millisecond: 0,
            is_speed_buffed: false,
            cooldown_reduced_by_speed: false,
            resource_required: vec![UseBuff(astral_fire_iii.get_id()), Resource(4, 1)],
            resource_created: HashMap::from([(1, 1)]),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 5000,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::NoTarget,
        };
        let fire_iv: AttackSkill = AttackSkill {
            id: 1703,
            name: "Fire IV".to_string(),
            player_id,
            potency: 300,
            trait_percent: 234,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 2000,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            resource_required: vec![],
            resource_created: HashMap::from([(2, 1)]),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::UseOnTarget,
        };

        let fire_iv_triplecast: AttackSkill = AttackSkill {
            id: 1704,
            name: "Fire IV".to_string(),
            player_id,
            potency: 310,
            trait_percent: 234,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            resource_required: vec![UseBuff(triplecast_buff.get_id())],
            resource_created: HashMap::from([(2, 1)]),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: Some(fire_iv.get_id()),
            use_type: UseType::UseOnTarget,
        };

        let fire_iii_ice: AttackSkill = AttackSkill {
            id: 1705,
            name: "Fire III Ice".to_string(),
            player_id,
            potency: 290,
            trait_percent: 91,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                astral_fire_iii.clone(),
                60000,
                60000,
                0,
            )],
            proc_events: vec![],
            combo: Some(0),
            delay_millisecond: None,
            casting_time_millisecond: 850,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            resource_required: vec![],
            resource_created: HashMap::from([(1, 1)]),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::UseOnTarget,
        };
        let fire_iii_astral_fire_i: AttackSkill = AttackSkill {
            id: 1706,
            name: "Fire III Astral Fire I".to_string(),
            player_id,
            potency: 290,
            trait_percent: 156,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                astral_fire_iii.clone(),
                60000,
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
            resource_required: vec![
                UseBuff(firestarter.get_id()),
                UseBuff(astral_fire_i.get_id()),
            ],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: Some(fire_iii_ice.get_id()),
            use_type: UseType::UseOnTarget,
        };

        let despair: AttackSkill = AttackSkill {
            id: 1707,
            name: "Despair".to_string(),
            player_id,
            potency: 350,
            trait_percent: 234,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                astral_fire_iii.clone(),
                60000,
                60000,
                0,
            )],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 2000,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            resource_required: vec![Resource(2, 6)],
            resource_created: HashMap::from([(3, 1)]),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::UseOnTarget,
        };
        let xenoglossy: AttackSkill = AttackSkill {
            id: 1710,
            name: "Xenoglossy".to_string(),
            player_id,
            potency: 900,
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
            resource_required: vec![Resource(0, 1)],
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

        let paradox: AttackSkill = AttackSkill {
            id: 1711,
            name: "Paradox".to_string(),
            player_id,
            potency: 520,
            trait_percent: 130,
            additional_skill_events: vec![
                RefreshBuff(
                    player_id,
                    player_id,
                    astral_fire_iii.clone(),
                    60000,
                    60000,
                    0,
                ),
                ApplyBuff(player_id, player_id, firestarter.clone(), 120000, 120000, 0),
            ],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            resource_required: vec![Resource(1, 1)],
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
        let blizzard_iii: AttackSkill = AttackSkill {
            id: 1712,
            name: "Blizzard III".to_string(),
            player_id,
            potency: 290,
            trait_percent: 91,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                thunderhead.clone(),
                120000,
                120000,
                0,
            )],
            proc_events: vec![],
            combo: Some(2),
            delay_millisecond: None,
            casting_time_millisecond: 850,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            resource_required: vec![Resource(4, 1), UseBuff(astral_fire_iii.get_id())],
            resource_created: HashMap::from([(1, 1)]),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::UseOnTarget,
        };
        let blizzard_iv: AttackSkill = AttackSkill {
            id: 1713,
            name: "Blizzard IV".to_string(),
            player_id,
            potency: 300,
            trait_percent: 130,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: Some(3),
            delay_millisecond: None,
            casting_time_millisecond: 2000,
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
        let triplecast: AttackSkill = AttackSkill {
            id: 1714,
            name: "Triplecast".to_string(),
            player_id,
            potency: 0,
            trait_percent: 130,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                triplecast_buff.clone(),
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
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 60000,
            current_cooldown_millisecond: 0,
            stacks: 2,
            max_stacks: 2,
            stack_skill_id: None,
            use_type: UseType::NoTarget,
        };

        let ley_lines: AttackSkill = AttackSkill {
            id: 1715,
            name: "Ley Lines".to_string(),
            player_id,
            potency: 0,
            trait_percent: 130,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                leylines_buff.clone(),
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
            resource_required: vec![],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 120000,
            current_cooldown_millisecond: 0,
            stacks: 2,
            max_stacks: 2,
            stack_skill_id: None,
            use_type: UseType::NoTarget,
        };

        let manafont: AttackSkill = AttackSkill {
            id: 1716,
            name: "Manafont".to_string(),
            player_id,
            potency: 0,
            trait_percent: 130,
            additional_skill_events: vec![
                ApplyBuff(player_id, player_id, thunderhead.clone(), 30000, 30000, 0),
                RefreshBuff(
                    player_id,
                    player_id,
                    astral_fire_iii.clone(),
                    15000,
                    15000,
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
            resource_required: vec![Resource(4, 1)],
            resource_created: HashMap::from([(1, 1)]),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 90000,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::NoTarget,
        };
        let amplifier: AttackSkill = AttackSkill {
            id: 1717,
            name: "Amplifier".to_string(),
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
            resource_created: HashMap::from([(0, 1)]),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 120000,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::NoTarget,
        };

        let fire_iii_opener: AttackSkill = AttackSkill {
            id: 1718,
            name: "Fire III".to_string(),
            player_id,
            potency: 290,
            trait_percent: 130,
            additional_skill_events: vec![
                ApplyBuff(
                    player_id,
                    player_id,
                    astral_fire_iii.clone(),
                    60000,
                    60000,
                    0,
                ),
                ApplyBuff(player_id, player_id, thunderhead.clone(), 120000, 120000, 0),
            ],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 3500,
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
            stack_skill_id: Some(fire_iii_ice.get_id()),
            use_type: UseType::UseOnTarget,
        };

        let fire_iv_swiftcast: AttackSkill = AttackSkill {
            id: 1719,
            name: "Fire IV".to_string(),
            player_id,
            potency: 300,
            trait_percent: 234,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            resource_required: vec![UseBuff(caster_skills.swiftcast.get_id())],
            resource_created: HashMap::from([(2, 1)]),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::UseOnTarget,
        };

        let flare_star: AttackSkill = AttackSkill {
            id: 1720,
            name: "Flare Star".to_string(),
            player_id,
            potency: 500,
            trait_percent: 234,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: Some(1),
            delay_millisecond: None,
            casting_time_millisecond: 3000,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            resource_required: vec![Resource(3, 1)],
            resource_created: HashMap::from([(4, 1)]),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::UseOnTarget,
        };
        let blizzard_iii_transpose_swift: AttackSkill = AttackSkill {
            id: 1721,
            name: "Blizzard III".to_string(),
            player_id,
            potency: 290,
            trait_percent: 130,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: Some(2),
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            resource_required: vec![
                UseBuff(umbral_ice_i.get_id()),
                UseBuff(caster_skills.swiftcast.get_id()),
            ],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: Some(blizzard_iii.get_id()),
            use_type: UseType::UseOnTarget,
        };

        let flare_star_triplecast: AttackSkill = AttackSkill {
            id: 1722,
            name: "Flare Star".to_string(),
            player_id,
            potency: 400,
            trait_percent: 234,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: Some(1),
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            resource_required: vec![UseBuff(triplecast_buff.get_id()), Resource(3, 1)],
            resource_created: HashMap::from([(4, 1)]),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: Some(flare_star.get_id()),
            use_type: UseType::UseOnTarget,
        };

        let blizzard_iii_transpose_triplecast: AttackSkill = AttackSkill {
            id: 1723,
            name: "Blizzard III".to_string(),
            player_id,
            potency: 290,
            trait_percent: 130,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: Some(2),
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 2500,
            charging_time_millisecond: 0,
            is_speed_buffed: true,
            cooldown_reduced_by_speed: true,
            resource_required: vec![
                UseBuff(umbral_ice_i.get_id()),
                UseBuff(triplecast_buff.get_id()),
            ],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: Some(blizzard_iii.get_id()),
            use_type: UseType::UseOnTarget,
        };
        let fire_iii_proc: AttackSkill = AttackSkill {
            id: 1724,
            name: "Fire III Proc".to_string(),
            player_id,
            potency: 290,
            trait_percent: 234,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                astral_fire_iii.clone(),
                60000,
                60000,
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
            resource_required: vec![UseBuff(firestarter.get_id())],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 0,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: Some(fire_iii_ice.get_id()),
            use_type: UseType::UseOnTarget,
        };

        let potion_skills = PotionSkill::new(player_id);

        BlackmageDatabase {
            transpose_ice_to_fire,
            high_thunder,
            transpose_fire_to_ice,
            fire4: fire_iv,
            fire4_triplecast: fire_iv_triplecast,
            fire3_ice: fire_iii_ice,
            fire3_f1: fire_iii_astral_fire_i,
            despair,
            xenoglossy,
            paradox,
            blizzard3: blizzard_iii,
            blizzard4: blizzard_iv,
            triplecast,
            leylines: ley_lines,
            manafont,
            fire3_opener: fire_iii_opener,
            amplifier,
            fire4_swiftcast: fire_iv_swiftcast,
            flare_star,
            blizzard3_transpose_swift: blizzard_iii_transpose_swift,
            flare_star_triplecast,
            blizzard3_transpose_triplecast: blizzard_iii_transpose_triplecast,
            fire_iii_proc,

            swiftcast: caster_skills.swiftcast,

            triplecast_buff,
            high_thunder_dot,
            astral_fire3: astral_fire_iii,
            umbral_ice1: umbral_ice_i,
            astral_fire1: astral_fire_i,
            leylines_buff,

            potion: potion_skills.potion,
        }
    }
}

pub(crate) fn make_blackmage_skill_list(player_id: PlayerIdType) -> SkillTable<AttackSkill> {
    let db = BlackmageDatabase::new(player_id);

    let blackmage_skill_list: Vec<AttackSkill> = vec![
        db.transpose_ice_to_fire,
        db.high_thunder,
        db.transpose_fire_to_ice,
        db.fire4,
        db.fire4_triplecast,
        db.fire3_ice,
        db.fire3_f1,
        db.despair,
        db.xenoglossy,
        db.manafont,
        db.paradox,
        db.blizzard3,
        db.blizzard4,
        db.triplecast,
        db.leylines,
        db.swiftcast,
        db.amplifier,
        db.fire3_opener,
        db.fire4_swiftcast,
        db.flare_star,
        db.blizzard3_transpose_swift,
        db.flare_star_triplecast,
        db.blizzard3_transpose_triplecast,
        db.fire_iii_proc,
        db.potion,
    ];

    make_skill_table(blackmage_skill_list)
}
