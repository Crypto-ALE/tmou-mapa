use std::collections::{HashMap};
use itertools::*;

use crate::controllers::standings;
use crate::models::*;
use crate::models::errors::*;
use crate::database::db::{DbControl};

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
    let teams_items_db = db_control.get_teams_items()?;
    standings::calculate_teams_standings(teams_items_db)
}

#[allow(unused)]
type StatVec = (Item, Vec<Received>);
#[allow(unused)]
type HashOfStatVec = HashMap<Item, Vec<Received>>;
#[allow(unused)]
type HashOfReceived = HashMap<String, Received>;

#[allow(unused)]
pub fn get_puzzles_stats(db_control: & impl DbControl) -> TmouResult<api::PuzzlesStats>
{
    todo!();
    /*
    // get all items from db - puzzles and badges
    let items_teams = db_control.get_items_teams()?;

    // group them into hashmap where key is item (name, level, type)
    // and value is a vector of Received structs (team, timestamp)
    let item_map_opt = items_teams.into_iter().map(|it| (
        Item
        {
            item_name: it.item_name,
            item_type: it.item_type,
            item_level: it.item_level
        }
        , match (&it.team_name, it.timestamp) {
        (Some(tn), Some(t)) => Some(Received {
            team_name: tn.to_string(),
            timestamp: t,
        }),
        _ => None
    }
    )).into_group_map();

    // remove Optional, where None becomes empty vector
    let item_map:HashOfStatVec = item_map_opt.into_iter()
      .map(|(k, v)| (k, v.into_iter()
           .filter(|i| i.is_some()).map(|b| b.unwrap())
           .collect()))
      .collect();

    // split items to separate collection of puzzles and badges (e. g. start of solving and finish of solving)
    let (puzzles, badges):(HashOfStatVec, HashOfStatVec) = 
      item_map.into_iter().partition(|(k,_)| k.item_type == String::from("puzzles"));

    // make stats for receiving puzzle stats retrieveable by level number
    let puzzle_vec_map: HashMap<i16, StatVec> = puzzles.into_iter()
        .map(|(p,v)| (p.item_level, (p,v))).collect();

    // here, it gets complicated. Every vector of Received record needs to be also hashed by team name
    // so that in the function below, the particular team's puzzle stat (e. g. when they picked up a puzzle)
    // is easily retrievable
    let puzzle_map: HashMap<i16, HashOfReceived> = puzzle_vec_map.into_iter()
      .map(|(l,(_,v))| 
            (l, 
              v.into_iter()
                   .map(|w| (w.team_name.clone(), w))
                   .collect()
            )
          ).collect();


    // for every badge, call a conversion function
    // send also puzzle of corresponding level:
    // for badge level1, puzzle level1 sent so that the duration can be computed for every team
    let stats:Vec<api::PuzzleStats> = badges.into_iter()
        .map(|b| item_group_to_badge_stats_opt(puzzle_map.get(&b.0.item_level), b)).collect();

    Ok(api::PuzzlesStats{stats})
    */
}

////////////////////////////////////////////////////////////////////
/// Implementation details
////////////////////////////////////////////////////////////////////

// I need to format duration to string myself
// ;-(
#[allow(unused)]
fn to_str(d: &chrono::Duration) -> String
{
    let h = d.num_hours();
    let m = d.num_minutes() - 60 * d.num_hours();
    let s = d.num_seconds() - 60 * d.num_minutes();
    let t = d.num_milliseconds() - 1000 * d.num_seconds();

    format!("{}:{}:{}.{}", h, m, s, t)
}

#[allow(unused)]
fn item_group_to_badge_stats(puzzles: &HashOfReceived, badges: StatVec) -> api::PuzzleStats
{
    todo!();
    /*
    // number of solved is number of badges
    let solved_by = badges.1.len();

    // first team is min from badges, trivial
    // TODO: factor out MIN algorithm implemented using fold, into templated fn
    //       or, wait for fold_first to become stable (more readable)
    let max_badge_received = Received{team_name: String::new(), timestamp:chrono::naive::MAX_DATETIME};
    let first = badges.1.iter().fold(&max_badge_received,|a, b| if a.timestamp < b.timestamp {a} else {b} );
    let (first_team, first_time) = if first.team_name.is_empty() { (None, None) } 
                                   else { (Some(first.team_name.clone()), Some(first.timestamp)) };

    // fastest is min from difference between getting the badge (solving) and getting the puzzle (starting)
    // so we need to calculate deltas
    let times: Vec<Solved> = badges.1.iter().map(|b| Solved{ 
        team_name: b.team_name.clone(),
        time: match puzzles.get(&b.team_name) 
        {
            Some(p) => b.timestamp - p.timestamp,
            None => chrono::Duration::max_value() // when no corresponding puzzle record found - should not happen
        }
    }
    ).collect();

    // MIN in terms of fold again
    let max_badge_solved = Solved{team_name: String::new(), time:chrono::Duration::max_value()};
    let fastest = times.iter().fold(&max_badge_solved,|a, b| if a.time < b.time {a} else {b} );
    let (fastest_team, fastest_time) = if fastest.team_name.is_empty() { (None, None) } 
                                   else { (Some(fastest.team_name.clone()), Some(to_str(&fastest.time))) };

    api::PuzzleStats{name: badges.0.item_name.clone(), solved_by, first_team, first_time, fastest_time, fastest_team, median_time: None}
    */
}

// helper function that strips off Optional puzzle records
// only happens for unsolved puzzles
#[allow(unused)]
fn item_group_to_badge_stats_opt(puzzles: Option<&HashOfReceived>, badges: StatVec) -> api::PuzzleStats
{
    todo!()
    /*
    match puzzles
    {
        Some(p) => item_group_to_badge_stats(p, badges),
        None => api::PuzzleStats
          {name: badges.0.item_name.clone(), 
            solved_by:0, first_team: None, first_time:None, fastest_time: None, fastest_team: None, median_time: None}
    }
    */
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

#[derive(Debug,Hash,PartialEq,Eq)]
struct Item {
    pub item_name: String,
    pub item_type: String,
    pub item_level: i16,
}


// helper sturcts for puzzle stats
#[derive(Debug)]
struct Received {
    team_name: String,
    timestamp: chrono::NaiveDateTime,
}

#[derive(Debug)]
struct Solved {
    team_name: String,
    time: chrono::Duration,
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
