use super::errors::*;
use super::db_models as db;

type Items = Vec<db::Item>;

// order of discovery when multiple items found on place
// Badges before checkpoints so that when a badge is found it can be redeemed in checkpoint
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

// returns new inventory 
// either the same when conditions for redeeming badges for new set of puzzles not met, or with added puzzles
fn get_puzzles(level: i16, inventory: Items, checkpoint: &Items) -> Items
{
    let eligible_for_new_puzzles = badges_count(&inventory, level) == max_badges_in_level(level);
    let new_puzzles = checkpoint.iter().find(|i| (i.level == level+1) && (i.type_ == "puzzles".to_string()));
    match eligible_for_new_puzzles && new_puzzles.is_some()
    {
        true => 
        {
            let mut res = inventory.clone();
            res.push(new_puzzles.unwrap().clone());
            res
        },
        false => inventory
    }
}

// returns new inventory 
// either the same when conditions for discovering not met, or with added badge
fn add_badge(level: i16, inventory: Items, it: &db::Item) -> Items
{

    match (level == it.level) && (badges_count(&inventory, level) < max_badges_in_level(level)) && !inventory.contains(it)
    {
        true =>
        {
            let mut res = inventory.clone();
            res.push(it.clone());
            res
        },
        false => inventory
    }
}    


pub fn discover_node(inventory: &Items, node_contents: &Items) -> TmouResult<(Items, Items)>
{
    let player_level = inventory.iter().map(|item| item.level).max().or(Some(0)).unwrap();
    // sort node contents so that:
    // 1. evaluation is deterministic
    // 2. badges are discovered before checkpoints (see fn item_type_order) to allow redeeming badge in checkpoint instantly
    let mut sorted_contents = node_contents.clone();
    sorted_contents.sort_by(|a, b| item_type_order(&a).partial_cmp(&item_type_order(&b)).unwrap());
    let mut new_inv = inventory[..].to_vec();
    let mut discovered = Vec::new();
    for item in sorted_contents.iter()
    {
        if item.level > player_level 
        { 
            continue;
        }
        let mut was_discovered = false;

        // checkpoints and badges can alter inventory and are discoverable
        // others (puzzles) are unseen to player
        new_inv = match item.type_.as_ref()
        {
            "checkpoint" => 
            { 
                was_discovered = true; 
                get_puzzles(player_level, new_inv, &sorted_contents)
            },
            "badge" => 
            {
                was_discovered = true;
                add_badge(player_level, new_inv, &item)
            }
            _ => new_inv // no change
        };

        if was_discovered
        {
            discovered.push(item.clone());
        }
    }

    Ok((new_inv, discovered))
}