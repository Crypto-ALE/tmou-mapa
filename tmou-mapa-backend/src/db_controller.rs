use super::db_models::*;
use super::errors::*;




// database control holds the state of the game for teams

pub trait DbControl
{
    fn get_team(&self, id: i32) -> Option<Team>;
    fn get_teams_positions(&self) -> TmouResult<Vec<TeamPosition>>;
    fn update_team_position(&mut self, team: &Team, position: i64) -> TmouResult<Team>;
    fn get_reachable_nodes(&self, seed: i64) -> TmouResult<Pois>;
    fn get_items_in_node(&self, node_id: i64) -> TmouResult<Vec<Item>>;
    fn get_team_items(&self, team_id: i32) -> TmouResult<Vec<Item>>;
    fn get_team_items_with_timestamps(&self, team_id: i32) -> TmouResult<Vec<TeamItem>>;
    fn put_team_items(&mut self, team_id: i32, items: Vec<Item>) -> TmouResult<()>;
    fn get_badge_labels(&self) -> TmouResult<Vec<String>>;
    fn get_teams_items(&self) -> TmouResult<std::vec::Vec<TeamStandingsItem>>;
    fn get_items_teams(&self) -> TmouResult<std::vec::Vec<ItemTeam>>;
    fn get_bonuses(&self) -> TmouResult<std::vec::Vec<Bonus>>;
    fn get_game_state_by_puzzles(&self) -> TmouResult<std::vec::Vec<i64>>;
    fn get_dead_item_for_level(&self, level: i16) -> TmouResult<Item>;
}

pub trait MessagesDbControl
{
    fn get_messages(&self, team_id: i32, limit: Option<i64>) -> Option<Vec<Message>>;
    fn put_message(&self, message: WebMessage, teams_ids: Vec<i32>) -> TmouResult<()>;
}
