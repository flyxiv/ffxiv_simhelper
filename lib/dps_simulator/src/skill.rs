use crate::player::Player;
use crate::status::{BuffStatus, DebuffStatus};
use crate::target::Target;
use crate::{Result, SimulatorError, TimeType};

pub static NON_GCD_DELAY_MILLISECOND: i32 = 700;

pub(crate) enum ResourceRequirements {
    Mana(i32),
    Status(i32),
    Combo(i32),
}

pub trait Skill {
    fn get_potency(&self) -> i32;
    fn get_cooldown_millisecond(&self) -> TimeType;
    fn get_delay_millisecond(&self) -> TimeType;
    fn is_gcd(&self) -> bool;
    fn afflict_debuff<T: Target>(&self, target: &mut T) -> Result<()>;
    fn afflict_buff<P: Player>(&self, player: &mut P, party: &mut Vec<P>) -> Result<()>;
}

pub struct AttackSkill {
    pub(crate) name: String,
    pub(crate) potency: i32,
    pub(crate) buff: Option<BuffStatus>,
    pub(crate) debuff: Option<DebuffStatus>,
    pub(crate) is_gcd: bool,
    pub(crate) delay_millisecond: Option<TimeType>,
    pub(crate) is_modified: bool,
    pub(crate) cooldown_millisecond: i32,
    pub(crate) resource_required: Vec<ResourceRequirements>,
}

impl Skill for AttackSkill {
    fn get_potency(&self) -> i32 {
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

    fn afflict_debuff<T: Target>(&self, target: &mut T) -> Result<()> {
        let debuff = &self.debuff;

        if let Some(debuff) = debuff {
            target.add_status(debuff.clone());
            Ok(())
        } else {
            Err(SimulatorError::DebuffNotFoundError(
                "Debuff not found".to_string(),
            ))
        }
    }

    fn afflict_buff<P: Player>(&self, player: &mut P, party: &mut Vec<P>) -> Result<()> {
        let buff = &self.buff;

        if let Some(buff) = buff {
            player.apply_buff(buff.clone());
            for member in party.iter_mut() {
                member.apply_buff(buff.clone());
            }
            Ok(())
        } else {
            Err(SimulatorError::BuffNotFoundError(
                "Buff not found".to_string(),
            ))
        }
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
            potency: 500,
            buff: Some(BuffStatus {
                id: 1,
                duration_left_millisecond: 15000,
                status_data: StatusInfo::DamagePercent(10),
                duration_millisecond: 15000,
                is_raidwide: false,
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
