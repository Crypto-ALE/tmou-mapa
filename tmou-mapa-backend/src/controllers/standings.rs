use std::cmp::Ordering;
use std::collections::{HashMap};

use itertools::*;

use crate::models::api;
use crate::models::db;
use crate::models::errors::*;

struct ResultItem {
    pub type_: String, // puzzles | badge | message
    pub level: i16,
    pub name: String,
    pub description: String,
    pub timestamp: chrono::NaiveDateTime,
}

fn item_vec_to_standing(name: String, its: Vec<ResultItem>) -> api::TeamStanding {
    // partition to puzzles, deads, badges; throw away rest (puzzle-fakes)
    let (mut puzzles_vec, non_puzzles): (Vec<ResultItem>, Vec<ResultItem>) =
        its.into_iter().partition(|i| i.type_.eq("puzzles"));
    // badge count is trivial
    let badge_count = 0;
    // so is start puzzles count
    let start_puzzles_solved = 0;
    // sort puzzles by time descending so that last puzzle (earliest) with the same level ends up in the hash map
    puzzles_vec.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    let puzzles: HashMap<String, api::PuzzleResult> = puzzles_vec
        .into_iter()
        // take all except start
        //.filter(|p| p.level > 0)
        // set dead if dead set does not contain its level
        .map(|p| {
            (p.name, {
                api::PuzzleResult {
                    dead: false,
                    timestamp: p.timestamp,
                }
            })
        })
        .collect();
    api::TeamStanding {
        rank: 0,
        name,
        puzzles,
        badge_count,
        start_puzzles_solved,
    }
}

fn solved_puzzles_count(ts: &api::TeamStanding) -> usize {
    ts.puzzles
        .iter()
        .count()
}

pub fn is_better_team(l: &api::TeamStanding, r: &api::TeamStanding) -> Ordering {
    match solved_puzzles_count(l).cmp(&solved_puzzles_count(r)) {
        Ordering::Greater => Ordering::Less,
        Ordering::Less => Ordering::Greater,
        _ => Ordering::Equal
    }
}

pub fn calculate_teams_standings(
    teams_items: Vec<db::TeamStandingsItem>,
) -> TmouResult<api::Standings> {
    // convert to ResultItem
    let items = teams_items.into_iter().map(|t| {
        (
            t.team_name,
            match t.type_ {
                Some(_) => Some(ResultItem {
                    type_: t.type_.unwrap(),
                    level: t.level.unwrap(),
                    name: t.name.unwrap(),
                    description: t.description.unwrap(),
                    timestamp: t.timestamp.unwrap(),
                }),
                None => None,
            },
        )
    });

    // group by team
    let items_per_team = items.into_iter().into_group_map();
    // remove Some and None
    let items_per_team_normalized: HashMap<_, _> = items_per_team
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().filter_map(|i| i).collect()))
        .collect();
    // collect to vector and sort in place according to criteria
    let mut res: Vec<_> = items_per_team_normalized
        .into_iter()
        .map(|(k, v)| item_vec_to_standing(k, v))
        .collect();
    // suboptimal - the criteria should be computed ahead of sorting for every team
    res.sort_by(|a, b| is_better_team(a, b));
    // add rankings
    let standings = res
        .into_iter()
        .enumerate()
        .map(|(i, v)| api::TeamStanding {
            rank: (i + 1) as u16,
            name: v.name,
            puzzles: v.puzzles,
            badge_count: v.badge_count,
            start_puzzles_solved: v.start_puzzles_solved,
        })
        .collect();
    Ok(api::Standings { standings })
}
