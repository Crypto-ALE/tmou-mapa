use super::errors::*;
use super::db_models as db;
use chrono::prelude::*;
use chrono::{Utc,Duration};

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

// every 30 minutes starting Friday 21:00, a new fake is available
fn is_eligible_for_fake(time: DateTime<Utc>, inventory: &Items) -> bool
{
    let fake_count= inventory.iter().filter(|i| i.type_ =="puzzles-fake".to_string()).count();
    let eligible_time = Utc.ymd(2020, 11, 06).and_hms(21, 0, 0) + Duration::minutes(30 * fake_count as i64);
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
      (it.level == level+1) && !inventory.contains(it)
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
