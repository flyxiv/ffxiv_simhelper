use crate::event::ffxiv_event::FfxivEvent;
use crate::jobs_skill_data::PotionSkill;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::damage_category::DamageCategory;
use crate::skill::use_type::UseType;
use crate::skill::{make_skill_table, ResourceTable};
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_info::StatusInfo;
use crate::types::PlayerIdType;

pub(crate) struct SageDatabase {
    pub(crate) dot: AttackSkill,
    pub(crate) gcd: AttackSkill,
    pub(crate) phlegma: AttackSkill,
    pub(crate) psyche: AttackSkill,

    pub(crate) potion: AttackSkill,
}

impl SageDatabase {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        let eukrasian_dosis_iii_dot: DebuffStatus = DebuffStatus {
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
            snapshotted_infos: Default::default(),
        };
        let eukrasian_dosis_iii: AttackSkill = AttackSkill {
            id: 700,
            name: String::from("Eukrasian Dosis III"),
            player_id,
            potency: 0,
            use_type: UseType::UseOnTarget,
            trait_percent: 130,
            additional_skill_events: vec![FfxivEvent::ApplyDebuff(
                player_id,
                eukrasian_dosis_iii_dot.clone(),
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
            cooldown_reduced_by_speed: true,
            cooldown_millisecond: 0,
            resource_required: vec![],
            resource_created: ResourceTable::new(),
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_direct_hit: false,
            is_guaranteed_crit: false,
        };
        let dosis_iii: AttackSkill = AttackSkill {
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
            cooldown_reduced_by_speed: true,
            cooldown_millisecond: 0,
            resource_required: vec![],
            resource_created: ResourceTable::new(),
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
        };
        let phlegma: AttackSkill = AttackSkill {
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
            is_speed_buffed: true,
            cooldown_reduced_by_speed: false,
            cooldown_millisecond: 40000,
            resource_required: vec![],
            resource_created: ResourceTable::new(),
            current_cooldown_millisecond: 0,
            stacks: 2,
            max_stacks: 2,
            stack_skill_id: None,
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
        };
        let psyche: AttackSkill = AttackSkill {
            id: 703,
            name: String::from("Psyche"),
            player_id,
            use_type: UseType::UseOnTarget,
            potency: 600,
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
            cooldown_millisecond: 60000,
            resource_required: vec![],
            resource_created: ResourceTable::new(),
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
        };

        let potion_skill = PotionSkill::new(player_id);

        Self {
            dot: eukrasian_dosis_iii,
            gcd: dosis_iii,
            phlegma,
            psyche,

            potion: potion_skill.potion,
        }
    }
}

pub(crate) fn make_sage_skills(player_id: PlayerIdType) -> SkillTable<AttackSkill> {
    let table = SageDatabase::new(player_id);

    let skills = vec![
        table.dot.clone(),
        table.gcd.clone(),
        table.phlegma.clone(),
        table.psyche.clone(),
        table.potion,
    ];
    make_skill_table(skills)
}
