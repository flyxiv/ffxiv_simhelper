use crate::id_entity::IdEntity;
use crate::owner_tracker::OwnerTracker;
use crate::rotation::cooldown_timer::CooldownTimer;
use crate::skill::{ResourceRequirements, Skill, NON_GCD_DELAY_MILLISECOND};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{DamageType, IdType, ResourceType, StackType, TimeType};
use ffxiv_simbot_db::MultiplierType;
use std::cmp::max;

#[derive(Clone)]
pub struct AttackSkill {
    pub(crate) id: IdType,
    pub(crate) name: String,
    pub(crate) player_id: IdType,
    pub(crate) potency: DamageType,
    pub(crate) trait_multiplier: MultiplierType,

    pub buff: Option<BuffStatus>,
    pub debuff: Option<DebuffStatus>,
    pub combo: Option<IdType>,

    pub(crate) delay_millisecond: Option<TimeType>,

    pub(crate) casting_time_millisecond: TimeType,
    pub(crate) gcd_cooldown_millisecond: TimeType,
    pub(crate) charging_time_millisecond: TimeType,
    pub(crate) is_speed_buffed: bool,

    pub(crate) resource_required: Vec<ResourceRequirements>,
    pub(crate) resource1_created: ResourceType,
    pub(crate) resource2_created: ResourceType,

    pub(crate) cooldown_millisecond: TimeType,
    pub(crate) current_cooldown_millisecond: TimeType,
    pub(crate) stacks: StackType,
    pub(crate) stack_skill_id: Option<IdType>,
}

#[derive(Clone)]
pub struct SkillInfo<S: Skill> {
    pub skill: S,
    pub damage_inflict_time_millisecond: Option<TimeType>,
    pub guaranteed_critical_hit: bool,
    pub guaranteed_direct_hit: bool,
}

impl SkillInfo<AttackSkill> {
    pub fn is_auto_attack(&self) -> bool {
        self.skill.is_auto_attack()
    }
}

impl IdEntity for AttackSkill {
    fn get_id(&self) -> IdType {
        self.id
    }
}

impl Skill for AttackSkill {
    fn get_potency(&self) -> DamageType {
        (self.potency as MultiplierType * self.trait_multiplier) as DamageType
    }

    fn get_cooldown_millisecond(&self) -> TimeType {
        self.cooldown_millisecond
    }
    fn get_charging_time_millisecond(&self) -> TimeType {
        self.charging_time_millisecond
    }

    /// All FFXIV Offensive Skills can be double-weaved except for Stardiver, so
    /// just give a default of 700ms, which is right for almost all skills.
    fn get_delay_millisecond(&self) -> TimeType {
        if let Some(delay) = self.delay_millisecond {
            delay
        } else {
            NON_GCD_DELAY_MILLISECOND
        }
    }

    #[inline]
    fn is_gcd(&self) -> bool {
        self.gcd_cooldown_millisecond > 0
    }

    fn get_gcd_cast_time(&self) -> TimeType {
        self.casting_time_millisecond
    }
    fn get_gcd_time_millisecond(&self) -> TimeType {
        self.gcd_cooldown_millisecond
    }
    fn get_current_cooldown_millisecond(&self) -> TimeType {
        self.current_cooldown_millisecond
    }
    fn get_gcd_cooldown_millsecond(&self) -> TimeType {
        max(self.gcd_cooldown_millisecond, self.casting_time_millisecond)
    }

    fn start_cooldown(&mut self) {
        self.stacks -= 1;
        self.current_cooldown_millisecond += self.cooldown_millisecond;
    }

    fn is_ready(&self) -> bool {
        self.stacks >= 1
    }
    fn is_raidbuff(&self) -> bool {
        if let Some(buff) = &self.buff {
            return buff.is_raidwide;
        }

        if let Some(debuff) = &self.debuff {
            return debuff.is_raidwide;
        }

        false
    }

    fn is_speed_buffed(&self) -> bool {
        self.is_speed_buffed
    }

    fn stack_skill_id(&self) -> IdType {
        if let Some(skill_id) = self.stack_skill_id {
            skill_id
        } else {
            self.id
        }
    }

    fn is_auto_attack(&self) -> bool {
        self.id == 0
    }

    fn get_resource1_created(&self) -> ResourceType {
        self.resource1_created
    }

    fn get_resource2_created(&self) -> ResourceType {
        self.resource2_created
    }

    fn get_combo(&self) -> Option<IdType> {
        self.combo
    }

    fn get_resource_required(&self) -> &Vec<ResourceRequirements> {
        &self.resource_required
    }
}

impl AttackSkill {
    #[inline]
    fn is_speed_buffed(&self) -> bool {
        self.is_speed_buffed
    }
}

impl OwnerTracker for AttackSkill {
    fn get_owner_id(&self) -> IdType {
        self.player_id
    }
}

impl CooldownTimer for AttackSkill {
    fn update_cooldown(&mut self, elapsed_time: TimeType) {
        let past_stack = self.get_stack();
        self.current_cooldown_millisecond -= elapsed_time;

        let current_stack = self.get_stack();

        if past_stack != current_stack {
            self.stacks += 1;
        }
    }
}

impl AttackSkill {
    fn get_stack(&self) -> StackType {
        f64::ceil(self.current_cooldown_millisecond as f64 / self.cooldown_millisecond as f64)
            as StackType
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skill::status::StatusInfo;

    #[test]
    fn attack_skill_test() {
        let skill: AttackSkill = AttackSkill {
            name: "Trick Attack".to_string(),
            player_id: 0,
            potency: 500,
            buff: Some(BuffStatus {
                id: 1,
                duration_left_millisecond: 15000,
                status_data: StatusInfo::DamagePercent(10),
                duration_millisecond: 15000,
                is_raidwide: false,
                cumulative_damage: None,
                owner_player_id: 0,
                status_info: (),
            }),
            debuff: None,
            is_gcd: true,
            delay_millisecond: None,
            is_modified: false,
            cooldown_millisecond: 2500,
            resource_required: vec![ResourceRequirements::Mana(100)],
        };

        assert_eq!(skill.get_potency(), 500);
        assert_eq!(skill.get_delay_millisecond(), NON_GCD_DELAY_MILLISECOND);
        assert_eq!(skill.is_gcd(), true);
        assert_eq!(skill.buff.is_some(), true);
        assert_eq!(skill.debuff.is_none(), true);
    }
}
