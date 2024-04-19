use crate::skill_calculator::{FfxivSkillCalculator, SkillCalculator, SkillDamageResult};
use ffxiv_simbot_combat_components::IdType;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::raw_damage_calculator::{FfxivRawDamageCalculator, RawDamageCalculator};
use ffxiv_simbot_combat_components::player::{FfxivPlayer, Player};
use ffxiv_simbot_combat_components::skill::{AttackSkill, Skill, SkillInfo};
use ffxiv_simbot_combat_components::status::{BuffStatus, DebuffStatus};
use ffxiv_simbot_combat_components::target::{FfxivTarget, Target};

pub(crate) struct SkillSimulationResult {
    pub(crate) skill_damage_result: SkillDamageResult,
    pub(crate) buff: Option<BuffStatus>,
    pub(crate) debuff: Option<DebuffStatus>,
    pub(crate) skill_user_id: IdType,
}

/// Gets necessary data needed for simulating skill's damage and effects
/// Then orders players/target/rdps table to update the simulated result
pub(crate) trait SkillSimulator<T, P, S>
where
    T: Target,
    P: Player,
    S: Skill,
{
    fn make_skill_simulation_result(
        &self,
        turn_player_id: IdType,
        players: Rc<RefCell<Vec<FfxivPlayer>>>,
        target: Rc<RefCell<FfxivTarget>>,
        skill_info: &SkillInfo<AttackSkill>,
    ) -> SkillDamageResult;
}

impl SkillSimulator<FfxivTarget, FfxivPlayer, AttackSkill> for FfxivSkillSimulator {
    fn make_skill_simulation_result(
        &self,
        turn_player_id: IdType,
        players: &Vec<Rc<RefCell<FfxivPlayer>>>,
        target: Rc<RefCell<FfxivTarget>>,
        skill_info: &SkillInfo<AttackSkill>,
    ) -> SkillDamageResult {
        let skill_user = players[turn_player_id].borrow();
        let power = skill_user.get_player_power();

        let raw_damage = self
            .raw_damage_calculator
            .calculate_raw_damage(skill_info, power);

        self.skill_calculator.make_damage_profile(
            skill_user,
            target.borrow(),
            raw_damage,
            power,
            turn_player_id,
        )
    }
}

pub(crate) struct FfxivSkillSimulator {
    skill_calculator: FfxivSkillCalculator,
    raw_damage_calculator: FfxivRawDamageCalculator,
}
