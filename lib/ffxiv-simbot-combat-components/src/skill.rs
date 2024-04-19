use crate::owner_tracker::OwnerTracker;
use crate::player::Player;
use crate::status::{BuffStatus, DebuffStatus, StatusHolder};
use crate::target::Target;
use crate::turn_type::FfxivTurnType;
use crate::CombatComponentsError;
use crate::{DamageType, IdType, Result, TimeType};

pub static GCD_TURN_DELAY_THRESHOLD: TimeType = 3 * NON_GCD_DELAY_MILLISECOND;
/// The normal delay time for o-GCD skills.
/// After using 1 oGCD, the player cannot use another skill for 0.7 seconds.
pub static NON_GCD_DELAY_MILLISECOND: i32 = 700;

/// The resource requirements for a skill.
/// Skill might need mana, status(suiton status is needed for Trick Attack), or combo status.
#[derive(Clone)]
pub(crate) enum ResourceRequirements {
    Mana(i32),
    Status(i32),
    PreviousCombo(IdType),
}

pub trait Skill: Sized + Clone {
    fn get_id(&self) -> IdType;
    fn get_potency(&self) -> DamageType;
    fn get_cooldown_millisecond(&self) -> TimeType;
    fn get_delay_millisecond(&self) -> TimeType;
    fn is_gcd(&self) -> bool;
    fn afflict_debuff<T: StatusHolder<DebuffStatus>>(&self, target: &mut T) -> Result<()>;
    fn afflict_buff<P: StatusHolder<BuffStatus>>(
        &self,
        player: &mut P,
        party: &mut Vec<P>,
    ) -> Result<()>;
}

#[derive(Clone)]
pub struct AttackSkill {
    pub(crate) id: IdType,
    pub(crate) name: String,
    pub(crate) player_id: IdType,
    pub(crate) potency: DamageType,
    pub buff: Option<BuffStatus>,
    pub debuff: Option<DebuffStatus>,
    pub is_gcd: bool,
    pub(crate) turn_type: FfxivTurnType,
    pub(crate) delay_millisecond: Option<TimeType>,
    pub(crate) is_modified: bool,
    pub(crate) cooldown_millisecond: i32,
    pub(crate) resource_required: Vec<ResourceRequirements>,
}

pub struct SkillInfo<S: Skill> {
    pub skill: S,
    pub guaranteed_critical_hit: bool,
    pub guaranteed_direct_hit: bool,
}

impl Skill for AttackSkill {
    fn get_id(&self) -> IdType {
        self.id
    }
    fn get_potency(&self) -> DamageType {
        self.potency
    }

    fn get_cooldown_millisecond(&self) -> TimeType {
        self.cooldown_millisecond
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

    fn is_gcd(&self) -> bool {
        self.is_gcd
    }

    fn afflict_debuff<T: StatusHolder<DebuffStatus>>(&self, target: &mut T) -> Result<()> {
        let debuff = &self.debuff;

        if let Some(debuff) = debuff {
            target.add_status(debuff.clone());
            Ok(())
        } else {
            Err(CombatComponentsError::DebuffNotFoundError(
                "Debuff not found".to_string(),
            ))
        }
    }

    fn afflict_buff<P: StatusHolder<BuffStatus>>(
        &self,
        player: &mut P,
        party: &mut Vec<P>,
    ) -> Result<()> {
        let buff = &self.buff;

        if let Some(buff) = buff {
            player.add_status(buff.clone());
            for member in party.iter_mut() {
                member.add_status(buff.clone());
            }
            Ok(())
        } else {
            Err(CombatComponentsError::BuffNotFoundError(
                "Buff not found".to_string(),
            ))
        }
    }
}

impl OwnerTracker for AttackSkill {
    fn get_owner_id(&self) -> IdType {
        self.player_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::status::StatusInfo;

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
