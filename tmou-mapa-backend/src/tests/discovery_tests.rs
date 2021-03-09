#[allow(unused_imports)]
use chrono::prelude::*;
#[allow(unused_imports)]
use chrono::{Duration, Utc};

#[allow(unused_imports)]
use crate::controllers::discovery as dis;
#[allow(unused_imports)]
use crate::models::db;
#[allow(unused_imports)]
use crate::models::errors::*;

#[allow(unused)]
fn item(t: &str, l: i16, n: &str) -> db::Item {
    db::Item {
        type_: t.to_string(),
        url: "Dummy".to_string(),
        level: l,
        name: n.to_string(),
        description: None,
    }
}

#[test]
fn discovery_returns_unchanged_inventory_when_nothing_found() -> TmouResult<()> {
    // ready for picking level 2:
    let inventory = vec![
        item("puzzles", 1, "puzzles-1"),
        item("badge", 1, "badge-1-1"),
        item("badge", 1, "badge-1-2"),
        item("badge", 1, "badge-1-3"),
    ];

    // empty node
    let node_contents = Vec::new();

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::Nothing);
    assert_eq!(evt.updated_inventory, inventory);
    assert_eq!(evt.newly_discovered_items, Vec::new());
    Ok(())
}

#[test]
fn discovery_returns_level_0_puzzles_at_start() -> TmouResult<()> {
    let inventory = Vec::new();

    // checkpoint with puzzles
    let node_contents = vec![item("puzzles", 0, "puzzles-0")];

    let expected_inventory = vec![item("puzzles", 0, "puzzles-0")];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::PuzzlesFound);
    assert_eq!(evt.updated_inventory, expected_inventory);
    assert_eq!(
        evt.newly_discovered_items,
        vec![item("puzzles", 0, "puzzles-0")]
    );
    Ok(())
}

#[test]
fn discovery_returns_level_1_puzzles_when_in_level_0() -> TmouResult<()> {
    let inventory = vec![item("puzzles", 0, "puzzles-0")];

    // checkpoint with puzzles
    let node_contents = vec![item("puzzles", 1, "puzzles-1")];

    let expected_inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles", 1, "puzzles-1"),
    ];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::PuzzlesFound);
    assert_eq!(evt.updated_inventory, expected_inventory);
    assert_eq!(
        evt.newly_discovered_items,
        vec![item("puzzles", 1, "puzzles-1")]
    );
    Ok(())
}

#[test]
fn discovery_returns_level_4_puzzles_when_in_level_4() -> TmouResult<()> {
    let inventory = vec![item("puzzles", 4, "puzzles-4a")];

    // checkpoint with puzzles
    let node_contents = vec![item("puzzles", 4, "puzzles-4b")];

    let expected_inventory = vec![
        item("puzzles", 4, "puzzles-4a"),
        item("puzzles", 4, "puzzles-4b"),
    ];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::PuzzlesFound);
    assert_eq!(evt.updated_inventory, expected_inventory);
    assert_eq!(
        evt.newly_discovered_items,
        vec![item("puzzles", 4, "puzzles-4b")]
    );
    Ok(())
}

#[test]
fn discovery_returns_empty_when_puzzle_level_4_found_and_in_level_5() -> TmouResult<()> {
    let inventory = vec![item("puzzles", 5, "puzzles-5")];

    // checkpoint with puzzles
    let node_contents = vec![item("puzzles", 4, "puzzles-4b")];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::PuzzlesFound);
    assert_eq!(evt.updated_inventory, inventory);
    assert_eq!(evt.newly_discovered_items, Vec::new());
    Ok(())
}

#[test]
fn discovery_returns_nothing_on_level_1_puzzles_at_start() -> TmouResult<()> {
    let inventory = Vec::new();

    // checkpoint with puzzles
    let node_contents = vec![item("puzzles", 1, "puzzles-1")];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::Nothing);
    assert_eq!(evt.updated_inventory, inventory);
    assert_eq!(evt.newly_discovered_items, Vec::new());
    Ok(())
}

#[test]
fn discovery_returns_badge_level_when_found_at_start() -> TmouResult<()> {
    // ready for a new badge
    let inventory = Vec::new();

    // new badge
    let node_contents = vec![item("badge", -1, "badge-1-3")];

    let expected_inventory = vec![item("badge", -1, "badge-1-3")];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::BadgeFound);
    assert_eq!(evt.updated_inventory, expected_inventory);
    assert_eq!(
        evt.newly_discovered_items,
        vec![item("badge", -1, "badge-1-3")]
    );
    Ok(())
}

#[test]
fn discovery_returns_badge_level_when_found_when_inventory_somehow_populated() -> TmouResult<()> {
    // ready for a new badge
    let inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles", 1, "puzzles-1"),
        item("puzzles", 2, "puzzles-2"),
        item("puzzles", 3, "puzzles-3"),
        item("puzzles", 4, "puzzles-4"),
        item("puzzles", 5, "puzzles-5"),
        item("badge", -1, "badge-1-1"),
        item("badge", -1, "badge-1-2"),
        item("badge", -1, "badge-1-4"),
        item("badge", -1, "badge-1-5"),
        item("badge", -1, "badge-1-6"),
        item("badge", -1, "badge-1-7"),
        item("badge", -1, "badge-1-8"),
        item("badge", -1, "badge-1-9"),
    ];

    // new badge
    let node_contents = vec![item("badge", -1, "badge-1-3")];

    let expected_inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles", 1, "puzzles-1"),
        item("puzzles", 2, "puzzles-2"),
        item("puzzles", 3, "puzzles-3"),
        item("puzzles", 4, "puzzles-4"),
        item("puzzles", 5, "puzzles-5"),
        item("badge", -1, "badge-1-1"),
        item("badge", -1, "badge-1-2"),
        item("badge", -1, "badge-1-4"),
        item("badge", -1, "badge-1-5"),
        item("badge", -1, "badge-1-6"),
        item("badge", -1, "badge-1-7"),
        item("badge", -1, "badge-1-8"),
        item("badge", -1, "badge-1-9"),
        item("badge", -1, "badge-1-3"),
    ];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::BadgeFound);
    assert_eq!(evt.updated_inventory, expected_inventory);
    assert_eq!(
        evt.newly_discovered_items,
        vec![item("badge", -1, "badge-1-3")]
    );
    Ok(())
}

#[test]
fn discovery_returns_nothing_when_inventory_already_contains_badge() -> TmouResult<()> {
    // ready for a new badge
    let inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles", 1, "puzzles-1"),
        item("puzzles", 2, "puzzles-2"),
        item("puzzles", 3, "puzzles-3"),
        item("puzzles", 4, "puzzles-4"),
        item("puzzles", 5, "puzzles-5"),
        item("badge", -1, "badge-1-1"),
        item("badge", -1, "badge-1-2"),
        item("badge", -1, "badge-1-4"),
        item("badge", -1, "badge-1-5"),
        item("badge", -1, "badge-1-6"),
        item("badge", -1, "badge-1-7"),
        item("badge", -1, "badge-1-8"),
        item("badge", -1, "badge-1-9"),
    ];

    // new badge
    let node_contents = vec![item("badge", -1, "badge-1-4")];

    let expected_inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles", 1, "puzzles-1"),
        item("puzzles", 2, "puzzles-2"),
        item("puzzles", 3, "puzzles-3"),
        item("puzzles", 4, "puzzles-4"),
        item("puzzles", 5, "puzzles-5"),
        item("badge", -1, "badge-1-1"),
        item("badge", -1, "badge-1-2"),
        item("badge", -1, "badge-1-4"),
        item("badge", -1, "badge-1-5"),
        item("badge", -1, "badge-1-6"),
        item("badge", -1, "badge-1-7"),
        item("badge", -1, "badge-1-8"),
        item("badge", -1, "badge-1-9"),
    ];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::BadgeFound);
    assert_eq!(evt.updated_inventory, expected_inventory);
    assert_eq!(evt.newly_discovered_items, Vec::new());
    Ok(())
}

#[test]
fn discovery_returns_final_badge_when_on_proper_level() -> TmouResult<()> {
    // ready for a new badge
    let inventory = vec![item("puzzles", 14, "puzzles-14")];

    // new badge
    let node_contents = vec![item("badge", 14, "final-badge")];

    let expected_inventory = vec![
        item("puzzles", 14, "puzzles-14"),
        item("badge", 14, "final-badge"),
    ];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::BadgeFound);
    assert_eq!(evt.updated_inventory, expected_inventory);
    assert_eq!(
        evt.newly_discovered_items,
        vec![item("badge", 14, "final-badge")]
    );
    Ok(())
}

#[test]
fn discovery_returns_nothing_when_on_badge_but_insufficient_level() -> TmouResult<()> {
    // ready for a new badge
    let inventory = vec![item("puzzles", 13, "puzzles-13")];

    // new badge
    let node_contents = vec![item("badge", 14, "final-badge")];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::Nothing);
    assert_eq!(evt.updated_inventory, inventory);
    assert_eq!(evt.newly_discovered_items, Vec::new());
    Ok(())
}

#[test]
fn discovery_returns_fakes_on_checkpoint_when_eligible_nothing_owned() -> TmouResult<()> {
    let inventory = vec![item("puzzles", 0, "puzzles-0")];

    // checkpoint with puzzles
    let node_contents = vec![
        item("checkpoint-start", 0, "checkpoint-start"),
        item("puzzles-fake", 1, "puzzles-1a-fake"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
    ];

    let expected_new_items = vec![
        item("puzzles-fake", 1, "puzzles-1a-fake"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
    ];

    let time = Utc.ymd(2020, 11, 06).and_hms(21, 0, 0) - Duration::hours(1);
    let evt = dis::discover_node(time, &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::CheckpointStartVisited);
    assert_eq!(evt.updated_inventory, inventory);
    assert_eq!(evt.newly_discovered_items, expected_new_items);
    Ok(())
}

#[test]
fn discovery_returns_nothing_on_checkpoint_when_not_eligible_nothing_owned() -> TmouResult<()> {
    let inventory = vec![item("puzzles", 0, "puzzles-0")];

    // checkpoint with puzzles
    let node_contents = vec![
        item("checkpoint-start", 0, "checkpoint-start"),
        item("puzzles-fake", 1, "puzzles-1a-fake"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
    ];

    let time = Utc.ymd(2020, 11, 06).and_hms(20, 59, 59) - Duration::hours(1);
    let evt = dis::discover_node(time, &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::CheckpointStartVisited);
    assert_eq!(evt.updated_inventory, inventory);
    assert_eq!(evt.newly_discovered_items, Vec::new());
    Ok(())
}

#[test]
fn discovery_returns_subset_on_checkpoint_when_eligible_some_owned() -> TmouResult<()> {
    let inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles", 1, "puzzles-1a"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
        item("puzzles-fake", 1, "puzzles-1e-fake"),
        item("puzzles-fake", 1, "puzzles-1f-fake"),
    ];

    // checkpoint with puzzles
    let node_contents = vec![
        item("checkpoint-start", 0, "checkpoint-start"),
        item("puzzles-fake", 1, "puzzles-1a-fake"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
        item("puzzles-fake", 1, "puzzles-1c-fake"),
        item("puzzles-fake", 1, "puzzles-1d-fake"),
    ];

    let expected_new_items = vec![
        item("puzzles-fake", 1, "puzzles-1c-fake"),
        item("puzzles-fake", 1, "puzzles-1d-fake"),
    ];

    let time = Utc.ymd(2020, 11, 06).and_hms(22, 30, 0) - Duration::hours(1);
    let evt = dis::discover_node(time, &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::CheckpointStartVisited);
    assert_eq!(evt.updated_inventory, inventory);
    assert_eq!(evt.newly_discovered_items, expected_new_items);
    Ok(())
}

#[test]
fn discovery_returns_nothing_on_checkpoint_when_not_eligible_some_owned() -> TmouResult<()> {
    let inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles-fake", 1, "puzzles-1a"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
        item("puzzles-fake", 1, "puzzles-1e-fake"),
        item("puzzles-fake", 1, "puzzles-1f-fake"),
    ];

    // checkpoint with puzzles
    let node_contents = vec![
        item("checkpoint-start", 0, "checkpoint-start"),
        item("puzzles-fake", 1, "puzzles-1a-fake"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
        item("puzzles-fake", 1, "puzzles-1c-fake"),
        item("puzzles-fake", 1, "puzzles-1d-fake"),
    ];

    let time = Utc.ymd(2020, 11, 06).and_hms(22, 29, 59) - Duration::hours(1);
    let evt = dis::discover_node(time, &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::CheckpointStartVisited);
    assert_eq!(evt.updated_inventory, inventory);
    assert_eq!(evt.newly_discovered_items, Vec::new());
    Ok(())
}

#[test]
fn format_skip_limit_honors_declinations() -> TmouResult<()> {
    assert_eq!(
        dis::format_skip_limit(0, 7, 200),
        String::from(" 0 bonusů: 200 týmů;")
    );
    assert_eq!(
        dis::format_skip_limit(1, 7, 200),
        String::from(" 1 bonus: 200 týmů;")
    );
    assert_eq!(
        dis::format_skip_limit(2, 7, 200),
        String::from(" 2 bonusy: 200 týmů;")
    );
    assert_eq!(
        dis::format_skip_limit(3, 7, 200),
        String::from(" 3 bonusy: 200 týmů;")
    );
    assert_eq!(
        dis::format_skip_limit(4, 7, 200),
        String::from(" 4 bonusy: 200 týmů;")
    );
    assert_eq!(
        dis::format_skip_limit(5, 7, 200),
        String::from(" 5 bonusů: 200 týmů;")
    );
    assert_eq!(
        dis::format_skip_limit(6, 7, 200),
        String::from(" 6 bonusů: 200 týmů;")
    );
    assert_eq!(
        dis::format_skip_limit(7, 7, 200),
        String::from(" 7 a více bonusů: 200 týmů;")
    );
    Ok(())
}

#[test]
fn get_puzzle_welcome_message_returns_nonskippable_before_start() -> TmouResult<()> {
    let game_state = vec![100, 90, 80, 70];
    let inventory = Vec::new();
    let msg = dis::get_puzzle_welcome_message(game_state, inventory).unwrap();
    assert_eq!(
        msg,
        String::from("Vítejte před hrou! Jste tu 100. Tuto šifru nelze přeskočit.")
    );
    Ok(())
}

#[test]
fn get_puzzle_welcome_message_returns_nonskippable_on_start() -> TmouResult<()> {
    let game_state = vec![100, 90, 80, 70];
    let inventory = vec![item("puzzles", 0, "šifra 0a")];
    let msg = dis::get_puzzle_welcome_message(game_state, inventory).unwrap();
    assert_eq!(
        msg,
        String::from("Vítejte na startu! Jste tu 100. Tuto šifru nelze přeskočit.")
    );
    Ok(())
}

#[test]
fn get_puzzle_welcome_message_returns_skip_sequence_on_1() -> TmouResult<()> {
    let game_state = vec![100, 90, 80, 70];
    let mut item_with_desc = item("puzzles", 1, "puzzle-1");
    item_with_desc.description = Some("šifra 1".to_string());
    let inventory = vec![item("puzzles", 0, "puzzles-0"), item_with_desc];
    let msg = dis::get_puzzle_welcome_message(game_state, inventory).unwrap();
    assert_eq!(msg, String::from("Vítejte na další šifře! Přibyla vám šifra 1. Jste tu 90. K přeskočení šifry potřebujete, aby šifrou prošlo pro: \
                                  0 bonusů: 250 týmů; 1 a více bonusů: 200 týmů;"));
    Ok(())
}

#[test]
fn discover_fake_puzzle_succeeds_when_eligible_some_owned() -> TmouResult<()> {
    let inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles", 1, "puzzles-1a"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
        item("puzzles-fake", 1, "puzzles-1e-fake"),
        item("puzzles-fake", 1, "puzzles-1f-fake"),
    ];

    // checkpoint with puzzles
    let node_contents = vec![
        item("checkpoint-start", 0, "checkpoint-start"),
        item("puzzles-fake", 1, "puzzles-1a-fake"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
        item("puzzles-fake", 1, "puzzles-1c-fake"),
        item("puzzles-fake", 1, "puzzles-1d-fake"),
    ];

    let expected_inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles", 1, "puzzles-1a"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
        item("puzzles-fake", 1, "puzzles-1e-fake"),
        item("puzzles-fake", 1, "puzzles-1f-fake"),
        item("puzzles-fake", 1, "puzzles-1d-fake"),
    ];

    let time = Utc.ymd(2020, 11, 06).and_hms(22, 20, 0) - Duration::hours(1);
    let updated_inventory = dis::discover_fake_puzzle(
        time,
        &inventory,
        &node_contents,
        &String::from("puzzles-1d-fake"),
    )?;
    assert_eq!(updated_inventory, expected_inventory);
    Ok(())
}

#[test]
fn discover_fake_puzzle_fails_on_checkpoint_when_not_eligible_some_owned() -> TmouResult<()> {
    let inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles", 1, "puzzles-1a"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
        item("puzzles-fake", 1, "puzzles-1e-fake"),
        item("puzzles-fake", 1, "puzzles-1f-fake"),
    ];

    // checkpoint with puzzles
    let node_contents = vec![
        item("checkpoint-start", 0, "checkpoint-start"),
        item("puzzles-fake", 1, "puzzles-1a-fake"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
        item("puzzles-fake", 1, "puzzles-1c-fake"),
        item("puzzles-fake", 1, "puzzles-1d-fake"),
    ];

    let time = Utc.ymd(2020, 11, 06).and_hms(22, 19, 0) - Duration::hours(1);
    let updated_inventory = dis::discover_fake_puzzle(
        time,
        &inventory,
        &node_contents,
        &String::from("puzzles-1d-fake"),
    );
    assert!(!updated_inventory.is_ok());
    Ok(())
}
