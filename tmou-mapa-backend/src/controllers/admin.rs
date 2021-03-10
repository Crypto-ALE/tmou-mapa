use itertools::*;

use crate::controllers::game;
use crate::controllers::standings;
use crate::database::db::{Db, ItemEditingDb};
use crate::models::errors::*;
use crate::models::*;
use std::sync::Mutex;

const DEFAULT_NODE_ID: i64 = 3750367566;

////////////////////////////////////////////////////////////////////
/// Interface
////////////////////////////////////////////////////////////////////

pub fn get_teams_positions(db: &impl Db) -> TmouResult<Vec<api::TeamPosition>> {
    let teams_positions = db.get_teams_positions()?;
    Ok(teams_positions.iter().map_into().collect())
}

pub fn unwrap_incoming_message(
    db: &impl Db,
    message: api::IncomingMessage,
) -> TmouResult<(db::Team, api::Message)> {
    let inner_message = message.message;
    db.get_team(message.recipient_id)
        .and_then(|team| Some((team, inner_message)))
        .ok_or(TmouError {
            message: format!("Team with id {} not found.", message.recipient_id),
            response: 400,
        })
}

pub fn get_teams_standings(db: &impl Db) -> TmouResult<api::Standings> {
    let teams_items_db = db.get_teams_items()?;
    standings::calculate_teams_standings(teams_items_db)
}

impl From<&db::TeamPosition> for api::TeamPosition {
    fn from(value: &db::TeamPosition) -> Self {
        api::TeamPosition {
            team_name: value.team_name.clone(),
            lat: value.lat,
            lon: value.lon,
            level: value.level.unwrap_or(0),
        }
    }
}

pub struct Position {
    pub node_id: Mutex<Option<i64>>,
}

impl Position {
    pub fn new() -> Position {
        Position {
            node_id: Mutex::new(None),
        }
    }
}

pub fn get_pois(db: &impl Db, pos: &Position) -> TmouResult<api::Pois> {
    let pos_holder = *pos.node_id.lock().unwrap();
    let node_id = match pos_holder {
        Some(id) => id,
        None => {
            *pos.node_id.lock().unwrap() = Some(DEFAULT_NODE_ID);
            DEFAULT_NODE_ID
        }
    };
    game::get_pois_for_position(db, node_id)
}

pub fn go_to_node(db: &impl Db, node_id: i64, pos: &Position) -> TmouResult<api::Pois> {
    *pos.node_id.lock().unwrap() = Some(node_id);
    game::get_pois_for_position(db, node_id)
}

#[allow(unused)]
pub fn get_items(db: &impl ItemEditingDb) -> TmouResult<Vec<api::ItemWithNodes>> {
    todo!()
}
