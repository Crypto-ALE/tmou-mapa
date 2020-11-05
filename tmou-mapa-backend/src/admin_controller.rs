use super::api_models as api;
use super::db_models as db;
use super::db_controller::{DbControl};
use std::collections::{HashMap,HashSet};
use std::cmp::Ordering;
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

struct ResultItem
{
    pub type_: String, // puzzles | badge | message
    pub level: i16,
    pub name: String,
    pub description: String,
    pub timestamp: chrono::NaiveDateTime
}

fn item_vec_to_result_map(name: String, its: Vec<ResultItem>) -> api::TeamStanding
{
    // partition to puzzles, deads, badges; throw away rest (puzzle-fakes)
    let (puzzles, non_puzzles) : (Vec<ResultItem>, Vec<ResultItem>) = 
        its.into_iter().partition(|i| i.type_.eq("puzzles"));
    let (deads, non_deads) : (Vec<ResultItem>, Vec<ResultItem>)  = 
        non_puzzles.into_iter().partition(|i| i.type_.eq("dead"));
    let (badges, _) : (Vec<ResultItem>, Vec<ResultItem>)  = 
        non_deads.into_iter().partition(|i| i.type_.eq("badge"));
    // badge count is trivial
    let badge_count = badges.len() as u16;
    // convert dead vector to set of dead levels for better use
    let dead_set:HashSet<i16> = deads.into_iter().map(|d| d.level).collect();
    
    let puzzles:HashMap<u16,api::PuzzleResult> = puzzles.into_iter()
        // take all except start
        .filter(|p| p.level > 0)
        // mark them solved if dead set does not contain its level
        .map(|p| (p.level as u16, {
            let solved = !dead_set.contains(&p.level);
            api::PuzzleResult{solved, timestamp: p.timestamp}
            })).collect();
    api::TeamStanding{rank: 0,name, puzzles, badge_count} 
}

fn solved_puzzles_count(ts: &api::TeamStanding) -> usize
{
    ts.puzzles.iter().filter(|(_,v)| v.solved).count()
}

fn highest_solved_level(ts: &api::TeamStanding) -> Option<(u16, chrono::NaiveDateTime)>
{
    // fortunately, HashMap is O(1)
    // take all puzzles on level k such that 
    // there is a puzzle on level k-1 and it is solved (no dead on previous)
    // then select maximum level of such puzzle, and create pair level, timestamp
    ts.puzzles.iter()
      .filter(|(k,_)| ts.puzzles.get(&(*k - 1))
                       .and_then(|p| Some(p.solved))
                       .or(Some(false))
                       .unwrap())
      .max_by_key(|(k,_)| *k)
      .and_then(|(k,v)| Some((*k, v.timestamp)))
}

pub fn is_better_team(l: &api::TeamStanding, r: &api::TeamStanding) -> Ordering
{
    match solved_puzzles_count(l).cmp(&solved_puzzles_count(r))
    {
        // more puzzles lower raning
        Ordering::Greater => Ordering::Less,
        Ordering::Less => Ordering::Greater,
        _ => {
            let l_hio = highest_solved_level(l);
            let r_hio = highest_solved_level(r);
            match (l_hio, r_hio)
            {
                (None, None) => l.name.cmp(&r.name), // alphabetical
                (Some(_), None) => Ordering::Less, // something is always better     
                (None, Some(_)) => Ordering::Greater,
                (Some(l_hi), Some(r_hi)) => match l_hi.0.cmp(&r_hi.0) {
                    // higher level lower ranking
                    Ordering::Greater => Ordering::Less,
                    Ordering::Less => Ordering::Greater,
                    _ => match l_hi.1.cmp(&r_hi.1) {
                        // lower time lower ranking
                        Ordering::Less => Ordering::Less,
                        Ordering::Greater => Ordering::Greater,
                        // alphabetical
                        _ => l.name.cmp(&r.name)
        
                    }

                }
            }
        }
    }
}

pub fn get_teams_standings_impl(teams_items: Vec<db::TeamStandingsItem>) -> TmouResult<api::Standings>
{
    // convert to ResultItem
    let items = teams_items.into_iter().map(|t| (t.team_name, match t.type_
        {
            Some(_) => Some(ResultItem{
                type_:t.type_.unwrap(),
                level:t.level.unwrap(),
                name:t.name.unwrap(),
                description:t.description.unwrap(),
                timestamp: t.timestamp.unwrap()
                }),
            None => None
        }));

    // group by team
    let items_per_team = items.into_iter().into_group_map();
    // remove Some and None
    let items_per_team_normalized:HashMap<_, _> = items_per_team.into_iter()
        .map(|(k,v)| (k, v.into_iter()
                          .filter_map(|i| i)
                          .collect()))
        .collect();
    // collect to vector and sort in place according to criteria
    let mut res:Vec<_> = items_per_team_normalized.into_iter().map(|(k,v)| item_vec_to_result_map(k, v)).collect();
    // suboptimal - the criteria should be computed ahead of sorting for every team
    res.sort_by(|a, b| is_better_team(a, b));
    // add rankings
    let standings = res.into_iter().enumerate().map(|(i, v)| api::TeamStanding{
        rank: (i+1) as u16,
        name: v.name,
        puzzles: v.puzzles,
        badge_count: v.badge_count
    }).collect();
    Ok(api::Standings{standings})
}

pub fn get_teams_standings(db_control: & impl DbControl) -> TmouResult<api::Standings>
{
    let teams_items_db = db_control.get_teams_items()?;
    get_teams_standings_impl(teams_items_db)
}

type StatVec = (Item, Vec<Received>);
type HashOfStatVec = HashMap<Item, Vec<Received>>;
type HashOfReceived = HashMap<String, Received>;

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
fn to_str(d: &chrono::Duration) -> String
{
    let h = d.num_hours();
    let m = d.num_minutes() - 60 * d.num_hours();
    let s = d.num_seconds() - 60 * d.num_minutes();
    let t = d.num_milliseconds() - 1000 * d.num_seconds();

    format!("{}:{}:{}.{}", h, m, s, t)
}

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
