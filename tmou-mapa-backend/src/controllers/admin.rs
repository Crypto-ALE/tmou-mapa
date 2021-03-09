use itertools::*;

use crate::controllers::standings;
use crate::database::db::Db;
use crate::models::errors::*;
use crate::models::*;

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
