pub mod admin;
pub mod datetime_operators;
pub mod discovery;
pub mod game;
pub mod message;
pub mod skip;
pub mod standings;

use std::vec::Vec;

use crate::db::Item;

// helper function for controllers
pub fn get_player_level(items: &Vec<Item>) -> i16 {
    items.iter().map(|item| item.level).max().unwrap_or(-1)
}
