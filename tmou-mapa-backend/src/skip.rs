use super::errors::*;

// Function for determination whether the skip is allowed.
//
// Input parameters are current puzzle (level), number of badges and game state - decreasing sum of
// acquired puzzles (level 0 was acquired by all teams, level 1 by 300 teams, level 2 by 200 teams
// etc.)
//
// Returns simple true/false value meaning skips is allowed or not.

pub fn is_allowed(level: usize, badges: usize, game_state: Vec<i32>) -> TmouResult<bool> {
    // check if game_state is correct - number of acquired puzzles per level, i.e. sorted DESC
    let mut rev = game_state.clone();
    rev.reverse();
    if !rev.is_sorted() {
        return Err(TmouError {
            message: "Invalid game state provided.".to_string(),
            response: 500,
        });
    }
    // source https://docs.google.com/spreadsheets/d/1WmDUIM449LT_mrxuotBnOs9MqBU5z2FhK6LawGPz-58/edit#gid=0
    let skips_limits = vec![
        vec![std::i32::MAX], //level 0 is start, cannot be skipped
        vec![300, 200],
        vec![250, 200, 150],
        vec![250, 225, 200, 150],
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

    if level >= skips_limits.len() {
        return Ok(false);
    }

    let skips_for_level = &skips_limits[level];
    let limit = match badges >= skips_for_level.len() {
        true => skips_for_level.last().unwrap_or(&0),
        false => &skips_for_level[badges],
    };

    return Ok(match level + 1 < game_state.len() {
        true => &game_state[level + 1] >= limit,
        false => false,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn level0_cannot_be_skipped() {
        assert_eq!(is_allowed(0, 0, vec!(500, 150, 150)).unwrap(), false);
    }
    #[test]
    fn level1_no_bonuses_few_teams() {
        assert_eq!(is_allowed(1, 0, vec!(500, 500, 100)).unwrap(), false);
    }
    #[test]
    fn level1_1_bonus_few_teams() {
        assert_eq!(is_allowed(1, 1, vec!(500, 500, 100)).unwrap(), false);
    }
    #[test]
    fn level1_no_bonuses_enough_teams() {
        assert_eq!(is_allowed(1, 1, vec!(500, 500, 301)).unwrap(), true);
    }
    #[test]
    fn level1_1_bonus_enough_teams() {
        assert_eq!(is_allowed(1, 1, vec!(500, 500, 201)).unwrap(), true);
    }
    #[test]
    fn level1_more_bonuses_than_defined_enough_teams() {
        assert_eq!(is_allowed(1, 5, vec!(500, 500, 201)).unwrap(), true);
    }
    #[test]
    fn level1_no_team_in_next_level() {
        assert_eq!(is_allowed(1, 0, vec!(500, 500)).unwrap(), false);
    }
    #[test]
    fn non_existing_level() {
        assert_eq!(is_allowed(20, 0, vec!(500, 500)).unwrap(), false);
    }
    #[test]
    fn invalid_game_state() {
        assert_eq!(
            is_allowed(20, 0, vec!(10, 20)).unwrap_err().message,
            "Invalid game state provided.".to_string()
        );
    }
}
