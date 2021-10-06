use std::cmp::Ordering;
use std::collections::{HashMap};

use itertools::*;

use crate::models::api;
use crate::models::db;
use crate::models::errors::*;

struct ResultItem {
    pub type_: String, // puzzles | badge | message
    pub level: i16,
    pub timestamp: chrono::NaiveDateTime,
    pub name: String,
}

fn item_vec_to_standing(name: String, its: Vec<ResultItem>) -> api::TeamStanding {
    // partition to puzzles, badges; throw away rest (puzzle-fakes)
    let (puzzles, non_puzzles): (Vec<ResultItem>, Vec<ResultItem>) =
        its.into_iter().partition(|i| i.type_.eq("puzzles"));
    let (badges, _): (Vec<ResultItem>, Vec<ResultItem>) =
        non_puzzles.into_iter().partition(|i| i.type_.eq("badge") && i.name.starts_with("badge"));



    // group badges per level
    let badges_group = badges.into_iter().map(|b| (b.level, b)).into_group_map();
    // sort badges by time descending so that last puzzle (earliest) with the same level ends up in the hash map
    let badges_group_f: HashMap<i16, Vec<ResultItem>> = badges_group
        .into_iter()
        .map(|(l, mut bg)| {bg.sort_by(|a, b| a.timestamp.cmp(&b.timestamp)); (l, bg)})
        .map(|(l, mut bg)| { 
            match (l, bg.len()) {
            (0,4) | (1,3) | (2,2) => {bg.pop(); (l, bg)},
            _ => (l, bg),
        }
    }).collect();
    let badges_filtered: HashMap<i16, Vec<api::PuzzleResult>> = badges_group_f
        .into_iter()
        .map(|(l,bg)| (l,bg
            .into_iter()
            .map(|b| api::PuzzleResult{name: b.name, timestamp: b.timestamp})
            .collect())
        )
        .collect();
        let badge_count = badges_filtered.values().fold(0, |acc, cur| acc + cur.len());

    api::TeamStanding {
        rank: 0,
        name,
        badges: badges_filtered,
        badge_count: badge_count as u16,
        puzzle_count: puzzles.len() as u16,
    }
}

// fn solved_puzzles_count(ts: &api::TeamStanding) -> usize {
//     ts.puzzles
//         .iter()
//         .filter(|(k, _)| {
//             ts.puzzles
//                 .get(&(*k - 1))
//                 .and_then(|p| Some(!p.dead))
//                 .or(Some(false))
//                 .unwrap()
//         })
//         .count()
// }

fn last_badge_timestamp(ts: &api::TeamStanding) -> Option<(i16, chrono::NaiveDateTime)> {
    // take all badges from maximum level
    // find latest timestamp
    ts.badges
        .iter()
        .max_by_key(|(k,_)| *k)
        .and_then(|(k, v)| v.into_iter().max_by_key(|v| v.timestamp).and_then(|x| Some((*k,x.timestamp))))
}

pub fn is_better_team(l: &api::TeamStanding, r: &api::TeamStanding) -> Ordering {
    match l.badge_count.cmp(&r.badge_count) {
        // more badges, better => lower ranking
        Ordering::Greater => Ordering::Less, 
        Ordering::Less => Ordering::Greater,
        _ => {
            match l.puzzle_count.cmp(&r.puzzle_count) {
                // more puzzles, better => lower ranking
                Ordering::Greater => Ordering::Less, 
                Ordering::Less => Ordering::Greater,
                _ => {
                    let l_timestamp = last_badge_timestamp(l);
                    let r_timestamp = last_badge_timestamp(r);
                    match (l_timestamp, r_timestamp) {
                        (None, None) => l.name.cmp(&r.name), // neither solved anything: alphabetical
                        (Some(_), None) => Ordering::Less,   // something is always better
                        (None, Some(_)) => Ordering::Greater,
                        (Some(l_hi), Some(r_hi)) => match l_hi.1.cmp(&r_hi.1) {
                            // lower time lower ranking
                            Ordering::Less => Ordering::Less,
                            Ordering::Greater => Ordering::Greater,
                            // same time (impossible), alphabetical ranking
                            _ => l.name.cmp(&r.name),
                        },
                    }
                },
            }
        }
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
            badges: v.badges,
            badge_count: v.badge_count,
            puzzle_count: v.puzzle_count,
        })
        .collect();
    Ok(api::Standings { standings })
}
