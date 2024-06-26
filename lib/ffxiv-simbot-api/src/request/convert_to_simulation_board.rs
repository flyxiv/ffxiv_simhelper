use crate::errors::{FfxivSimbotServiceError, Result};
use crate::request::simulation_api_request::{PlayerInfoRequest, StatsRequest};
use ffxiv_simbot_combat_components::event::FfxivEventQueue;
use ffxiv_simbot_combat_components::live_objects::player::ffxiv_player::FfxivPlayer;
use ffxiv_simbot_combat_components::IdType;
use ffxiv_simbot_db::stat_calculator::{convert_stat_info_to_power, StatInfo};
use ffxiv_simbot_db::{IncreaseType, MultiplierType, StatModifier};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::rc::Rc;

lazy_static! {
    static ref PARTNER_PRIORITY: Vec<&'static str> = vec![
        "SAM", "NIN", "MNK", "DRG", "RPR", "BLM", "SMN", "RDM", "MCH", "DNC", "BRD", "DRK", "PLD",
        "WAR", "GNB", "WHM", "SGE", "SCH", "AST",
    ];
    static ref MELEE_PRIORITY: Vec<&'static str> =
        vec!["SAM", "NIN", "MNK", "DRG", "RPR", "DRK", "PLD", "WAR", "GNB",];
    static ref RANGED_PRIORITY: Vec<&'static str> =
        vec!["BLM", "SMN", "DNC", "RDM", "MCH", "BRD", "WHM", "SGE", "SCH", "AST",];
}

pub(crate) fn convert_to_stats_info(stats: &StatsRequest) -> StatInfo {
    StatInfo {
        weapon_damage: stats.weapon_damage,
        main_stat: stats.main_stat,
        critical_strike: stats.critical_strike,
        direct_hit: stats.direct_hit,
        determination: stats.determination,
        speed: stats.speed,
        tenacity: stats.tenacity,
    }
}

pub(crate) fn create_player(
    player_info: PlayerInfoRequest,
    composition_buff: IncreaseType,
    player_jobs: &Vec<(IdType, String)>,
    stat_modifier: StatModifier,
    event_queue: Rc<RefCell<FfxivEventQueue>>,
) -> Result<FfxivPlayer> {
    let stat_info = convert_to_stats_info(&player_info.stats);
    let character_power = convert_stat_info_to_power(
        &stat_info,
        &player_info.job,
        1.0 + (composition_buff as MultiplierType / 100.0),
        &stat_modifier,
    )?;

    match player_info.job.as_str() {
        "WAR" => Ok(FfxivPlayer::new_warrior(
            player_info.player_id,
            character_power,
            event_queue,
        )),
        "PLD" => Ok(FfxivPlayer::new_paladin(
            player_info.player_id,
            character_power,
            event_queue,
        )),
        "GNB" => Ok(FfxivPlayer::new_gunbreaker(
            player_info.player_id,
            character_power,
            event_queue,
        )),
        "WHM" => Ok(FfxivPlayer::new_whitemage(
            player_info.player_id,
            character_power,
            event_queue,
        )),
        "SCH" => Ok(FfxivPlayer::new_scholar(
            player_info.player_id,
            character_power,
            event_queue,
        )),
        "SGE" => Ok(FfxivPlayer::new_sage(
            player_info.player_id,
            character_power,
            event_queue,
        )),
        "MNK" => Ok(FfxivPlayer::new_monk(
            player_info.player_id,
            character_power,
            event_queue,
        )),
        "DRG" => Ok(FfxivPlayer::new_dragoon(
            player_info.player_id,
            get_partner_id(player_info.partner1_id, player_jobs),
            character_power,
            event_queue,
        )),
        "NIN" => Ok(FfxivPlayer::new_ninja(
            player_info.player_id,
            character_power,
            event_queue,
        )),
        "SAM" => Ok(FfxivPlayer::new_samurai(
            player_info.player_id,
            character_power,
            event_queue,
        )),
        "RPR" => Ok(FfxivPlayer::new_reaper(
            player_info.player_id,
            character_power,
            event_queue,
            player_jobs.len(),
        )),
        "BRD" => Ok(FfxivPlayer::new_bard(
            player_info.player_id,
            character_power,
            event_queue,
        )),
        "DNC" => Ok(FfxivPlayer::new_dancer(
            player_info.player_id,
            get_partner_id(player_info.partner1_id, player_jobs),
            character_power,
            event_queue,
        )),
        "MCH" => Ok(FfxivPlayer::new_machinist(
            player_info.player_id,
            character_power,
            event_queue,
        )),
        "RDM" => Ok(FfxivPlayer::new_redmage(
            player_info.player_id,
            character_power,
            event_queue,
        )),
        "SMN" => Ok(FfxivPlayer::new_summoner(
            player_info.player_id,
            character_power,
            event_queue,
        )),
        "BLM" => Ok(FfxivPlayer::new_blackmage(
            player_info.player_id,
            character_power,
            event_queue,
        )),
        _ => Err(FfxivSimbotServiceError::InvalidJobString(player_info.job).into()),
    }
}

fn get_partner_id(partner_id: Option<IdType>, party_jobs: &Vec<(IdType, String)>) -> IdType {
    match partner_id {
        Some(id) => id,
        None => {
            for job in PARTNER_PRIORITY.iter() {
                for (id, job_name) in party_jobs {
                    if job_name == *job {
                        return *id;
                    }
                }
            }

            return 0;
        }
    }
}
