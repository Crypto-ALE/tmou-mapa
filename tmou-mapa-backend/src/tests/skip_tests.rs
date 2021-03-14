#[allow(unused_imports)]
use crate::controllers::skip::*;
#[allow(unused_imports)]
use crate::models::db::Item;
#[allow(unused_imports)]
use crate::models::errors::*;

#[test]
fn level0_cannot_be_skipped() {
    let inventory = vec![Item {
        level: 0,
        description: None,
        condition: None,
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
        condition: None,
        type_: "puzzles".to_string(),
        name: "puzzle".to_string(),
        url: "".to_string(),
    }];
    assert_eq!(is_allowed(inventory, vec!(500, 500, 100)).unwrap(), false);
}
#[test]
fn level1_1_bonus_few_teams() {
    let inventory = vec![
        Item {
            level: 1,
            description: None,
            condition: None,
            type_: "puzzles".to_string(),
            name: "puzzle".to_string(),
            url: "".to_string(),
        },
        Item {
            level: -1,
            description: None,
            condition: None,
            type_: "badge".to_string(),
            name: "badge".to_string(),
            url: "".to_string(),
        },
    ];
    assert_eq!(is_allowed(inventory, vec!(500, 500, 100)).unwrap(), false);
}
#[test]
fn level1_no_bonuses_enough_teams() {
    let inventory = vec![Item {
        level: 1,
        description: None,
        condition: None,
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
        condition: None,
        type_: "puzzles-fake".to_string(),
        name: "puzzle".to_string(),
        url: "".to_string(),
    }];
    assert_eq!(is_allowed(inventory, vec!(500, 500, 301)).unwrap(), true);
}
#[test]
fn level1_no_bonuses_enough_teams_already_dead() {
    let inventory = vec![
        Item {
            level: 1,
            description: None,
            condition: None,
            type_: "puzzles".to_string(),
            name: "puzzle".to_string(),
            url: "".to_string(),
        },
        Item {
            level: 1,
            description: None,
            condition: None,
            type_: "dead".to_string(),
            name: "dead".to_string(),
            url: "".to_string(),
        },
    ];
    assert_eq!(is_allowed(inventory, vec!(500, 500, 301)).unwrap(), false);
}
#[test]
fn level1_1_bonus_enough_teams() {
    let inventory = vec![
        Item {
            level: 1,
            description: None,
            condition: None,
            type_: "puzzles".to_string(),
            name: "puzzle".to_string(),
            url: "".to_string(),
        },
        Item {
            level: -1,
            description: None,
            condition: None,
            type_: "badge".to_string(),
            name: "badge".to_string(),
            url: "".to_string(),
        },
    ];
    assert_eq!(is_allowed(inventory, vec!(500, 500, 201)).unwrap(), true);
}
#[test]
fn level1_more_bonuses_than_defined_enough_teams() {
    let inventory = vec![
        Item {
            level: 1,
            description: None,
            condition: None,
            type_: "puzzles".to_string(),
            name: "puzzle".to_string(),
            url: "".to_string(),
        },
        Item {
            level: -1,
            description: None,
            condition: None,
            type_: "badge".to_string(),
            name: "badge".to_string(),
            url: "".to_string(),
        },
        Item {
            level: -1,
            description: None,
            condition: None,
            type_: "badge".to_string(),
            name: "badge".to_string(),
            url: "".to_string(),
        },
        Item {
            level: -1,
            description: None,
            condition: None,
            type_: "badge".to_string(),
            name: "badge".to_string(),
            url: "".to_string(),
        },
    ];
    assert_eq!(is_allowed(inventory, vec!(500, 500, 201)).unwrap(), true);
}
#[test]
fn level1_no_team_in_next_level() {
    let inventory = vec![Item {
        level: 0,
        description: None,
        condition: None,
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
        condition: None,
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
        condition: None,
        type_: "puzzles".to_string(),
        name: "puzzle".to_string(),
        url: "".to_string(),
    }];
    assert_eq!(
        is_allowed(inventory, vec!(10, 20)).unwrap_err().message,
        "Invalid game state provided.".to_string()
    );
}
