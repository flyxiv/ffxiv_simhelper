use crate::skill::attack_skill::AttackSkill;
use crate::skill::use_type::UseType;
use crate::status::buff_status::BuffStatus;
use crate::IdType;

pub(crate) mod bard;
pub(crate) mod black_mage;
pub(crate) mod dancer;
pub(crate) mod dragoon;
pub(crate) mod monk;
pub mod ninja;
pub(crate) mod paladin;
pub(crate) mod sage;
pub(crate) mod warrior;
pub(crate) mod white_mage;

struct CasterGlobalSkill {
    swiftcast: AttackSkill,
    swiftcast_buff: BuffStatus,
}

impl CasterGlobalSkill {
    pub fn new(player_id: IdType) -> Self {
        let SWIFTCAST = AttackSkill {
            id: 0,
            name: String::from("Swiftcast"),
            player_id,
            potency: 0,
            trait_multiplier: 0.0,
            additional_skill_events: vec![],
            proc_events: vec![],
            combo: None,
            delay_millisecond: None,
            casting_time_millisecond: 0,
            gcd_cooldown_millisecond: 0,
            charging_time_millisecond: 0,
            is_speed_buffed: false,
            resource_required: vec![],
            resource_created: Default::default(),
            is_guaranteed_crit: false,
            is_guaranteed_direct_hit: false,
            cooldown_millisecond: 60000,
            current_cooldown_millisecond: 0,
            stacks: 1,
            stack_skill_id: None,
            use_type: UseType::NoTarget,
        };

        let SWIFTCAST_BUFF: BuffStatus = BuffStatus {
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

        Self {
            swiftcast: SWIFTCAST,
            swiftcast_buff: SWIFTCAST_BUFF,
        }
    }
}
