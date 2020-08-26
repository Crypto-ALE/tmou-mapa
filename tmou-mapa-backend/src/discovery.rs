use super::errors::*;
use super::db_models as db;

type Items = Vec<db::Item>;

fn item_type_order(item: &db::Item) -> i32
{
    match item.type_.as_ref()
    {
        "shop" => 0,
        "badge" => 1,
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

fn get_puzzles(level: i16, itinerary: Items, shop: &Items) -> Items
{
    let eligible_for_new_puzzles = badges_count(&itinerary, level) == max_badges_in_level(level);
    let new_puzzles = shop.iter().find(|i| (i.level == level+1) && (i.type_ == "puzzles".to_string()));
    match eligible_for_new_puzzles && new_puzzles.is_some()
    {
        true => 
        {
            let mut res = itinerary.clone();
            res.push(new_puzzles.unwrap().clone());
            res
        },
        false => itinerary
    }
}

fn add_badge(level: i16, itinerary: Items, it: &db::Item) -> Items
{

    match (level == it.level) && (badges_count(&itinerary, level) < max_badges_in_level(level)) && !itinerary.contains(it)
    {
        true =>
        {
            let mut res = itinerary.clone();
            res.push(it.clone());
            res
        },
        false => itinerary
    }
}    


pub fn discover_node(itinerary: &Items, node_contents: &Items) -> TmouResult<(Items, Items)>
{
    let player_level = itinerary.iter().map(|item| item.level).max().or(Some(0)).unwrap();
    let mut sorted_contents = node_contents.clone();
    sorted_contents.sort_by(|a, b| item_type_order(&a).partial_cmp(&item_type_order(&b)).unwrap());
    let mut new_iti = itinerary[..].to_vec();
    let mut discovered = Vec::new();
    for item in sorted_contents.iter()
    {
        if item.level > player_level 
        { 
            continue;
        }
        let mut was_discovered = false;
        new_iti = match item.type_.as_ref()
        {
            "shop" => 
            { 
                was_discovered = true; 
                get_puzzles(player_level, new_iti, &sorted_contents)
            },
            "badge" => 
            {
                was_discovered = true;
                add_badge(player_level, new_iti, &item)
            }
            _ => new_iti // no change
        };

        if was_discovered
        {
            discovered.push(item.clone());
        }
    }

    Ok((new_iti, discovered))
}