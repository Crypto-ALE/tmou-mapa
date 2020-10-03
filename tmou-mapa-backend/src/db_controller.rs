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
    fn get_teams_badges(&self) -> TmouResult<std::vec::Vec<TeamBadge>>;
}

pub trait MessagesDbControl
{
    fn get_messages(&self, team_id: i32, limit: Option<i64>) -> Option<Vec<Message>>;
    fn put_message(&self, message: WebMessage, teams_ids: Vec<i32>) -> TmouResult<()>;
}
