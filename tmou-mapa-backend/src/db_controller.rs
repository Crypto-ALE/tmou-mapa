use super::db_models::*;
use super::errors::*;




// database control holds the state of the game for teams

pub trait DbControl
{
    fn get_team(&self, id: i32) -> Option<Team>;
    fn put_team(&mut self, team: Team) -> TmouResult<Team>;
    fn update_team_position(&mut self, team: &Team, position: i64) -> TmouResult<()>;
    fn get_reachable_nodes(&self, seed: i64) -> TmouResult<Pois>;
    fn get_items_in_node(&self, node_id: i64) -> TmouResult<Vec<Item>>;
}
