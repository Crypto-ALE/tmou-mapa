
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

pub fn get_pois_for_team(db_control: &impl DbControl, position: i64) -> TmouResult<api::Pois>
{
    let db_pois = db_control.get_reachable_nodes(position)?;
    let nodes: Vec<api::Node> = db_pois.nodes
        .iter()
        .map(|n| n.into())
        .collect();
    let ways: Vec<api::Way> = db_pois.ways_to_nodes
        .into_iter()
        .map(|w2n| (w2n.way_id, w2n.node_id))
        .into_group_map()
        .into_iter()
        .map(|(k,v)| api::Way{id: k, nodes: v })
        .collect();
    Ok(api::Pois{nodes,ways})
}

fn get_team_state(db_control: &impl DbControl, id: i32) -> TmouResult<api::Team>
{
    match db_control.get_team(id)
    {
        Some(t) => Ok((&t).into()),
        None => Err(TmouError{message:"Team does not exist".to_string(), response:404})
    }
}

fn get_items_for_team(db_control: &impl DbControl, id: i32) -> TmouResult<api::Items>
{
    let db_items = db_control.get_team_items(id)?;
    let items = db_items.iter().map(|i| i.into()).collect();
    Ok(api::Items{items})
}

pub fn get_info(db_control: &impl DbControl, team: db::Team) -> TmouResult<api::TeamInfo>
{
    let state = get_team_state(db_control, team.id)?;
    let pois = get_pois_for_team(db_control, team.position)?;
    let items = get_items_for_team(db_control, team.id)?;
    Ok(api::TeamInfo{state: state, pois: pois, items: items})
}





pub fn go_to_node(db_control: & mut impl DbControl, team: db::Team, pos: i64) -> TmouResult<api::TeamInfo>
{
    let updated_team = db_control.update_team_position(&team, pos)?;
    get_info(db_control, updated_team)
}

pub fn discover_node(db_control: & mut impl DbControl, team: db::Team) -> TmouResult<api::DiscoveryEvent>
{
    let node_contents = db_control.get_items_in_node(team.position)?;
    let team_inventory = db_control.get_team_items(team.id)?;
    match disc::discover_node(&team_inventory, &node_contents)
    {
        Ok((new_inv, discovered)) =>
        {
            db_control.put_team_items(team.id, new_inv)?;
            let disc_as_api_items = discovered.iter().map(|i| i.into()).collect();
            let mut event = "nothing".to_string();
            for i in discovered.iter() {
                if i.type_.eq("checkpoint") {
                    // whenever there is a checkpoint, everything else comes from it
                    event = "checkpoint-visited".to_string();
                    //TODO need to push last added puzzle, but have no idea how
                    break;
                } else if i.type_.eq("badge") {
                    event = "badge-found".to_string();
                }
            }
            let disc_event = api::DiscoveryEvent {
                event,
                newItems: disc_as_api_items,
            };

            Ok(disc_event)
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
