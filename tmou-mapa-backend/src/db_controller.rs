use super::db_models::*;
use super::errors::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};


// for in-memory db
use std::fs::{File, read_to_string};
use std::io::prelude::*;
use super::errors;




// database control holds the state of the game for teams
// the map data are handled by map module

pub trait DbControl
{
    fn get_team(&self, id: i32) -> Option<Team>;
    fn put_team(&mut self, team: Team) -> TmouResult<()>;
    fn update_team_position(&mut self, team: &Team, position: i64) -> TmouResult<()>;
    fn get_reachable_nodes(&self, seed: i64) -> TmouResult<Pois>;
}
