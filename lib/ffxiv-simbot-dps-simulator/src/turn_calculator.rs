use ffxiv_simbot_combat_components::player::{FfxivPlayer, Player};
use ffxiv_simbot_combat_components::IdType;
use std::cell::RefCell;
use std::rc::Rc;

/// Finds which player gets the next turn the earliest, and give that
/// player the next turn.

pub(crate) trait TurnCalculator<P>
where
    P: Player,
{
    fn find_next_turn_player_id(&self, players: &Vec<Rc<RefCell<P>>>) -> IdType {
        let earliest_turn_player = players
            .iter()
            .min_by_key(|player| {
                let player = player.borrow();
                player.get_next_turn_time_milliseconds()
            })
            .unwrap();

        let earliest_turn_player = earliest_turn_player.borrow();
        earliest_turn_player.get_id()
    }
}

pub(crate) struct FfxivTurnCalculator {}

impl TurnCalculator<FfxivPlayer> for FfxivTurnCalculator {}
