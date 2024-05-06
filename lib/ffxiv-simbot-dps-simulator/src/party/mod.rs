use ffxiv_simbot_combat_components::live_objects::player::Player;
use ffxiv_simbot_combat_components::status::buff_status::BuffStatus;
use ffxiv_simbot_combat_components::status::status_timer::StatusTimer;
use ffxiv_simbot_combat_components::IdType;

pub(crate) trait Party<P: Player>: StatusTimer<BuffStatus> {
    fn get_party_member(&self, player_id: IdType) -> Vec<P>;
    fn apply_buff(&mut self, player_id: IdType, skill_id: IdType);
}
