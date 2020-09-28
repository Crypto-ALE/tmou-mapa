use super::api_models as api;
use super::db_models as db;
use super::db_controller::{DbControl};
use super::errors::*;
use itertools::*;

////////////////////////////////////////////////////////////////////
/// Interface
////////////////////////////////////////////////////////////////////

pub fn get_teams_positions(db_control: & impl DbControl) -> TmouResult<Vec<api::TeamPosition>>
{
    let teams_poistions = db_control.get_teams_positions()?;
    Ok(teams_poistions.iter().map_into().collect())
}

pub fn unwrap_incoming_message(db_control: & impl DbControl, message: api::IncomingMessage) -> TmouResult<(db::Team, api::Message)> {
    let inner_message = message.message;
    db_control.get_team(message.recipient_id)
        .and_then(|team| Some((team, inner_message)))
        .ok_or(TmouError {
            message: format!("Team with id {} not found.", message.recipient_id),
            response: 400
    })
}

pub fn get_teams_standings(db_control: & impl DbControl) -> TmouResult<api::Standings>
{
    let badge_labels_db = db_control.get_badge_labels()?;
    let teams_badges_db = db_control.get_teams_badges()?;

    dbg!(badge_labels_db, teams_badges_db);
    todo!();
    //let badge_labels = badge_labels_db.iter().map_into().collect();
    //let standings = teams_badges_db.iter().map_into().collect();
    //Ok(api::Standings{badge_labels, standings})
}

////////////////////////////////////////////////////////////////////
/// Implementation details
////////////////////////////////////////////////////////////////////



impl From<&db::TeamPosition> for api::TeamPosition
{

    fn from(value: &db::TeamPosition) -> Self
    {
        api::TeamPosition{
            team_name: value.team_name.clone(),
            lat: value.lat,
            lon: value.lon,
            level: value.level.unwrap_or(0),
        }
    }
}
