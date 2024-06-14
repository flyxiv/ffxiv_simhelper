use crate::damage_calculator::multiplier_calculator::DIRECT_HIT_DAMAGE_MULTIPLIER;
use crate::id_entity::IdEntity;
use crate::status::status_info::StatusInfo;
use crate::{IdType, ResourceType, TimeType};
use ffxiv_simbot_db::MultiplierType;

pub mod buff_status;
pub mod debuff_status;
pub mod status_holder;
pub mod status_info;
pub mod status_timer;

/// Interface for player buffs and target debuffs
pub trait Status: Sized + IdEntity {
    /// in miliseconds
    fn get_duration_left_millisecond(&self) -> TimeType;
    fn set_duration_left_millisecond(&mut self, duration: TimeType);
    /// add trigger event to skill use
    fn get_name(&self) -> &String;
    fn start_duration(&mut self) {
        self.set_duration_left_millisecond(self.get_duration_millisecond());
    }
    /// get the type of status and amount
    /// ex) Battle Litany: 10% Crit Buff = CritHitRatePercent(10)
    fn get_status_info(&self) -> &Vec<StatusInfo>;
    fn get_duration_millisecond(&self) -> TimeType;
    fn is_raidwide(&self) -> bool;
    fn add_stack(&mut self, stack: ResourceType);
    fn get_stack(&self) -> ResourceType;
    fn get_damage_skill_id(&self) -> Option<IdType>;

    fn get_critical_strike_rate_increase(&self, is_guaranteed_crit: bool) -> MultiplierType {
        let mut critical_strike_rate_increase = 0.0;

        if is_guaranteed_crit {
            return critical_strike_rate_increase;
        }

        let status_infos = self.get_status_info();

        for status_info in status_infos {
            match status_info {
                StatusInfo::CritHitRatePercent(percent) => {
                    critical_strike_rate_increase += (*percent as MultiplierType) / 100.0
                }
                _ => {}
            };
        }

        critical_strike_rate_increase
    }

    fn get_direct_hit_rate_increase(&self, is_guaranteed_direct_hit: bool) -> MultiplierType {
        let mut direct_hit_rate_increase = 0.0;

        if is_guaranteed_direct_hit {
            return direct_hit_rate_increase;
        }

        let status_infos = self.get_status_info();

        for status_info in status_infos {
            match status_info {
                StatusInfo::DirectHitRatePercent(percent) => {
                    direct_hit_rate_increase += (*percent as MultiplierType) / 100.0
                }
                _ => {}
            };
        }

        direct_hit_rate_increase
    }

    fn get_damage_increase(
        &self,
        is_guaranteed_crit: bool,
        is_guaranteed_direct_hit: bool,
        crit_damage_rate: MultiplierType,
    ) -> MultiplierType {
        let mut total_damage_increase = 0.0;
        let status_infos = self.get_status_info();

        for status_info in status_infos {
            match status_info {
                StatusInfo::DamagePercent(percent) => {
                    total_damage_increase += (*percent as MultiplierType) / 100.0
                }
                StatusInfo::CritHitRatePercent(percent) => {
                    let damage_increase = if is_guaranteed_crit {
                        (*percent as MultiplierType) / 100.0 * (crit_damage_rate - 1.0)
                    } else {
                        0.0
                    };

                    total_damage_increase += damage_increase;
                }
                StatusInfo::DirectHitRatePercent(percent) => {
                    let damage_increase = if is_guaranteed_direct_hit {
                        (*percent as MultiplierType) / 100.0 * (DIRECT_HIT_DAMAGE_MULTIPLIER - 1.0)
                    } else {
                        0.0
                    };

                    total_damage_increase += damage_increase;
                }
                _ => {}
            };
        }

        total_damage_increase
    }
}
