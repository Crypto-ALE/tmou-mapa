use std::collections::{HashMap,HashSet};
use std::cmp::Ordering;

use itertools::*;

use crate::models::api as api;
use crate::models::db as db;
use crate::models::errors::*;


struct ResultItem
{
    pub type_: String, // puzzles | badge | message
    pub level: i16,
    pub name: String,
    pub description: String,
    pub timestamp: chrono::NaiveDateTime
}

fn item_vec_to_standing(name: String, its: Vec<ResultItem>) -> api::TeamStanding
{
    // partition to puzzles, deads, badges; throw away rest (puzzle-fakes)
    let (mut puzzles_vec, non_puzzles) : (Vec<ResultItem>, Vec<ResultItem>) = 
        its.into_iter().partition(|i| i.type_.eq("puzzles"));
    let (deads, non_deads) : (Vec<ResultItem>, Vec<ResultItem>)  = 
        non_puzzles.into_iter().partition(|i| i.type_.eq("dead"));
    let (badges, _) : (Vec<ResultItem>, Vec<ResultItem>)  = 
        non_deads.into_iter().partition(|i| i.type_.eq("badge"));
    // badge count is trivial
    let badge_count = badges.len() as u16;
    // so is start puzzles count
    let start_puzzles_solved = puzzles_vec.iter().filter(|p| p.level == 1).count() as u16;
    // convert dead vector to set of dead levels for better use
    let dead_set:HashSet<i16> = deads.into_iter().map(|d| d.level).collect();
    // sort puzzles by time descending so that last puzzle (earliest) with the same level ends up in the hash map
    puzzles_vec.sort_by(|a,b| b.timestamp.cmp(&a.timestamp));
    let puzzles:HashMap<u16,api::PuzzleResult> = puzzles_vec.into_iter()
        // take all except start
        //.filter(|p| p.level > 0)
        // set dead if dead set does not contain its level
        .map(|p| (p.level as u16, {
            let dead = dead_set.contains(&p.level);
            api::PuzzleResult{dead, timestamp: p.timestamp}
            })).collect();
    api::TeamStanding{rank: 0,name, puzzles, badge_count, start_puzzles_solved} 
}

fn solved_puzzles_count(ts: &api::TeamStanding) -> usize
{
    ts.puzzles.iter()
      .filter(|(k,_)| ts.puzzles.get(&(*k - 1))
                       .and_then(|p| Some(!p.dead))
                       .or(Some(false))
                       .unwrap())
      .count()
}

fn highest_solved_level(ts: &api::TeamStanding) -> Option<(u16, chrono::NaiveDateTime)>
{
    // fortunately, HashMap is O(1)
    // take all puzzles on level k such that 
    // there is a puzzle on level k-1 and it is solved (no dead on previous)
    // then select maximum level of such puzzle, and create pair level, timestamp
    ts.puzzles.iter()
      .filter(|(k,_)| ts.puzzles.get(&(*k - 1))
                       .and_then(|p| Some(!p.dead))
                       .or(Some(false))
                       .unwrap())
      .max_by_key(|(k,_)| *k)
      .and_then(|(k,v)| Some((*k, v.timestamp)))
}

pub fn is_better_team(l: &api::TeamStanding, r: &api::TeamStanding) -> Ordering
{
    match solved_puzzles_count(l).cmp(&solved_puzzles_count(r))
    {
        // more puzzles lower ranking
        Ordering::Greater => Ordering::Less,
        Ordering::Less => Ordering::Greater,
        _ => {
            let l_hio = highest_solved_level(l);
            let r_hio = highest_solved_level(r);
            match (l_hio, r_hio)
            {
                (None, None) => l.name.cmp(&r.name), // neither solved anything: alphabetical
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

pub fn calculate_teams_standings(teams_items: Vec<db::TeamStandingsItem>) -> TmouResult<api::Standings>
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
    let mut res:Vec<_> = items_per_team_normalized.into_iter().map(|(k,v)| item_vec_to_standing(k, v)).collect();
    // suboptimal - the criteria should be computed ahead of sorting for every team
    res.sort_by(|a, b| is_better_team(a, b));
    // add rankings
    let standings = res.into_iter().enumerate().map(|(i, v)| api::TeamStanding{
        rank: (i+1) as u16,
        name: v.name,
        puzzles: v.puzzles,
        badge_count: v.badge_count,
        start_puzzles_solved: v.start_puzzles_solved
    }).collect();
    Ok(api::Standings{standings})
}
