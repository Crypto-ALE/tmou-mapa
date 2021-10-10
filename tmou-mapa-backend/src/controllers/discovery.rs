use evalexpr::*;
use std::env;

use chrono::prelude::*;
use chrono::{Duration, Utc};

use crate::models::db;
use crate::models::errors::*;

use super::get_team_level;

#[derive(PartialEq, Debug)]
pub enum EventType {
    CheckpointStartVisited,
    PuzzlesFound,
    BadgeFound,
    Nothing,
}

pub type Items = Vec<db::Item>;

pub struct DiscoveryEvent {
    pub event: EventType,
    pub newly_discovered_items: Items,
    pub updated_inventory: Items,
}

///////////////////////////////////////////////////////////
/// Interface
///////////////////////////////////////////////////////////

// time parameter added for unit testing
pub fn discover_node(
    time: DateTime<Utc>,
    inventory: &Items,
    node_contents: &Items,
) -> TmouResult<DiscoveryEvent> {
    // player-level is the maximum level of any item, or -1 at start (eligible for puzzles level 0)
    let team_level = get_team_level(&inventory);

    // intermediate collections, accumulated during controllers::discovery of all items in node
    let mut current_inventory = inventory[..].to_vec();
    let mut newly_discovered_items = Vec::new();
    // in some situations, there can be multiple events in one node
    // last visible event wins, default event is Nothing 
    let mut event = EventType::Nothing;

    for item in node_contents.iter() {
        match item.type_.as_ref() {
            "puzzles" => {
                // all puzzles up to level+1 are visible
                let visible = is_item_visible(item, inventory, team_level)?;
                //but only those at least your level are active
                let active = item.level >= team_level;
                if visible {
                    event = EventType::PuzzlesFound;
                };
                if visible && active && !current_inventory.contains(item) {
                    current_inventory.push(item.clone());
                    newly_discovered_items.push(item.clone());
                }
            }
            "badge" => {
                let visible = is_item_visible(item, inventory, team_level)?;
                if visible {
                    event = EventType::BadgeFound;
                };
                if visible && !current_inventory.contains(item) {
                    current_inventory.push(item.clone());
                    newly_discovered_items.push(item.clone());
                }
            }
            "checkpoint-start" => {
                event = EventType::CheckpointStartVisited;
                // pass all fake puzzles to the function
                let new_items =
                    get_fake_puzzles(time, team_level, &current_inventory, item, &node_contents)?;
                // not included in inventory
                newly_discovered_items.extend(new_items);
            }
            _ => (), // fake puzzles found - handled by checkpoint-start
        }
    }

    Ok(DiscoveryEvent {
        event,
        newly_discovered_items: newly_discovered_items,
        updated_inventory: current_inventory,
    })
}

// time parameter added for unit testing
// returns updated inventory
pub fn discover_fake_puzzle(
    time: DateTime<Utc>,
    inventory: &Items,
    node_contents: &Items,
    puzzle_name: &String,
) -> TmouResult<Items> {
    // player-level is the maximum level of any item, or -1 at start (eligible for puzzles level 0)
    let team_level = inventory.iter().map(|item| item.level).max().unwrap_or(-1);
    let checkpoint = node_contents
        .iter()
        .find(|i| i.type_ == String::from("checkpoint-start"))
        .ok_or(TmouError {
            message: String::from("not on checkpoint"),
            response: 404,
        })?;
    let puzzles = get_fake_puzzles(time, team_level, inventory, &checkpoint, &node_contents)?;
    match puzzles.iter().find(|i| i.name.eq(puzzle_name)) {
        Some(p) => {
            let mut updated_inventory = inventory.clone();
            updated_inventory.push(p.clone());
            Ok(updated_inventory)
        }
        None => Err(TmouError {
            message: String::from("not eligible for this fake puzzle"),
            response: 404,
        }),
    }
}

///////////////////////////////////////////////////////////
/// Implementation helpers
///////////////////////////////////////////////////////////

// inventory contains real puzzle for given fake puzzle name
// based on name: the puzzles must correspond, e. g.
// real: "puzzles-1a"; fake: "puzzles-1a-fake"
fn contains_real(inventory: &Items, fake_name: &String) -> bool {
    inventory
        .iter()
        .any(|i| (i.name.clone() + "-fake").eq(fake_name))
}

fn get_game_start() -> TmouResult<DateTime<FixedOffset>> {
    // TODO: do timezoning properly
    let time_str = env::var("TMOU_GAME_START")?;
    DateTime::parse_from_rfc3339(time_str.as_str()).or_else(|e| Err(e.into()))
}

// starting Friday 21:00, a new fake is available:
// every 30 minutes for first hour (2 fakes)
// every 20 minutes for next hour (3 fakes)
// every 15 minutes for next hour (4 fakes)
fn is_eligible_for_fake(time: DateTime<Utc>, inventory: &Items) -> TmouResult<bool> {
    let fake_count = inventory
        .iter()
        .filter(|i| i.type_ == "puzzles-fake".to_string())
        .count();
    let minutes = match fake_count {
        0 => 60,
        1 => 60 + 30,
        2 => 60 + 60,
        3 => 120 + 20,
        4 => 120 + 40,
        5 => 120 + 60,
        6 => 180 + 15,
        7 => 180 + 30,
        8 => 180 + 45,
        9 => 180 + 60,
        _ => 9999, /* doesn't happen */
    };
    let eligible_time = get_game_start()? + Duration::minutes(minutes);
    Ok(time >= eligible_time)
}

// return all fakes that are not in inventory in real or fake form (if time allows)
fn get_fake_puzzles(
    time: DateTime<Utc>,
    level: i16,
    inventory: &Items,
    it: &db::Item,
    checkpoint_content: &Items,
) -> TmouResult<Items> {
    let res = if !is_eligible_for_fake(time, &inventory)? || (it.level > level) {
        Vec::new()
    } else {
        checkpoint_content
            .iter()
            .filter(|i| {
                (i.level <= level + 1)
                    && (i.type_ == "puzzles-fake".to_string() && !inventory.contains(i))
                    && !contains_real(inventory, &i.name)
            })
            .cloned()
            .collect()
    };
    Ok(res)
}

pub fn format_skip_limit(badges: usize, max_badges: usize, limit: i64) -> String {
    let badges = match badges {
        x if x == max_badges => format!("{} a více bonusů", x),
        0 => String::from("0 bonusů"),
        1 => String::from("1 bonus"),
        x if x >= 2 && x <= 4 => format!("{} bonusy", x),
        x => format!("{} bonusů", x),
    };

    format!(" {}: {} týmů;", badges, limit)
}

pub fn get_puzzle_welcome_message(_game_state: Vec<i64>, inventory: Items) -> TmouResult<String> {
    // In current flow, inventory is guaranteed to contain puzzles only; this can change in the future
    let max_puzzle = inventory.iter().max_by_key(|i| i.level);
    let welcome: String = match max_puzzle {
        None => String::from("Vítejte před hrou!"), // defensive; this should not happen
        Some(x) => match x.level as usize {
            0 => String::from("Vítejte v 1. levelu! Odznáčky ukažte na Náměstí Svobody u orloje."),
            1 => String::from("Vítejte ve 2. levelu! Odznáčky ukažte ve středu Velké pyramidy."),
            2 => String::from("Vítejte ve 3. levelu! Odznáčky ukažte v Řetězové bráně u Zdi nářků."),
            3 => String::from("Vítejte ve 4. levelu! Odznáček ukažte poblíž Opery v Sydney."),
            4 => String::from("Vítejte v 5. levelu! Získáním posledního odznáčku ukončíte kvalifikaci."),
            _ => String::from("Stalo se něco neočekávaného, kontaktujte organizátory."),
        },
    };

    Ok(format!("{}", welcome))
}

pub fn has<'a>(needle: String, haystack: Vec<String>) -> bool {
    haystack.contains(&needle)
}

pub fn evaluate_condition(
    condition: &str,
    inventory: &Items,
    team_level: i16,
) -> TmouResult<bool> {
    let items: Vec<String> = inventory.iter().map(|i| i.name.clone()).collect();
    let context = context_map! {
        "level" => team_level as i64,
        "has" => Function::new(Box::new(move |argument| {
            argument.as_string().and_then(|item| Ok(Value::Boolean(items.contains(&item))))
        }))
    }
    .unwrap();
    let res = eval_boolean_with_context(condition, &context)?;
    Ok(res)
}

fn is_item_visible(item: &db::Item, inventory: &Items, team_level: i16) -> TmouResult<bool> {
    match &item.condition {
        Some(cond) => evaluate_condition(&cond, inventory, team_level),
        None => Ok(true),
    }
}
