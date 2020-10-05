use super::api_models as api;
use super::db_models as db;
use super::db_controller::{DbControl};
use std::collections::HashMap;
use super::errors::*;
use itertools::*;
use chrono::{NaiveDate,NaiveDateTime,NaiveTime};

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

    let potential_badges_per_team = teams_badges_db.iter().map(|tb| (&tb.team_name, match (&tb.badge_name, tb.timestamp) {
        (Some(bn), Some(t)) => Some(Badge {
            badge_name: bn.to_string(),
            badge_timestamp: t,
        }),
        _ => None
    }
    )).into_group_map();

    // Potentially suboptimal, could be concatenated to stuff above, but much more straight forward
    // and readable
    let mut badges_per_team: HashMap<&str, BadgesResults> = HashMap::new();
    for (k,v) in potential_badges_per_team {
        let badges: Vec<Badge> = v.into_iter().filter_map(std::convert::identity).collect();
        let last_badge = badges.iter().max_by_key(|b| b.badge_timestamp).and_then(|b| Some(b.badge_timestamp));
        badges_per_team.insert(k, BadgesResults {
            badges,
            last_badge,
        });
    }

    let sorted_badges_per_team = badges_per_team.iter()
        .sorted_by(|(_,v), (_,w)| {
            match v.badges.len().cmp(&w.badges.len())
            {
                std::cmp::Ordering::Equal => v.last_badge.cmp(&w.last_badge),
                o => o.reverse()
            }
        });

    let standings: Vec<api::TeamStanding> = sorted_badges_per_team.enumerate().map(|(i, (k,v))| {
        api::TeamStanding {
            rank: (i+1) as u16,
            name: k.to_string(),
            badge_timestamps: v.badges.iter().map(|b: &Badge| {(b.badge_name.to_string(), b.badge_timestamp)}).collect()
        }
    }).collect();

    Ok(api::Standings{badge_labels: badge_labels_db, standings})
}

pub fn get_puzzles_stats(db_control: & impl DbControl) -> TmouResult<api::PuzzlesStats>
{
    let badges_teams = db_control.get_badges_teams()?;

    let badges_grouped = badges_teams.iter().map(|bt| (&bt.badge_name, match (&bt.team_name, bt.timestamp) {
        (Some(tn), Some(t)) => Some(BadgeSolved {
            team_name: tn.to_string(),
            timestamp: t,
        }),
        _ => None
    }
    )).into_group_map();

    let puzzles:Vec<api::PuzzleStats> = badges_grouped.into_iter().map(badge_group_to_badge_stats).collect();

    Ok(api::PuzzlesStats{puzzles})
}

////////////////////////////////////////////////////////////////////
/// Implementation details
////////////////////////////////////////////////////////////////////

fn badge_group_to_badge_stats(group: (&String, Vec<Option<BadgeSolved>>)) -> api::PuzzleStats
{
    let (n, opt_vec) = group;
    let vec: Vec<BadgeSolved> = opt_vec.into_iter().filter(|bs| bs.is_some()).map(|b| b.unwrap()).collect();
    let solved_by = vec.len();
    let max_date_time = NaiveDate::from_ymd(3000, 1, 1).and_hms(0, 0, 0);
    let max_badge_solved = BadgeSolved{team_name: String::from(""), timestamp: max_date_time};
    let first = vec.iter().fold(&max_badge_solved,|a, b| if a.timestamp < b.timestamp {a} else {b} );

    let (first_team, first_time) = if (first.team_name == "") { (None, None) } 
    else { (Some(first.team_name.clone()), Some(first.timestamp.time())) };

    api::PuzzleStats{name: n.clone(), solved_by, first_team, first_time, fastest_time: None, fastest_team: None, median_time: None}
}


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

#[derive(Debug)]
struct BadgeSolved {
    team_name: String,
    timestamp: chrono::NaiveDateTime,
}



#[derive(Debug)]
struct Badge {
    badge_name: String,
    badge_timestamp: chrono::NaiveDateTime,
}

#[derive(Debug)]
struct BadgesResults {
    badges: Vec<Badge>,
    last_badge: Option<chrono::NaiveDateTime>
}
