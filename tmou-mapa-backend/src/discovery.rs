use super::errors::*;
use super::db_models as db;
use chrono::prelude::*;
use chrono::{Utc,Duration};
use super::skip::get_skips_limits;

#[derive(PartialEq, Debug)]
pub enum EventType {CheckpointStartVisited, PuzzlesFound, BadgeFound, Nothing}

pub type Items = Vec<db::Item>;

pub struct DiscoveryEvent
{
    pub event: EventType,
    pub newly_discovered_items: Items,
    pub updated_inventory: Items
}



// inventory contains real puzzle for given fake puzzle name
// based on name: the puzzles must correspond, e. g.
// real: "puzzles-1a"; fake: "puzzles-1a-fake"
fn contains_real(inventory: &Items, fake_name: &String) -> bool
{
    inventory.iter().any(|i| (i.name.clone() + "-fake").eq(fake_name))
}

fn get_game_start() -> DateTime<Utc>
{
    // TODO: do timezoning properly
    // TODO: read game start time from somewhere (env variables do not work locally)
    Utc.ymd(2020, 11, 06).and_hms(20, 0, 0) - Duration::hours(1)
}

// every 30 minutes starting Friday 21:00, a new fake is available
fn is_eligible_for_fake(time: DateTime<Utc>, inventory: &Items) -> bool
{
    let fake_count= inventory.iter().filter(|i| i.type_ =="puzzles-fake".to_string()).count();
    let eligible_time = get_game_start() + Duration::minutes(60 + 30 * fake_count as i64);
    time >= eligible_time
}


// return all fakes that are not in inventory in real or fake form (if time allows)
fn get_fake_puzzles(time: DateTime<Utc>, level: i16, inventory: &Items, it: &db::Item, checkpoint_content: &Items) -> Items
{
    if !is_eligible_for_fake(time, &inventory) || (it.level > level)  { Vec::new() } else 
    {
        checkpoint_content.iter()
        .filter(|i| (i.level <= level+1) 
                    && (i.type_ == "puzzles-fake".to_string() 
                    && !inventory.contains(i))
                    && !contains_real(inventory, &i.name))
        .cloned().collect()
  
    }
}

fn is_eligible_for_puzzle(level: i16, inventory: &Items, it: &db::Item) -> bool
{
    // can take the same or next level
    (it.level == level || it.level == level + 1) && !inventory.contains(it)
}


fn is_eligible_for_badge(level: i16, inventory: &Items, it: &db::Item) -> bool
{
    (it.level <= level) && !inventory.contains(it)
}

// time parameter added for unit testing
pub fn discover_node(time: DateTime<Utc>, inventory: &Items, node_contents: &Items) -> TmouResult<DiscoveryEvent>
{
    // player-level is the maximum level of any item, or -1 at start (eligible for puzzles level 0)
    let player_level = inventory.iter().map(|item| item.level).max().unwrap_or(-1);

    // intermediate collections, accumulated during discovery of all items in node
    let mut event = EventType::Nothing; // last event wins - should be only one
    let mut current_inventory= inventory[..].to_vec();
    let mut newly_discovered_items = Vec::new();


    for item in node_contents.iter()
    {
        match item.type_.as_ref()
        {
            "puzzles" => 
            {
                event = EventType::PuzzlesFound;
                if is_eligible_for_puzzle(player_level, &current_inventory, item)
                {
                    current_inventory.push(item.clone());
                    newly_discovered_items.push(item.clone());
                }
            }
            "badge" => 
            {
                event = EventType::BadgeFound;
                if is_eligible_for_badge(player_level, &current_inventory, item)
                {
                    current_inventory.push(item.clone());
                    newly_discovered_items.push(item.clone());
                }
            }
            "checkpoint-start" => 
            {
                event = EventType::CheckpointStartVisited;
                // pass all fake puzzles to the function
                let new_items = get_fake_puzzles(time, player_level, &current_inventory, item, &node_contents);
                // not included in inventory
                newly_discovered_items.extend(new_items);
            }
            _ => () // fake puzzles found - handled by checkpoint-start
        }
    }


    Ok(DiscoveryEvent{
        event, 
        newly_discovered_items: newly_discovered_items,
        updated_inventory: current_inventory})
}

// time parameter added for unit testing
// returns updated inventory
pub fn discover_fake_puzzle(
    time: DateTime<Utc>, 
    inventory: &Items, 
    node_contents: &Items,
    puzzle_name: &String) -> TmouResult<Items>
{
    // player-level is the maximum level of any item, or -1 at start (eligible for puzzles level 0)
    let player_level = inventory.iter().map(|item| item.level).max().unwrap_or(-1);
    let checkpoint = node_contents.iter()
      .find(|i| i.type_ == String::from("checkpoint-start"))
      .ok_or(TmouError{message:String::from("not on checkpoint"), response:404})?;
    let puzzles = get_fake_puzzles(time, player_level, inventory, &checkpoint, &node_contents);
    match puzzles.iter().find(|i| i.name.eq(puzzle_name))
    {
        Some(p) =>
        {
            let mut updated_inventory = inventory.clone();
            updated_inventory.push(p.clone());
            Ok(updated_inventory)
        }
        None => Err(TmouError{message:String::from("not eligible for this fake puzzle"), response:404})
    }
}




pub fn format_skip_limit(badges:usize, max_badges: usize, limit: i64) -> String
{
    let badges = match badges
    {
        x if x == max_badges => format!("{} a více bonusů", x),
        0 => String::from("0 bonusů"),
        1 => String::from("1 bonus"),
        x if x>=2 && x <=4 => format!("{} bonusy", x),
        x => format!("{} bonusů", x),
    };

    format!(" {}: {} týmů;", badges, limit)
}

pub fn get_puzzle_welcome_message(
    game_state: Vec<i64>, 
    inventory: Items) -> TmouResult<String>
{
    let max_puzzle_level = inventory.iter().map(|item| item.level as usize).max().unwrap_or(0);
    let ranking = match max_puzzle_level
    {
        x if x >= game_state.len() => 1,
        x => game_state[x]
    };
    let skips_limits = get_skips_limits(max_puzzle_level);
    let bonus_line = match skips_limits
    {
        Some(limits) => limits.iter().enumerate()
          .fold(String::from("K přeskočení šifry potřebujete, aby šifrou prošlo pro:"), 
          |acc, (i, l)| acc + &format_skip_limit(i,limits.len() - 1, *l)),
  
        None => String::from("Tuto šifru nelze přeskočit.")
    };
    
    Ok(format!("Jste tu {}. {}", ranking, bonus_line))
}
