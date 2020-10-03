use super::api_models as api;
use super::db_models as db;
use super::db_controller::{DbControl};
use std::collections::HashMap;
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
        }
    }
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
