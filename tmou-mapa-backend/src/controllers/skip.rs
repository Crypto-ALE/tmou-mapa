use crate::models::db::Item;
use crate::models::errors::*;
use itertools::*;
use lazy_static::lazy_static;

// Function for determination whether the skip is allowed.
//
// Input parameters are current puzzle (level), number of badges and game state - decreasing sum of
// acquired puzzles (level 0 was acquired by all teams, level 1 by 300 teams, level 2 by 200 teams
// etc.)
//
// Returns simple true/false value meaning skips is allowed or not.
//
// Assumption: there is level 0, which is start. If not, whole functionality needs to be shifted

// skip limits table
// source https://docs.google.com/spreadsheets/d/1WmDUIM449LT_mrxuotBnOs9MqBU5z2FhK6LawGPz-58/edit#gid=0
lazy_static! {
static ref SKIPS_LIMITS: Vec<Vec<i64>> = vec![
    vec![std::i64::MAX], //level 0 is start, cannot be skipped
    vec![250, 200],
    vec![250, 200, 175],
    vec![250, 225, 175, 150],
    vec![200, 180, 160, 140, 120],
    vec![200, 180, 160, 140, 120, 100],
    vec![200, 180, 160, 140, 120, 100, 80],
    vec![200, 180, 160, 140, 120, 100, 80, 75],
    vec![150, 140, 130, 120, 110, 100, 90, 80, 70],
    vec![150, 140, 130, 120, 110, 100, 90, 80, 70, 65],
    vec![150, 140, 130, 120, 110, 100, 90, 80, 70, 65, 60],
    vec![150, 140, 130, 120, 110, 100, 90, 80, 70, 60, 50],
    vec![100, 90, 80, 70, 60, 55, 50, 45, 40, 35, 30],
    vec![50, 48, 45, 42, 40, 38, 35, 33, 30, 28, 25],
    vec![20, 18, 16, 14, 12, 10, 9, 8, 7, 6, 5],
];
}

fn contains_dead_for_level(items: Vec<Item>, level: i16) -> bool {
    items
        .iter()
        .any(|i| i.type_ == "dead".to_string() && i.level == level)
}

pub fn is_allowed(items: Vec<Item>, game_state: Vec<i64>) -> TmouResult<bool> {
    let grouped_items = items
        .iter()
        .map(|item| (item.type_.clone(), item))
        .into_group_map();
    let empty = Vec::new();
    let puzzles = grouped_items.get("puzzles").unwrap_or(&empty);
    let puzzles_fake = grouped_items.get("puzzles-fake").unwrap_or(&empty);
    let level = puzzles
        .iter()
        .chain(puzzles_fake)
        .map(|item| item.level as usize)
        .max()
        .unwrap_or(0);
    let badges_count = grouped_items
        .get("badge")
        .and_then(|bdgs| Some(bdgs.len()))
        .unwrap_or(0);

    // Check if team already has a dead for current level
    if contains_dead_for_level(items, level as i16) {
        return Ok(false);
    }

    // check if game_state is correct - number of acquired puzzles per level, i.e. sorted DESC
    let mut rev = game_state.clone();
    rev.reverse();
    if !rev.is_sorted() {
        return Err(TmouError {
            message: "Invalid game state provided.".to_string(),
            response: 500,
        });
    }

    if level >= SKIPS_LIMITS.len() {
        return Ok(false);
    }

    let skips_for_level = &SKIPS_LIMITS[level];
    let limit = match badges_count >= skips_for_level.len() {
        true => skips_for_level.last().unwrap_or(&0),
        false => &skips_for_level[badges_count],
    };

    return Ok(match level + 1 < game_state.len() {
        true => &game_state[level + 1] >= limit,
        false => false,
    });
}

pub fn get_skips_limits(level: usize) -> Option<Vec<i64>> {
    match level {
        0 => None,
        x if x >= SKIPS_LIMITS.len() => None,
        x => Some(SKIPS_LIMITS[x].clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn level0_cannot_be_skipped() {
        let inventory = vec![Item {
            level: 0,
            description: None,
            type_: "puzzles".to_string(),
            name: "puzzle".to_string(),
            url: "".to_string(),
        }];
        assert_eq!(is_allowed(inventory, vec!(500, 150, 150)).unwrap(), false);
    }
    #[test]
    fn level1_no_bonuses_few_teams() {
        let inventory = vec![Item {
            level: 0,
            description: None,
            type_: "puzzles".to_string(),
            name: "puzzle".to_string(),
            url: "".to_string(),
        }];
        assert_eq!(is_allowed(inventory, vec!(500, 500, 100)).unwrap(), false);
    }
    #[test]
    fn level1_1_bonus_few_teams() {
        let inventory = vec![Item {
            level: 1,
            description: None,
            type_: "puzzles".to_string(),
            name: "puzzle".to_string(),
            url: "".to_string(),
        },Item {
            level: -1,
            description: None,
            type_: "badge".to_string(),
            name: "badge".to_string(),
            url: "".to_string(),
        }];
        assert_eq!(is_allowed(inventory, vec!(500, 500, 100)).unwrap(), false);
    }
    #[test]
    fn level1_no_bonuses_enough_teams() {
        let inventory = vec![Item {
            level: 1,
            description: None,
            type_: "puzzles".to_string(),
            name: "puzzle".to_string(),
            url: "".to_string(),
        }];
        assert_eq!(is_allowed(inventory, vec!(500, 500, 301)).unwrap(), true);
    }
    #[test]
    fn fake_level1_no_bonuses_enough_teams() {
        let inventory = vec![Item {
            level: 1,
            description: None,
            type_: "puzzles-fake".to_string(),
            name: "puzzle".to_string(),
            url: "".to_string(),
        }];
        assert_eq!(is_allowed(inventory, vec!(500, 500, 301)).unwrap(), true);
    }
    #[test]
    fn level1_no_bonuses_enough_teams_already_dead() {
        let inventory = vec![Item {
            level: 1,
            description: None,
            type_: "puzzles".to_string(),
            name: "puzzle".to_string(),
            url: "".to_string(),
        },Item {
            level: 1,
            description: None,
            type_: "dead".to_string(),
            name: "dead".to_string(),
            url: "".to_string(),
        }];
        assert_eq!(is_allowed(inventory, vec!(500, 500, 301)).unwrap(), false);
    }
    #[test]
    fn level1_1_bonus_enough_teams() {
        let inventory = vec![Item {
            level: 1,
            description: None,
            type_: "puzzles".to_string(),
            name: "puzzle".to_string(),
            url: "".to_string(),
        },Item {
            level: -1,
            description: None,
            type_: "badge".to_string(),
            name: "badge".to_string(),
            url: "".to_string(),
        }];
        assert_eq!(is_allowed(inventory, vec!(500, 500, 201)).unwrap(), true);
    }
    #[test]
    fn level1_more_bonuses_than_defined_enough_teams() {
        let inventory = vec![Item {
            level: 1,
            description: None,
            type_: "puzzles".to_string(),
            name: "puzzle".to_string(),
            url: "".to_string(),
        },Item {
            level: -1,
            description: None,
            type_: "badge".to_string(),
            name: "badge".to_string(),
            url: "".to_string(),
        },Item {
            level: -1,
            description: None,
            type_: "badge".to_string(),
            name: "badge".to_string(),
            url: "".to_string(),
        },Item {
            level: -1,
            description: None,
            type_: "badge".to_string(),
            name: "badge".to_string(),
            url: "".to_string(),
        }];
        assert_eq!(is_allowed(inventory, vec!(500, 500, 201)).unwrap(), true);
    }
    #[test]
    fn level1_no_team_in_next_level() {
        let inventory = vec![Item {
            level: 0,
            description: None,
            type_: "puzzles".to_string(),
            name: "puzzle".to_string(),
            url: "".to_string(),
        }];
        assert_eq!(is_allowed(inventory, vec!(500, 500)).unwrap(), false);
    }
    #[test]
    fn non_existing_level() {
        let inventory = vec![Item {
            level: 0,
            description: None,
            type_: "puzzles".to_string(),
            name: "puzzle".to_string(),
            url: "".to_string(),
        }];
        assert_eq!(is_allowed(inventory, vec!(500, 500)).unwrap(), false);
    }
    #[test]
    fn invalid_game_state() {
        let inventory = vec![Item {
            level: 0,
            description: None,
            type_: "puzzles".to_string(),
            name: "puzzle".to_string(),
            url: "".to_string(),
        }];
        assert_eq!(
            is_allowed(inventory, vec!(10, 20)).unwrap_err().message,
            "Invalid game state provided.".to_string()
        );
    }
}
