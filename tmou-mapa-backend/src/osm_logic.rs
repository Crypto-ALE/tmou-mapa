use super::osm_models::*;
use itertools::Itertools;

pub fn get_ways_going_through_node_id<'a>(osm:&'a Osm, node_id:String)-> Vec<&'a Way>
{
    let node_id_copy = node_id.clone(); // couldn't make node_id to be borrowed into this closure
    osm.ways.iter()
            .filter(move |(_,w)| w.nodes.iter().any(|n| *n == node_id_copy))
            .map(|(_,v)| v)
            .collect()
}

fn get_node_ids_in_ways<'a>(ways: &Vec<&'a Way>) -> impl Iterator<Item = String>
{
    ways.into_iter().map(|w| w.nodes.iter()).flatten().map(|s| s.clone()).collect::<Vec<String>>().into_iter().unique()
}

pub fn get_nodes_in_ways<'a>(osm: &'a Osm, ways: &Vec<&'a Way>) -> Vec<&'a Node>
{
    let node_ids = get_node_ids_in_ways(ways);
    node_ids.filter_map(|id| osm.nodes.get(&id)).collect()
}

pub fn get_reachable_ways_for_node_id<'a>(osm:&'a Osm, node_id:String)-> Vec<&'a Way>
{
    let ways_level_0 = get_ways_going_through_node_id(&osm, node_id);
    let node_ids_level_0 = get_node_ids_in_ways(&ways_level_0);
    let ways_level_1 = node_ids_level_0.map(|n| get_ways_going_through_node_id(&osm, n));
    ways_level_1.flatten().unique().collect()
}

