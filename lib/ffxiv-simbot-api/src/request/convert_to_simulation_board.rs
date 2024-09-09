use crate::errors::{FfxivSimbotServiceError, Result};
use crate::request::simulation_api_request::PlayerInfoRequest;
use ffxiv_simbot_combat_components::event::FfxivEventQueue;
use ffxiv_simbot_combat_components::live_objects::player::ffxiv_player::FfxivPlayer;
use ffxiv_simbot_combat_components::live_objects::player::player_power::add_main_stat;
use ffxiv_simbot_combat_components::types::{BuffIncreasePercentType, IncreaseType, PlayerIdType};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::rc::Rc;

enum PartnerCategory {
    MeleePartner,
    RangedPartner,
    AllPartner,
}

lazy_static! {
    static ref ALL_PARTNER_PRIORITY: Vec<&'static str> = vec![
        "VPR", "PCT", "SAM", "NIN", "MNK", "DRG", "RPR", "BLM", "SMN", "RDM", "MCH", "DNC", "BRD",
        "DRK", "PLD", "WAR", "GNB", "WHM", "SGE", "SCH", "AST",
    ];
    static ref MELEE_PRIORITY: Vec<&'static str> =
        vec!["VPR", "SAM", "NIN", "MNK", "DRG", "RPR", "DRK", "PLD", "WAR", "GNB",];
    static ref RANGED_PRIORITY: Vec<&'static str> =
        vec!["PCT", "BLM", "SMN", "DNC", "RDM", "MCH", "BRD", "WHM", "SGE", "SCH", "AST",];
}

pub(crate) fn create_player(
    player_info: PlayerInfoRequest,
    composition_buff_percent: BuffIncreasePercentType,
    player_jobs: &[(PlayerIdType, String)],
    event_queue: Rc<RefCell<FfxivEventQueue>>,
    use_pots: bool,
) -> Result<FfxivPlayer> {
    let character_power = player_info.power;
    let character_power = add_main_stat(
        &character_power,
        &player_info.job_abbrev,
        character_power.main_stat as IncreaseType,
        composition_buff_percent,
    );
    let player_count = player_jobs.len();

    match player_info.job_abbrev.as_str() {
        "WAR" => Ok(FfxivPlayer::new_warrior(
            player_info.player_id,
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        "PLD" => Ok(FfxivPlayer::new_paladin(
            player_info.player_id,
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        "DRK" => Ok(FfxivPlayer::new_darkknight(
            player_info.player_id,
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        "GNB" => Ok(FfxivPlayer::new_gunbreaker(
            player_info.player_id,
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        "WHM" => Ok(FfxivPlayer::new_whitemage(
            player_info.player_id,
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        "SCH" => Ok(FfxivPlayer::new_scholar(
            player_info.player_id,
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        "AST" => Ok(FfxivPlayer::new_astrologian(
            player_info.player_id,
            character_power,
            get_partner_id(
                player_info.partner1_id,
                player_jobs,
                PartnerCategory::MeleePartner,
            ),
            get_partner_id(
                player_info.partner2_id,
                player_jobs,
                PartnerCategory::RangedPartner,
            ),
            event_queue,
            player_count,
            use_pots,
        )),
        "SGE" => Ok(FfxivPlayer::new_sage(
            player_info.player_id,
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        "MNK" => Ok(FfxivPlayer::new_monk(
            player_info.player_id,
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        "DRG" => Ok(FfxivPlayer::new_dragoon(
            player_info.player_id,
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        "NIN" => Ok(FfxivPlayer::new_ninja(
            player_info.player_id,
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        "SAM" => Ok(FfxivPlayer::new_samurai(
            player_info.player_id,
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        "RPR" => Ok(FfxivPlayer::new_reaper(
            player_info.player_id,
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        "VPR" => Ok(FfxivPlayer::new_viper(
            player_info.player_id,
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        "BRD" => Ok(FfxivPlayer::new_bard(
            player_info.player_id,
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        "DNC" => Ok(FfxivPlayer::new_dancer(
            player_info.player_id,
            get_partner_id(
                player_info.partner1_id,
                player_jobs,
                PartnerCategory::AllPartner,
            ),
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        "MCH" => Ok(FfxivPlayer::new_machinist(
            player_info.player_id,
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        "RDM" => Ok(FfxivPlayer::new_redmage(
            player_info.player_id,
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        "SMN" => Ok(FfxivPlayer::new_summoner(
            player_info.player_id,
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        "BLM" => Ok(FfxivPlayer::new_blackmage(
            player_info.player_id,
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        "PCT" => Ok(FfxivPlayer::new_pictomancer(
            player_info.player_id,
            character_power,
            event_queue,
            player_count,
            use_pots,
        )),
        _ => Err(FfxivSimbotServiceError::InvalidJobString(player_info.job_abbrev).into()),
    }
}

fn get_partner_id(
    partner_id: Option<PlayerIdType>,
    party_jobs: &[(PlayerIdType, String)],
    partner_category: PartnerCategory,
) -> PlayerIdType {
    match partner_id {
        Some(id) => id,
        None => {
            let partner_priority_table = match partner_category {
                PartnerCategory::MeleePartner => MELEE_PRIORITY.iter(),
                PartnerCategory::RangedPartner => RANGED_PRIORITY.iter(),
                PartnerCategory::AllPartner => ALL_PARTNER_PRIORITY.iter(),
            };

            for job in partner_priority_table {
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
