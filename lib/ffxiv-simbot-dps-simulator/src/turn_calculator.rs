use crate::IdType;
use ffxiv_simbot_combat_components::player::{FfxivPlayer, Player};

/// Finds which player gets the next turn the earliest, and give that
/// player the next turn.

pub(crate) trait TurnCalculator<P>
where
    P: Player,
{
    fn find_next_turn_player_id(&self, players: &Vec<P>) -> IdType {
        let earliest_turn_player = players
            .iter()
            .min_by_key(|player| player.get_next_gcd_time_millisecond())
            .unwrap();

        earliest_turn_player.get_id()
    }
}

pub(crate) struct FfxivTurnCalculator {}

impl TurnCalculator<FfxivPlayer> for FfxivTurnCalculator {}
