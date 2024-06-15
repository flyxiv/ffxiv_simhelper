use crate::errors::{FfxivSimbotServiceError, Result};
use crate::request::simulation_api_request::{PlayerInfoRequest, StatsRequest};
use ffxiv_simbot_combat_components::event::FfxivEventQueue;
use ffxiv_simbot_combat_components::live_objects::player::ffxiv_player::FfxivPlayer;
use ffxiv_simbot_db::stat_calculator::{convert_stat_info_to_power, StatInfo};
use ffxiv_simbot_db::{IncreaseType, MultiplierType, StatModifier};
use std::cell::RefCell;
use std::rc::Rc;

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
        "NIN" => Ok(FfxivPlayer::new_ninja(
            player_info.player_id,
            character_power,
            event_queue,
        )),
        "SGE" => Ok(FfxivPlayer::new_sage(
            player_info.player_id,
            character_power,
            event_queue,
        )),
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
        "WHM" => Ok(FfxivPlayer::new_whitemage(
            player_info.player_id,
            character_power,
            event_queue,
        )),
        "BLM" => Ok(FfxivPlayer::new_blackmage(
            player_info.player_id,
            character_power,
            event_queue,
        )),
        "BRD" => Ok(FfxivPlayer::new_bard(
            player_info.player_id,
            character_power,
            event_queue,
        )),
        "MNK" => Ok(FfxivPlayer::new_monk(
            player_info.player_id,
            character_power,
            event_queue,
        )),
        //TODO: Choose partner ID.
        "DRG" => Ok(FfxivPlayer::new_dragoon(
            player_info.player_id,
            player_info.player_id + 1,
            character_power,
            event_queue,
        )),
        "DNC" => Ok(FfxivPlayer::new_dancer(
            player_info.player_id,
            player_info.player_id + 1,
            character_power,
            event_queue,
        )),
        _ => Err(FfxivSimbotServiceError::InvalidJobString(player_info.job).into()),
    }
}
