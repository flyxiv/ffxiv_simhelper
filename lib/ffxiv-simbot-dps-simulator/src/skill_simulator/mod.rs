use crate::damage_rdps_profile::FfxivRaidDamageTable;
use ffxiv_simbot_combat_components::DamageType;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) mod ffxiv_skill_simulator;
mod skill_calculator;

/// Simulates the effect of a single skill and distribute the damage contribution of each
/// buff to the rightful owner.

pub(crate) struct SkillDamageResult {
    pub(crate) raw_damage: DamageType,
    /// damage after adding all buffs/debuffs
    pub(crate) final_damage: DamageType,
    pub(crate) raid_damage_profile: FfxivRaidDamageTable,
}

pub(crate) struct SkillSimulationResult {
    pub(crate) skill_damage_result: SkillDamageResult,
    pub(crate) buff: Option<StatusApply<BuffStatus>>,
    pub(crate) debuff: Option<StatusApply<DebuffStatus>>,
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
