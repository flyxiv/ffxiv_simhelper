use crate::rotation::SkillPriorityInfo;
use crate::skill::attack_skill::AttackSkill;
use crate::status::buff_status::BuffStatus;
use crate::status::status_info::StatusInfo;
use crate::{ResourceType, TurnCount};
use lazy_static::lazy_static;

lazy_static! {
    static ref HUTON_STATUS: BuffStatus = {
        BuffStatus {
            id: 0,
            owner_id: 0,
            duration_left_millisecond: 0,
            status_info: StatusInfo::SpeedPercent(15),
            duration_millisecond: 60000,
            is_raidwide: false,
            owner_player_id: 0,
        }
    };
    static ref HUTON: AttackSkill = AttackSkill {
        id: 3,
        name: String::from("Huton"),
        player_id: 0,
        potency: 0,
        trait_multiplier: 1.0,
        buff: None,
        debuff: None,
        combo: None,
        turn_type: FfxivTurnType::Gcd,
        delay_millisecond: None,
        cooldown_millisecond: 0,
        resource_required: vec![],
        current_cooldown_millisecond: 0,
        stacks: 1,
    };
}

pub(crate) struct NinjaPriorityTable {
    turn_count: TurnCount,
    cooldowns: Vec<AttackSkill>,
    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,

    ninki: ResourceType,
}
