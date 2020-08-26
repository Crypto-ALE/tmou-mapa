
use super::api_models as api;
use super::db_models as db;
use super::db_controller::{DbControl};
use super::errors::*;
use itertools::*;
use super::discovery as disc;

// const FILLOVA_X_BROZIKOVA_NODE_ID: i64 = 3750367566;

////////////////////////////////////////////////////////////////////
/// Interface
////////////////////////////////////////////////////////////////////

pub fn get_pois_for_team(conn: &impl DbControl, position: i64) -> TmouResult<api::Pois>
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

fn get_team_state(conn: &impl DbControl, id: i32) -> TmouResult<api::Team>
{
    match conn.get_team(id)
    {
        Some(t) => Ok((&t).into()),
        None => Err(TmouError{message:"Team does not exist".to_string(), response:404})
    }
}

fn get_items_for_team(conn: &impl DbControl, id: i32) -> TmouResult<api::Items>
{
    let db_items = conn.get_team_items(id)?;
    let items = db_items.iter().map(|i| i.into()).collect();
    Ok(api::Items{items})
}

pub fn get_info(conn: &impl DbControl, team: db::Team) -> TmouResult<api::TeamInfo>
{
    let state = get_team_state(conn, team.id)?;
    let pois = get_pois_for_team(conn, team.position)?;
    let items = get_items_for_team(conn, team.id)?;
    Ok(api::TeamInfo{state: state, pois: pois, items: items})
}





pub fn go_to_node(conn: & mut impl DbControl, team: db::Team, pos: i64) -> TmouResult<api::TeamInfo>
{
    conn.update_team_position(&team, pos)?;
    get_info(conn, team)
}

pub fn discover_node(conn: & mut impl DbControl, team: db::Team) -> TmouResult<api::Items>
{
    let node_contents = conn.get_items_in_node(team.position)?;
    let team_iti = conn.get_team_items(team.id)?;
    match disc::discover_node(&team_iti, &node_contents)
    {
        Ok((new_iti, discovered)) =>
        {
            conn.put_team_items(team.id, new_iti)?;
            let api_disc = discovered.iter().map(|i| i.into()).collect();
            Ok(api::Items{items: api_disc})
        },
        Err(e) => Err(e)
    }
}

////////////////////////////////////////////////////////////////////
/// Implementation details
////////////////////////////////////////////////////////////////////

impl From<&db::Team> for api::Team
{
    fn from(value: &db::Team) -> Self
    {
        api::Team{
            name: value.name.clone(),
            position:value.position
        }
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
            name: value.name.clone(),
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
