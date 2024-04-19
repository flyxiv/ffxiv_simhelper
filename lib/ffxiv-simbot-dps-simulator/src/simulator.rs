use crate::skill_simulator::FfxivSkillSimulator;
use crate::turn_calculator::FfxivTurnCalculator;
use ffxiv_simbot_combat_components::player::Player;
use ffxiv_simbot_combat_components::skill::Skill;
use ffxiv_simbot_combat_components::status::Status;
use ffxiv_simbot_combat_components::target::Target;
use ffxiv_simbot_combat_components::{DamageType, DpsType, IdType, TimeType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Simulate DPS for a job based on Priority System
/// 1) Read the priority table and get the next skill
/// 2) Calculate the expected damage of the skill, and distribute RDPS to the applied buffs
/// 3) After simulation is done, return the DPS and Simulation Log of the job.
/// Simulating all 8 Jobs at once is really tough, so we're gonna only count for 1 Job first.
pub trait SimulationBoard<T, P, S>
where
    T: Target + Sized,
    P: Player + Sized,
    S: Skill + Sized,
{
    fn get_player_data(&self, player_id: IdType) -> P;
    fn get_target(&self) -> T;
    fn get_rotation_log(&self) -> &Vec<S>;
    fn get_players(&self) -> &Vec<P>;
    fn get_rotation_log_mut(&self) -> &mut Vec<S>;
    fn get_players_mut(&self) -> &mut Vec<P>;
    fn use_skill(&self, player_id: IdType, skill: S);
    fn get_final_rotation_log(&self) -> Vec<PlayerSimulationData<P, S>>;
    /// Gets the RDPS Profile by each buff. Raw Damage is id 0.
    fn get_final_rdps_table(&self) -> HashMap<IdType, DpsType>;
    fn get_finish_combat_time(&self) -> TimeType;
    fn record_damage_contributions<S>(&self, status_list: Vec<S>, damage: DamageType)
    where
        S: Status + Sized,
    {
        for status in status_list {
            self.update_buff_score(status.get_id(), damage);
        }
    }
}

pub struct PlayerSimulationData<P, S>
where
    P: Player,
    S: Skill,
{
    pub player: P,
    pub skill: S,
    pub rdps_table: HashMap<IdType, DpsType>,
}

/// The main party combat simluation board for FFXIV. Think of this simulation of one instance of combat.
/// The DpsSimulator does the following:
/// Keeps track of the party and target data.
/// Keeps RDPS contribution for all buffs that are in the party.
pub struct FfxivDpsSimulatorBoard<T, P, S>
where
    T: Target,
    P: Player,
    S: Skill,
{
    skill_simulator: FfxivSkillSimulator,
    turn_simulator: FfxivTurnCalculator,

    party: Rc<RefCell<Vec<P>>>,
    target: Rc<RefCell<T>>,
    rdps_table: HashMap<IdType, DpsType>,
    finish_combat_time: TimeType,
}
