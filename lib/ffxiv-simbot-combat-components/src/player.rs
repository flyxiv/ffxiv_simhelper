use crate::simulator::{FfxivDpsSimulator, PlayerSimulationData};
use crate::skill::{Skill, NON_GCD_DELAY_MILLISECOND};
use crate::status::{BuffStatus, Status, StatusHolder, StatusInfo, StatusTimer};
use crate::{DamageProfile, DamageType, IdType, TimeType};
use ffxiv_simbot_lib_db::job::Job;
use ffxiv_simbot_lib_db::stat_calculator::CharacterPower;
use std::cell::{Ref, RefCell, RefMut};
use ffxiv_simbot_lib_db::DamageMultiplierType;
use crate::target::Target;

/// If the delay is over 3 * OGCD delay, then it is turn to use a GCD skill,
/// Since in FFXIV a player can use at most 2 OGCD skills between GCD skills.
/// so 1 GCD delay + 2 oGCD delay = 3 * oGCD delay.
static GCD_TURN_DELAY_THRESHOLD: TimeType = 3 * NON_GCD_DELAY_MILLISECOND;
static MAX_MANA: i32 = 10000;

/// Saves information about the player: buffs, stat multipliers, jobs.
pub trait Player: Sized + StatusHolder<BuffStatus> {
    fn get_job(&self) -> &Job;
    fn get_player_power(&self) -> &CharacterPower;
    fn get_delay(&self) -> TimeType;
    fn use_skill<S: Skill>(&mut self, skill: S) -> ;

    fn has_resources_for_skill<S: Skill>(&self, skill: S) -> bool;
    fn get_damage_profile(&self) -> DamageProfile;
    fn get_combat_time_millisecond(&self) -> TimeType;
    fn get_next_gcd_time_millisecond(&self) -> TimeType;
}

pub(crate) trait PlayerPowerCalculator {
    fn calculate_total_multiplier(&self) -> DamageMultiplierType;
    fn calculate_gcd_delay(&self, base_delay: TimeType) -> TimeType;
}

pub(crate) trait TurnType {
    fn get_next_turn_type<P>(&self, player: &P) -> PlayerTurn where P: Player + Sized;
}

/// Represents what type of skill the player can use the next turn.
/// GCD: Global Cooldown Skill
/// oGCD1: First oGCD Skill after a GCD skill
/// oGCD2: Second oGCD Skill after a GCD
pub(crate) enum FfxivTurnType {
    Gcd,
    Ogcd1,
    Ogcd2,
}

impl PartialEq for FfxivTurnType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TurnType::Gcd, TurnType::Gcd) => true,
            (TurnType::Ogcd1, TurnType::Ogcd2) => true,
            _ => false,
        }
    }
}

impl Eq for FfxivTurnType {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl FfxivTurnType {
    fn get_next_turn_type<P>(&self, player: &P) -> PlayerTurn where P: Player + Sized {
        let current_turn_combat_time = player.get_combat_time_millisecond();

        match self {
            FfxivTurnType::Gcd => {
                PlayerTurn {
                    turn_type: FfxivTurnType::Ogcd1,
                    next_turn_combat_time_millisecond: current_turn_combat_time + NON_GCD_DELAY_MILLISECOND
                }
            },
            FfxivTurnType::Ogcd1 => {
                if player.get_delay() >= GCD_TURN_DELAY_THRESHOLD {
                    PlayerTurn {
                        turn_type: FfxivTurnType::Gcd,
                        next_turn_combat_time_millisecond: player.get_next_gcd_time_millisecond()
                    }
                } else {
                    PlayerTurn {
                        turn_type: FfxivTurnType::Ogcd2,
                        next_turn_combat_time_millisecond: current_turn_combat_time + NON_GCD_DELAY_MILLISECOND
                    }
                }
            }
            FfxivTurnType::Ogcd2 => PlayerTurn {
                turn_type: FfxivTurnType::Gcd,
                next_turn_combat_time_millisecond: player.get_next_gcd_time_millisecond()
            }
        }
    }
}

/// https://github.com/flyxiv/ffxiv_simbot/issues/6#issuecomment-2057989693
/// Ffxiv players have a 0.7 second delay after using an oGCD skill, and GCD seconds delay for each
/// Consecutive GCD skill, so FFXIV's combat can be seen as players taking turns, with the player
/// getting the earliest turn going first.
pub(crate) struct PlayerTurn {
    turn_type: FfxivTurnType,
    next_turn_combat_time_millisecond: TimeType,
}

impl Default for PlayerTurn {
    fn default() -> Self {
        PlayerTurn {
            turn_type: TurnType::Gcd,
            next_turn_combat_time_millisecond: 0,
        }
    }
}

impl PartialEq for PlayerTurn {
    fn eq(&self, other: &Self) -> bool {
        self.next_turn_combat_time_millisecond == other.next_turn_combat_time_millisecond
            && self.turn_type == other.turn_type
    }
}

impl Eq for PlayerTurn {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl PartialOrd for PlayerTurn {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.next_turn_combat_time_millisecond
            .partial_cmp(&other.next_turn_combat_time_millisecond)
    }
}

impl Ord for PlayerTurn {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.next_turn_combat_time_millisecond.cmp(&other.next_turn_combat_time_millisecond)
    }
}

impl TurnType for PlayerTurn {
    fn get_next_turn_type<P>(&self, player: &P) -> PlayerTurn where P: Player + Sized {
        self.turn_type.get_next_turn_type(player)
    }
}


/// The Abstraction for an actual FFXIV Player in the combat.
pub struct FfxivPlayer {
    /// Stat/Job Data about the player
    id: IdType,
    job: Job,
    power: CharacterPower,

    /// Realtime Combat Data about the player
    buff_list: RefCell<Vec<BuffStatus>>,
    current_combo: Option<IdType>,
    /// How many seconds passed after the most recent GCD. If delay is close to GCD, an oGCD will
    /// clip the player's next GCD so it becomes a GCD turn.
    total_delay: RefCell<TimeType>,

    /// Combat time related data
    combat_time_millisecond: TimeType,
    next_gcd_time_millisecond: TimeType,
    next_turn: PlayerTurn,

    mana_available: Option<i32>,

    damage_profile: DamageProfile,
}

impl Player for FfxivPlayer {
    fn get_job(&self) -> &Job {
        &self.job
    }

    fn get_player_power(&self) -> &CharacterPower {
        &self.power
    }

    fn get_delay(&self) -> TimeType {
        self.total_delay.borrow().clone()
    }

    fn use_skill<T, P, S>(&mut self, skill: S, simulator: &FfxivDpsSimulator<T, P, S>) where T: Target, P: Player, S: Skill {
        let mut total_delay = self.total_delay.borrow_mut();

        if skill.is_gcd() {
            let gcd_delay = f64::ceil(self.get_player_power().speed_multiplier)
        }

        self.calculate_next_turn();
    }

    fn has_resources_for_skill<S: Skill>(&self, skill: S) -> bool {
        /// TODO: Implement mana resource check for casters.
        return false;
    }

    fn get_damage_profile(self) -> DamageProfile {
        self.damage_profile
    }

    fn get_combat_time_millisecond(&self) -> TimeType {
        self.combat_time_millisecond
    }

    fn get_next_gcd_time_millisecond(&self) -> TimeType {
        self.next_gcd_time_millisecond
    }
}

impl FfxivPlayer {
    /// After using a turn, calculate when the next turn will be in combat time,
    /// and also figure out if it is a GCD/oGCD turn.
    pub(crate) fn calculate_next_turn<S>(&mut self) -> PlayerTurn
    where
        S: Skill,
    {
        let current_turn = &self.next_turn;
        let next_turn_type = current_turn
            .turn_type
            .get_next_turn_type(self.get_delay());

        match next_turn_type
    }
}

impl StatusHolder<BuffStatus> for FfxivPlayer {
    fn get_status_list(&self) -> Ref<Vec<BuffStatus>> {
        self.buff_list.borrow()
    }

    fn get_status_list_mut(&self) -> RefMut<Vec<BuffStatus>> {
        self.buff_list.borrow_mut()
    }

    fn get_combat_time_millisecond(&self) -> TimeType {
        self.combat_time_millisecond
    }
}

impl StatusTimer<BuffStatus> for FfxivPlayer {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::status::{Status, StatusInfo};

    #[test]
    fn target_basic_test() {
        let mut target = FfxivPlayer {
            id: 0,
            job: Job::default(),
            power: CharacterPower::default(),
            buff_list: RefCell::new(vec![]),
            total_delay: RefCell::new(0),
            combat_time_millisecond: 0,
            buff_scores: RefCell::new(Default::default()),
            mana_available: RefCell::new(0),
        };

        let buff1 = BuffStatus {
            id: 1,
            duration_left_millisecond: 1000,
            status_data: StatusInfo::CritHitRatePercent(10),
            duration_millisecond: 1000,
            is_raidwide: false,
            cumulative_damage: None,
            owner_player_id: 0,
        };

        target.add_status(buff1);
        assert_eq!(target.get_status_list().len(), 1);

        let buff = &target.get_status_list()[0];
        assert_eq!(buff.id, 1);
        assert_eq!(buff.get_duration_left_millisecond(), 1000);
        assert_eq!(buff.get_status_info(), StatusInfo::CritHitRatePercent(10));
    }

    #[test]
    fn target_debuff_timer_test() {
        let mut target = FfxivPlayer {
            id: 0,
            job: Job::default(),
            power: CharacterPower::default(),
            buff_list: RefCell::new(vec![]),
            total_delay: RefCell::new(0),
            combat_time_millisecond: 0,
            buff_scores: RefCell::new(Default::default()),
            mana_available: RefCell::new(0),
        };

        let two_seconds_left_buff = BuffStatus {
            id: 1,
            duration_left_millisecond: 2000,
            status_data: StatusInfo::CritHitRatePercent(10),
            duration_millisecond: 10000,
            is_raidwide: false,
            cumulative_damage: None,
            owner_player_id: 0,
        };

        let five_seconds_left_buff = BuffStatus {
            id: 2,
            duration_left_millisecond: 5000,
            status_data: StatusInfo::CritHitRatePercent(10),
            duration_millisecond: 10000,
            is_raidwide: true,
            cumulative_damage: None,
            owner_player_id: 0,
        };

        target.add_status(two_seconds_left_buff);
        target.add_status(five_seconds_left_buff);

        target.update_combat_time(3000);

        assert_eq!(target.get_status_list().len(), 1);

        let buff = &target.get_status_list()[0];
        assert_eq!(buff.id, 2);
        assert_eq!(buff.get_duration_left_millisecond(), 2000);
    }
}
