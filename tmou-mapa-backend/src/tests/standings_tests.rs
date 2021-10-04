#[allow(unused_imports)]
use std::cmp::Ordering;

use itertools::Itertools;

#[allow(unused_imports)]
use crate::controllers::standings;
#[allow(unused_imports)]
use crate::models::api;
#[allow(unused_imports)]
use crate::models::db;
#[allow(unused_imports)]
use crate::models::errors::*;
#[allow(unused_imports)]
use super::item;

#[allow(unused)]
fn team(rank: u16, name: &str, ps: Vec<(u16, &str, u32)>) -> api::TeamStanding {
    let badge_count = ps.len() as u16;
    api::TeamStanding {
        rank,
        name: name.to_string(),
        badges: ps
            .into_iter()
            .map(|(level, name, min)| {
                (
                    level as i16,
                    api::PuzzleResult {
                        name: name.to_string(),
                        timestamp: chrono::NaiveDate::from_ymd(2020, 11, 6).and_hms(22, min, 00),
                    },
                )
            })
            .into_group_map(),
        badge_count,
        puzzle_count: 0,
    }
}

#[test]
fn is_better_team_returns_alphabetical_for_empty_teams() -> TmouResult<()> {
    let a = team(0, "Absolutno", Vec::new());
    let b = team(0, "Bazinga", Vec::new());
    assert_eq!(standings::is_better_team(&a, &b), Ordering::Less);
    Ok(())
}

#[test]
fn is_better_team_returns_alphabetical_for_equivalent_teams() -> TmouResult<()> {
    let a = team(
        0,
        "Absolutno",
        vec![
            (0, "badge11", 1),
            (0, "badge12", 17),
            (0, "badge14", 23),
            (1, "badge21", 51),
            (1, "badge22", 59),
        ],
    );
    let b = team(
        0,
        "Bazinga",
        vec![
            (0, "badge11", 1),
            (0, "badge12", 17),
            (0, "badge14", 23),
            (1, "badge21", 51),
            (1, "badge23", 59),
        ],
    );
    assert_eq!(standings::is_better_team(&a, &b), Ordering::Less);
    Ok(())
}

#[test]
fn is_better_team_does_prefer_puzzle_visit_over_nothing() -> TmouResult<()> {
    let a = team(0, "Absolutno", Vec::new());
    let b = team(0, "Bazinga", vec![(1, "badge32", 0)]);
    assert_eq!(standings::is_better_team(&a, &b), Ordering::Greater);
    Ok(())
}

#[test]
fn is_better_team_prefers_faster_team() -> TmouResult<()> {
    let a = team(0, "Absolutno", vec![(0, "badge11", 0), (0, "badge12", 1)]);
    let b = team(0, "Bazinga", vec![(0, "badge12", 0), (0, "badge13", 0)]);
    assert_eq!(standings::is_better_team(&a, &b), Ordering::Greater);
    Ok(())
}

#[test]
fn is_better_team_prefers_faster_team_on_last_solved() -> TmouResult<()> {
    let a = team(
        0,
        "Absolutno",
        vec![
            (0, "badge32", 0),
            (0, "badge32", 1),
            (0, "badge32", 1),
            (1, "badge32", 1),
            (1, "badge32", 1),
            (2, "badge32", 1),
            (3, "badge32", 1),
            (4, "badge32", 1),
        ],
    );
    let b = team(
        0,
        "Bazinga",
        vec![
            (0, "badge32", 0),
            (0, "badge32", 1),
            (0, "badge32", 1),
            (1, "badge32", 1),
            (1, "badge32", 1),
            (2, "badge32", 1),
            (3, "badge32", 2),
            (4, "badge32", 0),
        ],
    );
    assert_eq!(standings::is_better_team(&a, &b), Ordering::Greater);
    Ok(())
}

#[test]
fn is_better_team_prefers_team_with_more_solved() -> TmouResult<()> {
    let a = team(
        0,
        "Absolutno",
        vec![
            (0, "badge32", 1),
            (0, "badge32", 1),
            (0, "badge32", 1),
            (1, "badge32", 1),
            (1, "badge32", 1),
        ],
    );
    let b = team(
        0,
        "Bazinga",
        vec![
            (0, "badge32", 1),
            (0, "badge32", 1),
            (0, "badge32", 1),
            (1, "badge32", 1),
        ],
    );
    assert_eq!(standings::is_better_team(&a, &b), Ordering::Less);
    Ok(())
}

////////////////////////////////////////////////////
#[allow(unused)]
fn db_team(name: &str, ps: Vec<(&str, i16, u32)>) -> Vec<db::TeamStandingsItem> {
    match ps.is_empty() {
        true => vec![db::TeamStandingsItem {
            team_name: name.to_string(),
            type_: None,
            level: None,
            name: None,
            description: None,
            timestamp: None,
        }],
        false => ps
            .into_iter()
            .map(|(typ, levl, min)| db::TeamStandingsItem {
                team_name: name.to_string(),
                type_: Some(typ.to_string()),
                level: Some(levl),
                name: Some(typ.to_string()),
                description: Some("item".to_string()),
                timestamp: Some(chrono::NaiveDate::from_ymd(2020, 11, 6).and_hms(22, min, 00)),
            })
            .collect(),
    }
}

#[test]
fn calculate_teams_standings_outputs_2_empty_teams_sorted() -> TmouResult<()> {
    let mut a = db_team("Absolutno", Vec::new());
    let mut b = db_team("Bazinga", Vec::new());
    a.append(&mut b);

    let expected = vec![
        team(1, "Absolutno", Vec::new()),
        team(2, "Bazinga", Vec::new()),
    ];

    let st = standings::calculate_teams_standings(a)?;
    assert_eq!(st.standings, expected);
    Ok(())
}

#[test]
fn calculate_teams_standings_outputs_2_complex_teams_sorted() -> TmouResult<()> {
    let mut a = db_team(
        "Absolutno",
        vec![
            ("badge", 0, 0),
            ("badge", 0, 1),
            ("badge", 0, 2),
            ("badge", 1, 3),
            ("badge", 1, 4),
            ("badge", 2, 5),
            ("badge", 3, 6),
            ("badge", 4, 7),
        ],
    );
    let mut b = db_team(
        "Bazinga",
        vec![
            ("badge", 0, 0),
            ("badge", 0, 1),
            ("badge", 0, 2),
            ("badge", 1, 3),
            ("badge", 1, 4),
            ("badge", 2, 5),
            ("badge", 3, 6),
            ("badge", 4, 6),
        ],
    );
    a.append(&mut b);

    let mut res_1 = team(
        1,
        "Bazinga",
        vec![
            (0,"badge", 0),
            (0,"badge", 1),
            (0,"badge", 2),
            (1,"badge", 3),
            (1,"badge", 4),
            (2,"badge", 5),
            (3,"badge", 6),
            (4,"badge", 6),
        ],
    );

    let mut res_2 = team(
        2,
        "Absolutno",
        vec![
            (0,"badge", 0),
            (0,"badge", 1),
            (0,"badge", 2),
            (1,"badge", 3),
            (1,"badge", 4),
            (2,"badge", 5),
            (3,"badge", 6),
            (4,"badge", 7),
        ],
    );

    let expected = vec![res_1, res_2];

    let st = standings::calculate_teams_standings(a)?;
    assert_eq!(st.standings, expected);
    Ok(())
}

#[test]
fn calculate_teams_standings_outputs_2_complex_teams_sorted_with_ignored_badges() -> TmouResult<()> {
    let mut a = db_team(
        "Absolutno",
        vec![
            ("badge", 0, 0),
            ("badge", 0, 1),
            ("badge", 0, 2),
            ("badge", 0, 9),
            ("badge", 1, 3),
            ("badge", 1, 4),
            ("badge", 1, 3),
            ("badge", 2, 5),
            ("badge", 3, 6),
            ("badge", 4, 7),
        ],
    );
    let mut b = db_team(
        "Bazinga",
        vec![
            ("badge", 0, 0),
            ("badge", 0, 1),
            ("badge", 0, 2),
            ("badge", 1, 3),
            ("badge", 1, 4),
            ("badge", 2, 5),
            ("badge", 3, 6),
            ("badge", 4, 6),
        ],
    );
    a.append(&mut b);

    let mut res_1 = team(
        1,
        "Bazinga",
        vec![
            (0,"badge", 0),
            (0,"badge", 1),
            (0,"badge", 2),
            (1,"badge", 3),
            (1,"badge", 4),
            (2,"badge", 5),
            (3,"badge", 6),
            (4,"badge", 6),
        ],
    );

    let mut res_2 = team(
        2,
        "Absolutno",
        vec![
            (0,"badge", 0),
            (0,"badge", 1),
            (0,"badge", 2),
            (1,"badge", 3),
            (1,"badge", 3),
            (2,"badge", 5),
            (3,"badge", 6),
            (4,"badge", 7),
        ],
    );

    let expected = vec![res_1, res_2];

    let st = standings::calculate_teams_standings(a)?;
    assert_eq!(st.standings, expected);
    Ok(())
}
