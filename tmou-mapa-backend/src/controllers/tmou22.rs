// file for game-specific logic for TMOU 22
use crate::models::api;
use crate::models::errors::*;
use super::get_player_level_api;

pub fn filter_pois_by_tag(pois: api::Pois, items: &api::Items) -> TmouResult<api::Pois>
{
    let level = get_player_level_api(&items.items);
    let has11:bool = items.items.iter().any(|i| ->bool { i.name == "puzzles-11"});
    type Pred = Box<dyn Fn(&String) -> bool>;
    let predicate:Pred = match (level, has11) {
        (-1,_) => Box::new(|s:&String| -> bool {s == "Europe"}),
        (0,false) => Box::new(|s:&String| -> bool {s == "Europe"}),
        (0,true) => Box::new(|s:&String| -> bool {s == "Europe" || 
             s == "Sifra11"}),
        (1,_) => Box::new(|s:&String| -> bool {s == "Europe" || s == "" || 
                                  s == "Europe,Africa" || s == "Africa"}),
        (2,_) => Box::new(|s:&String| -> bool {s == "Europe" || s == "" || 
                                  s == "Europe,Africa" || s == "Africa" ||
                                  s == "Africa,Asia" || s == "Asia"}),
        (3,_) => Box::new(|s:&String| -> bool {s == "Europe" || s == "" || 
                                  s == "Europe,Africa" || s == "Africa" ||
                                  s == "Africa,Asia" || s == "Asia" ||
                                  s == "Asia,Australia" || s == "Australia"}),
        _ => Box::new(|_:&String| -> bool {true})
    };
    let api::Pois{nodes:n, ways:w} = pois;
    let empty:Option<String> = Some(String::from(""));
    let nodes:Vec<api::Node> = n.into_iter()
        .filter(|n| predicate(&n.tag.as_ref().or(empty.as_ref()).unwrap()))
        .collect();
    let ways:Vec<api::Way> = w.into_iter()
        .filter(|w| predicate(&w.tag.as_ref().or(empty.as_ref()).unwrap()))
        .collect();
    drop(predicate);    
    Ok(api::Pois{nodes, ways})
}
