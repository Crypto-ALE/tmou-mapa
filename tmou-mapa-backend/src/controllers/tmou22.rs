// file for game-specific logic for TMOU 22
use crate::models::api;
use crate::models::errors::*;
use super::get_player_level_api;
use std::collections::HashSet;

pub fn filter_pois_by_tag(pois: api::Pois, items: &api::Items) -> TmouResult<api::Pois>
{
    let level = get_player_level_api(&items.items);
    let has11:bool = items.items.iter().any(|i| ->bool { i.name == "puzzles-11"});
    let predicate = match (level, has11) {
        (-1,_) => |s:&String| s == "Europe",
        (0,false) => |s:&String|s == "Europe",
        (0,true) => |s:&String| s == "Europe" || s == "Sifra11",
        (1,_) => |s:&String| s == "Europe" || s == "" || 
                             s == "Europe,Africa" || s == "Africa",
        (2,_) => |s:&String| s == "Europe" || s == "" || 
                             s == "Europe,Africa" || s == "Africa" ||
                             s == "Africa,Asia" || s == "Asia",
        (3,_) => |s:&String| s == "Europe" || s == "" || 
                             s == "Europe,Africa" || s == "Africa" ||
                             s == "Africa,Asia" || s == "Asia" ||
                             s == "Asia,Australia" || s == "Australia",
        _ => |_:&String| true
    };
    let api::Pois{nodes:n, ways:w} = pois;
    let empty:Option<String> = Some(String::from(""));
    let node_ids:HashSet<i64> = n.iter()
        .filter(|n| predicate(&n.tag.as_ref().or(empty.as_ref()).unwrap()))
        .map(|n| n.id)
        .collect();
    let ways:Vec<api::Way> = w.into_iter()
        .filter(|w| predicate(&w.tag.as_ref().or(empty.as_ref()).unwrap()))
        .filter(|w| w.nodes.iter().all(|nid| node_ids.contains(nid)))
        .collect();
    let used_node_ids: HashSet<i64> = ways.iter()
        .map(|w| w.nodes.clone())
        .flatten()
        .collect();
    let nodes = n.into_iter()
        .filter(|n| node_ids.contains(&n.id) && used_node_ids.contains(&n.id))
        .collect();
    Ok(api::Pois{nodes, ways})
}
