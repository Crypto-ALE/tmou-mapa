pub mod admin;
pub mod datetime_operators;
pub mod discovery;
pub mod game;
pub mod message;
pub mod skip;
pub mod standings;
pub mod tmou22;

use std::vec::Vec;

use crate::db as db;
use crate::api as api;

// helper function for controllers
pub fn get_player_level(items: &Vec<db::Item>) -> i16 {
    items.iter().map(|item| item.level).max().unwrap_or(-1)
}

pub fn get_player_level_api(items: &Vec<api::Item>) -> i16 {
    items.iter().map(|item| item.level).max().unwrap_or(-1)
}
