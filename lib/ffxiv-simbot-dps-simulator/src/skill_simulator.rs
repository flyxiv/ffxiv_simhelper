use crate::skill_calculator::{FfxivSkillCalculator, SkillCalculator, SkillDamageResult};
use ffxiv_simbot_combat_components::live_objects::player::ffxiv_player::FfxivPlayer;
use ffxiv_simbot_combat_components::live_objects::player::Player;
use ffxiv_simbot_combat_components::live_objects::target::ffxiv_target::FfxivTarget;
use ffxiv_simbot_combat_components::live_objects::target::Target;
use ffxiv_simbot_combat_components::IdType;
use std::cell::RefCell;
use std::rc::Rc;

use crate::raw_damage_calculator::{FfxivRawDamageCalculator, RawDamageCalculator};
use ffxiv_simbot_combat_components::skill::attack_skill::{AttackSkill, SkillInfo};
use ffxiv_simbot_combat_components::skill::skill::Skill;
use ffxiv_simbot_combat_components::status::buff_status::BuffStatus;
use ffxiv_simbot_combat_components::status::debuff_status::DebuffStatus;

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
        players: &Vec<Rc<RefCell<FfxivPlayer>>>,
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
        let skill_user = players[turn_player_id].clone();

        let raw_damage = self
            .raw_damage_calculator
            .calculate_raw_damage(skill_info, skill_user.borrow().get_player_power());

        let player_power = skill_user.borrow().get_player_power().clone();

        self.skill_calculator.make_damage_profile(
            skill_user,
            target.clone(),
            raw_damage,
            &player_power,
            turn_player_id,
        )
    }
}

pub struct FfxivSkillSimulator {
    pub(crate) skill_calculator: FfxivSkillCalculator,
    raw_damage_calculator: FfxivRawDamageCalculator,
}

impl Default for FfxivSkillSimulator {
    fn default() -> Self {
        FfxivSkillSimulator {
            skill_calculator: FfxivSkillCalculator::default(),
            raw_damage_calculator: FfxivRawDamageCalculator::default(),
        }
    }
}
