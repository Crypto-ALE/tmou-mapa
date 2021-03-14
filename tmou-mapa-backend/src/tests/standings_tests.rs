#[allow(unused_imports)]
use std::cmp::Ordering;

#[allow(unused_imports)]
use crate::controllers::standings;
#[allow(unused_imports)]
use crate::models::api;
#[allow(unused_imports)]
use crate::models::db;
#[allow(unused_imports)]
use crate::models::errors::*;
#[allow(unused_imports)]
#[allow(unused)]
fn item(t: &str, l: i16, n: &str) -> db::Item {
    db::Item {
        type_: t.to_string(),
        url: "Dummy".to_string(),
        level: l,
        name: n.to_string(),
        description: None,
        condition: None,
    }
}

#[allow(unused)]
fn team(rank: u16, name: &str, ps: Vec<(u16, bool, u32)>) -> api::TeamStanding {
    let start_puzzles_solved = ps.iter().filter(|(level, _, _)| *level == 1).count() as u16;
    api::TeamStanding {
        rank: rank,
        name: name.to_string(),
        puzzles: ps
            .into_iter()
            .map(|(level, dead, min)| {
                (
                    level,
                    api::PuzzleResult {
                        dead: dead,
                        timestamp: chrono::NaiveDate::from_ymd(2020, 11, 6).and_hms(22, min, 00),
                    },
                )
            })
            .collect(),
        badge_count: 0,
        start_puzzles_solved,
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
            (1, true, 1),
            (2, true, 17),
            (3, true, 23),
            (4, false, 51),
            (5, true, 59),
        ],
    );
    let b = team(
        0,
        "Bazinga",
        vec![
            (1, true, 1),
            (2, true, 17),
            (3, true, 23),
            (4, false, 51),
            (5, true, 59),
        ],
    );
    assert_eq!(standings::is_better_team(&a, &b), Ordering::Less);
    Ok(())
}

#[test]
fn is_better_team_does_not_prefer_puzzle_visit_over_nothing() -> TmouResult<()> {
    let a = team(0, "Absolutno", Vec::new());
    let b = team(0, "Bazinga", vec![(1, false, 0)]);
    assert_eq!(standings::is_better_team(&a, &b), Ordering::Less);
    Ok(())
}

#[test]
fn is_better_team_does_not_prefer_dead() -> TmouResult<()> {
    let a = team(0, "Absolutno", vec![(1, false, 0)]);
    let b = team(0, "Bazinga", vec![(1, true, 0)]);
    assert_eq!(standings::is_better_team(&a, &b), Ordering::Less);
    Ok(())
}

#[test]
fn is_better_team_prefers_team_with_solved_puzzle() -> TmouResult<()> {
    let a = team(0, "Absolutno", vec![(1, true, 0), (2, true, 0)]);
    let b = team(0, "Bazinga", vec![(1, false, 1), (2, true, 1)]);
    assert_eq!(standings::is_better_team(&a, &b), Ordering::Greater);
    Ok(())
}

#[test]
fn is_better_team_prefers_faster_team() -> TmouResult<()> {
    let a = team(0, "Absolutno", vec![(1, false, 0), (2, false, 1)]);
    let b = team(0, "Bazinga", vec![(1, false, 1), (2, false, 0)]);
    assert_eq!(standings::is_better_team(&a, &b), Ordering::Greater);
    Ok(())
}

#[test]
fn is_better_team_prefers_faster_team_on_last_solved() -> TmouResult<()> {
    let a = team(
        0,
        "Absolutno",
        vec![
            (1, true, 0),
            (2, true, 1),
            (3, false, 1),
            (4, true, 1),
            (5, false, 1),
            (6, true, 1),
            (7, true, 1),
            (8, false, 1),
        ],
    );
    let b = team(
        0,
        "Bazinga",
        vec![
            (1, true, 0),
            (2, true, 1),
            (3, true, 1),
            (4, false, 1),
            (5, true, 1),
            (6, false, 1),
            (7, true, 2),
            (8, false, 2),
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
            (1, true, 1),
            (2, true, 1),
            (3, true, 1),
            (4, false, 1),
            (5, true, 1),
        ],
    );
    let b = team(
        0,
        "Bazinga",
        vec![(1, false, 0), (2, false, 5), (3, true, 7)],
    );
    assert_eq!(standings::is_better_team(&a, &b), Ordering::Greater);
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
                name: Some("item".to_string()),
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
            ("puzzles-fake", 1, 0),
            ("puzzles-fake", 1, 0),
            ("puzzles", 1, 5),
            ("puzzles", 1, 6),
            ("puzzles", 1, 8),
            ("puzzles", 1, 7),
            ("dead", 1, 10),
            ("puzzles", 2, 20),
            ("dead", 2, 25),
            ("puzzles", 3, 30),
            ("puzzles", 4, 50),
        ],
    );
    let mut b = db_team(
        "Bazinga",
        vec![
            ("puzzles-fake", 1, 0),
            ("puzzles-fake", 1, 0),
            ("puzzles", 1, 5),
            ("puzzles", 1, 6),
            ("puzzles", 2, 20),
            ("dead", 2, 25),
            ("puzzles", 3, 30),
            ("puzzles", 4, 50),
        ],
    );
    a.append(&mut b);

    let mut res_1 = team(
        1,
        "Bazinga",
        vec![(1, false, 5), (2, true, 20), (3, false, 30), (4, false, 50)],
    );
    res_1.start_puzzles_solved = 2;

    let mut res_2 = team(
        2,
        "Absolutno",
        vec![(1, true, 5), (2, true, 20), (3, false, 30), (4, false, 50)],
    );
    res_2.start_puzzles_solved = 4;

    let expected = vec![res_1, res_2];

    let st = standings::calculate_teams_standings(a)?;
    assert_eq!(st.standings, expected);
    Ok(())
}

#[test]
fn calculate_teams_standings_outputs_correct_badge_counts() -> TmouResult<()> {
    let mut a = db_team(
        "Absolutno",
        vec![
            ("puzzles-fake", 1, 0),
            ("puzzles-fake", 1, 0),
            ("puzzles", 1, 5),
            ("puzzles", 1, 6),
            ("badge", 0, 21),
            ("puzzles", 1, 7),
            ("puzzles", 1, 8),
            ("dead", 1, 10),
            ("puzzles", 2, 20),
            ("badge", 0, 29),
            ("dead", 2, 25),
            ("puzzles", 3, 30),
            ("badge", 0, 11),
            ("puzzles", 4, 50),
            ("badge", 0, 38),
            ("badge", 0, 20),
        ],
    );
    let mut b = db_team(
        "Bazinga",
        vec![
            ("puzzles-fake", 1, 0),
            ("badge", 0, 21),
            ("puzzles-fake", 1, 0),
            ("puzzles", 1, 5),
            ("puzzles", 1, 6),
            ("puzzles", 2, 20),
            ("dead", 2, 25),
            ("puzzles", 3, 30),
            ("puzzles", 4, 50),
        ],
    );
    let mut c = db_team(
        "Corn Flakes",
        vec![
            ("puzzles-fake", 1, 0),
            ("puzzles-fake", 1, 0),
            ("puzzles", 1, 5),
            ("puzzles", 1, 6),
            ("puzzles", 2, 59),
        ],
    );
    let mut d = db_team("Degen a spol", Vec::new());
    a.append(&mut b);
    a.append(&mut c);
    a.append(&mut d);

    let st = standings::calculate_teams_standings(a)?;
    assert_eq!(st.standings.len(), 4);
    assert_eq!(st.standings[0].name, "Bazinga".to_string());
    assert_eq!(st.standings[0].badge_count, 1);
    assert_eq!(st.standings[1].name, "Absolutno".to_string());
    assert_eq!(st.standings[1].badge_count, 5);
    assert_eq!(st.standings[2].name, "Corn Flakes".to_string());
    assert_eq!(st.standings[2].badge_count, 0);
    assert_eq!(st.standings[3].name, "Degen a spol".to_string());
    assert_eq!(st.standings[3].badge_count, 0);
    Ok(())
}
