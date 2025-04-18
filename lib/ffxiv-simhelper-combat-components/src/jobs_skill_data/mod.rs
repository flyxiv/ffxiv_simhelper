use crate::event::ffxiv_event::FfxivEvent::ApplyBuff;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::use_type::UseType;
use crate::status::buff_status::BuffStatus;
use crate::status::status_info::StatusInfo;
use crate::types::PlayerIdType;

pub(crate) mod astrologian;
pub(crate) mod bard;
pub(crate) mod black_mage;
pub(crate) mod dancer;
pub(crate) mod darkknight;
pub(crate) mod dragoon;
pub(crate) mod gunbreaker;
pub(crate) mod machinist;
pub(crate) mod monk;
pub mod ninja;
pub(crate) mod paladin;
pub(crate) mod pictomancer;
pub(crate) mod reaper;
pub(crate) mod redmage;
pub(crate) mod sage;
pub(crate) mod samurai;
pub(crate) mod scholar;
pub(crate) mod summoner;
pub(crate) mod viper;
pub(crate) mod warrior;
pub(crate) mod white_mage;

struct PotionSkill {
    potion: AttackSkill,

    #[allow(unused)]
    potion_buff: BuffStatus,
}

struct CasterGlobalSkill {
    swiftcast: AttackSkill,
    swiftcast_buff: BuffStatus,
}

impl PotionSkill {
    pub fn new(player_id: PlayerIdType) -> Self {
        let potion_buff = BuffStatus {
            id: 1,
            owner_id: player_id,
            name: String::from("Potion Buff"),
            duration_left_millisecond: 0,
            status_info: vec![StatusInfo::IncreaseMainStat(461, 10)],
            duration_millisecond: 31000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            trigger_proc_event_on_gcd: vec![],
        };

        let potion = AttackSkill {
            id: 1,
            name: String::from("Potion"),
            player_id,
            potency: 0,
            trait_percent: 100,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                potion_buff.clone(),
                30000,
                30000,
                700,
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
            cooldown_millisecond: 270000,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::NoTarget,
        };

        Self {
            potion,
            potion_buff,
        }
    }
}

impl CasterGlobalSkill {
    pub fn new(player_id: PlayerIdType) -> Self {
        let swiftcast_buff: BuffStatus = BuffStatus {
            id: 0,
            owner_id: player_id,
            name: String::from("Swiftcast"),
            duration_left_millisecond: 0,
            status_info: vec![],
            duration_millisecond: 10000,
            is_raidwide: false,
            stacks: 1,
            max_stacks: 1,
            trigger_proc_event_on_gcd: vec![],
        };

        let swiftcast = AttackSkill {
            id: 0,
            name: String::from("Swiftcast"),
            player_id,
            potency: 0,
            trait_percent: 100,
            additional_skill_events: vec![ApplyBuff(
                player_id,
                player_id,
                swiftcast_buff.clone(),
                10000,
                10000,
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
            cooldown_millisecond: 40000,
            current_cooldown_millisecond: 0,
            stacks: 1,
            max_stacks: 1,
            stack_skill_id: None,
            use_type: UseType::NoTarget,
        };

        Self {
            swiftcast,
            swiftcast_buff,
        }
    }
}
