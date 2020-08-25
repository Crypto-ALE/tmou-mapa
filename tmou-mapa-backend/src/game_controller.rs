
use std::vec::Vec;
use super::api_models as api;
use super::db_models as db;
use super::db_controller::{DbControl};
use super::errors::*;
use super::map_contents::*;
use itertools::*;

// const FILLOVA_X_BROZIKOVA_NODE_ID: i64 = 3750367566;

////////////////////////////////////////////////////////////////////
/// Interface
////////////////////////////////////////////////////////////////////

pub fn get_pois(conn: &impl DbControl, position: i64) -> TmouResult<api::Pois>
{
    let db_pois = conn.get_reachable_nodes(position)?;
    let nodes = db_pois.nodes
        .iter()
        .map(|n| n.into())
        .collect();
    let ways = db_pois.ways_to_nodes
        .into_iter()
        .map(|w2n| (w2n.way_id, w2n.node_id))
        .into_group_map()
        .into_iter()
        .map(|(k,v)| api::Way{id: k, nodes: v })
        .collect();
    Ok(api::Pois{nodes,ways})
}


pub fn get_info(conn: &impl DbControl, team: db::Team) -> TmouResult<api::TeamInfo>
{
    let state = get_team_state(conn, team.id)?;
    let pois = get_pois(conn, team.position)?;
    let items = api::Items{items:Vec::new()};
    Ok(api::TeamInfo{state: state, pois: pois, items: items})
}



fn get_team_state(conn: &impl DbControl, id: i32) -> TmouResult<api::TeamState>
{
    match conn.get_team(id)
    {
        Some(t) => Ok(team_db_to_api(&t)),
        None => Err(TmouError{message:"Team does not exist".to_string(), response:404})
    }
}

pub fn go_to_node(conn: & mut impl DbControl, team: db::Team, pos: i64) -> TmouResult<api::TeamInfo>
{
    conn.update_team_position(&team, pos)?;
    get_info(conn, team)
}

#[allow(unused)]
pub fn discover_node(conn: &impl DbControl, node_id: i64) -> TmouResult<api::Items>
{
    //   let db_items = get_contents_of_node(node_id)?;
    let db_items = conn.get_items_in_node(node_id)?;
    let items = db_items.iter().map(|i| i.into()).collect();
    Ok(api::Items{items: items})
}

////////////////////////////////////////////////////////////////////
/// Implementation details
////////////////////////////////////////////////////////////////////

fn team_db_to_api(t: &db::Team)->api::TeamState
{
    api::TeamState{
        name: t.name.clone(),
        position:t.position,
        ranking: 2,
        leader:"Bazinga".to_string(),
        timeBehind:"00:22:00".to_string()
    }
}

impl From<&db::Item> for api::Item
{
    fn from(value: &db::Item) -> Self
    {
        api::Item{
            r#type: value.type_.clone(),
            url: value.url.clone(),
            level: value.level,
            label: value.name.clone(),
            description: match &value.description { Some(d) => d.clone(), None => "".to_string()}
        }
    }
}

impl From<&db::Node> for api::Node
{
    fn from(value: &db::Node) -> Self
    {
        api::Node{
            id: value.id,
            y:value.lat.clone(),
            x:value.lon.clone(),
            r#type:value.type_.clone(),
            data: "<none>".to_string() 
        }
    }
}
