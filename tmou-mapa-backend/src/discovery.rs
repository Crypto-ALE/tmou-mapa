use super::errors::*;
use super::db_models as db;

#[derive(PartialEq, Debug)]
pub enum EventType {CheckpointVisited, BadgeFound, Nothing}

pub type Items = Vec<db::Item>;

pub struct DiscoveryEvent
{
    pub event: EventType,
    pub newly_discovered_items: Items,
    pub updated_inventory: Items
}



// order of discovery when multiple items found on place
// Badges before checkpoints so that when a badge is found it can be redeemed in checkpoint later
fn item_type_order(item: &db::Item) -> i32
{
    match item.type_.as_ref()
    {
        "badge" => 0,
        "checkpoint" => 1,
        "puzzles" => 2,
        _ => 1000
    }
}

fn max_badges_in_level(level:i16) -> usize
{
    match level
    {
        1 => 3,
        2 => 2,
        3 => 1,
        4 => 1,
        5 => 1,
        _ => 0
    }
}

fn badges_count(its: &Items, level: i16) -> usize
{
    its.iter().filter(|i| i.type_ == "badge".to_string() && i.level == level).count()
}

// returns newly discovered items
// when conditions met => new items (usually 1 set of puzzles)
// otherwise => empty Vec
fn get_new_items_from_checkpoint(level: i16, inventory: &Items, checkpoint_content: &Items) -> Items
{
    let eligible_for_new_puzzles = badges_count(&inventory, level) == max_badges_in_level(level);
    let new_puzzles = checkpoint_content.iter().find(|i| (i.level == level+1) && (i.type_ == "puzzles".to_string()));
    match eligible_for_new_puzzles && new_puzzles.is_some()
    {
        true => vec![new_puzzles.unwrap().clone()],
        false => Vec::new()
    }
}

// returns newly discovered items
// when conditions met => new items (usually 1 set of puzzles)
// otherwise => empty
fn get_badge(level: i16, inventory: &Items, it: &db::Item) -> Items
{

    match (level == it.level) && (badges_count(&inventory, level) < max_badges_in_level(level)) && !inventory.contains(it)
    {
        true => vec![it.clone()],
        false => Vec::new()
    }
}

// TODO: make rust code out of this fortran bullcrap
// (it has a backstory)
fn get_most_significant_event(events: &Vec<EventType>) -> EventType
{
    let mut res = EventType::Nothing;
    for e in events
    {
        match e
        {
            EventType::CheckpointVisited => {res = EventType::CheckpointVisited},
            EventType::BadgeFound => {if res == EventType::Nothing {res = EventType::BadgeFound}},
            _ => ()
        }
    }
    res
}


pub fn discover_node(inventory: &Items, node_contents: &Items) -> TmouResult<DiscoveryEvent>
{
    let player_level = inventory.iter().map(|item| item.level).max().or(Some(0)).unwrap();
    // sort node contents so that:
    // 1. evaluation is deterministic
    // 2. badges are discovered before checkpoints (see fn item_type_order) to allow redeeming badge in checkpoint instantly
    // so, first come badges, then checkpoints, then rest (=puzzles)
    let mut sorted_contents = node_contents.clone();
    sorted_contents.sort_by(|a, b| item_type_order(&a).partial_cmp(&item_type_order(&b)).unwrap());


    // intermediate collections, accumulated during discovery of all items in node
    let mut events = Vec::new();
    let mut current_inventory= inventory[..].to_vec();
    let mut newly_discovered_items = Vec::new();


    for item in sorted_contents.iter().filter(|i| i.level <= player_level)
    {
        match item.type_.as_ref()
        {
            "checkpoint" => 
            {
                events.push(EventType::CheckpointVisited);
                let new_items = get_new_items_from_checkpoint(player_level, &current_inventory, &sorted_contents);
                current_inventory.extend(new_items.clone());
                newly_discovered_items.extend(new_items);
            }
            "badge" => 
            {
                events.push(EventType::BadgeFound);
                let new_items = get_badge(player_level, &current_inventory, item);
                current_inventory.extend(new_items.clone());
                newly_discovered_items.extend(new_items);
            }
            _ => () // puzzles found - not directly accessible by player
        }
    }
    let most_significant_event = get_most_significant_event(&events);


    Ok(DiscoveryEvent{
        event: most_significant_event, 
        newly_discovered_items: newly_discovered_items,
        updated_inventory: current_inventory})
}
