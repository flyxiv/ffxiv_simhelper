use crate::skill_calculator::{FfxivSkillCalculator, SkillCalculator, SkillDamageResult};
use ffxiv_simbot_combat_components::IdType;

use crate::raw_damage_calculator::{FfxivRawDamageCalculator, RawDamageCalculator};
use ffxiv_simbot_combat_components::player::{FfxivPlayer, Player};
use ffxiv_simbot_combat_components::skill::{AttackSkill, Skill, SkillInfo};
use ffxiv_simbot_combat_components::status::{BuffStatus, DebuffStatus, StatusHolder};
use ffxiv_simbot_combat_components::target::{FfxivTarget, Target};

pub(crate) struct SkillSimulationResult {
    skill_damage_result: SkillDamageResult,
    buff: Option<BuffStatus>,
    debuff: Option<DebuffStatus>,
    skill_user_id: IdType,
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
        &mut self,
        turn_player_id: IdType,
        players: &Vec<FfxivPlayer>,
        target: &FfxivTarget,
        skill_info: &SkillInfo<AttackSkill>,
    ) -> SkillDamageResult;
}

impl FfxivSkillSimulator {
    fn make_skill_simulation_result(
        &mut self,
        turn_player_id: IdType,
        players: &Vec<FfxivPlayer>,
        target: &FfxivTarget,
        skill_info: &SkillInfo<AttackSkill>,
    ) -> SkillDamageResult {
        let skill_user = &players[turn_player_id];

        let raw_damage = self
            .raw_damage_calculator
            .calculate_raw_damage(skill_info, skill_user.get_power());

        self.skill_calculator.make_damage_profile(
            skill_user,
            target,
            raw_damage,
            skill_user.get_power(),
            turn_player_id,
        )
    }
}

pub(crate) struct FfxivSkillSimulator {
    skill_calculator: FfxivSkillCalculator,
    raw_damage_calculator: FfxivRawDamageCalculator,
}
