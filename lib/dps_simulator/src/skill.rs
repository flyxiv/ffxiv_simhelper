use crate::status::{BuffStatus, DebuffStatus};

pub static NON_GCD_DELAY_MILLISECOND: i32 = 700;

pub(crate) enum ResourceRequirements {
    Mana(i32),
    Status(i32),
    Combo(i32),
}

pub trait Skill {
    fn get_potency(&self) -> i32;
    fn get_delay(&self) -> f64;
    fn is_gcd(&self) -> bool;
    fn afflict_debuff(&self) -> Option<DebuffStatus>;
    fn afflict_buff(&self) -> Option<BuffStatus>;
}

pub struct AttackSkill {
    pub name: String,
    pub potency: i32,
    pub buff: Option<BuffStatus>,
    pub debuff: Option<DebuffStatus>,
    pub is_gcd: bool,
    pub is_modified: bool,
    pub cooldown_millisecond: i32,
    pub resource_required: Vec<ResourceRequirements>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::status::StatusInfo;

    #[test]
    fn attack_skill_test() {
        let skill = AttackSkill {
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
            is_modified: false,
            cooldown_millisecond: 2500,
            resource_required: vec![ResourceRequirements::Mana(100)],
        };

        assert_eq!(skill.get_potency(), 100);
        assert_eq!(skill.get_delay(), 2.5);
        assert_eq!(skill.is_gcd(), true);
        assert_eq!(skill.afflict_buff().is_some(), true);
        assert_eq!(skill.afflict_debuff().is_none(), true);
    }
}
