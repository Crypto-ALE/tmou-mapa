
use rocket::http::RawStr;
use std::vec::Vec;
use super::api_models as api;
use super::db_models as db;
use super::db_controller::{DbControl};
use super::osm_models as osm;
use super::osm_reader::read_osm_from_file;
use super::osm_logic::*;
use super::errors::*;
use super::map_contents::*;
use std::env;
use super::db_models;

const FILLOVA_X_BROZIKOVA_NODE_ID: i64 = 3750367566;

////////////////////////////////////////////////////////////////////
/// Interface
////////////////////////////////////////////////////////////////////

pub fn get_pois(position: i64) -> TmouResult<api::Pois>
{
    let osm = get_osm();
    let osm_ways= get_reachable_ways_for_node_id(&osm, position);
    let osm_nodes = get_nodes_in_ways(&osm, &osm_ways);
    let nodes = osm_nodes.iter().map(|n| node_osm_to_api(n)).collect();
    let ways = osm_ways.iter().map(|w| way_osm_to_api(w)).collect();
    Ok(api::Pois{nodes,ways})
}


pub fn get_info(conn: impl DbControl, team: db_models::Team) -> TmouResult<api::TeamInfo>
{
    let state = get_team_state(conn, team.id)?;
    let pois = get_pois(team.position)?;
    let items = api::Items{items:Vec::new()};
    Ok(api::TeamInfo{state: state, pois: pois, items: items})
}



fn get_team_state(conn: impl DbControl, id: i32) -> TmouResult<api::TeamState>
{
    match conn.get_team(id)
    {
        Some(t) => Ok(team_db_to_api(&t)),
        None => Err(TmouError{message:"Team does not exist".to_string(), response:404})
    }
}

pub fn go_to_node(mut conn: impl DbControl, team: db_models::Team, pos: i64) -> TmouResult<api::TeamInfo>
{
    conn.update_team_position(&team, pos);
    get_info(conn, team)
}

#[allow(unused)]
pub fn discover_node(node_id: i64) -> TmouResult<api::Items>
{
    let db_items = get_contents_of_node(node_id)?;
    let items = db_items.into_iter().map(|i| i.into()).collect();
    Ok(api::Items{items: items})
}

////////////////////////////////////////////////////////////////////
/// Implementation details
////////////////////////////////////////////////////////////////////


fn get_default_team(phrase: &str) -> db::Team
{
    db::Team{
        id: 1,
        team_id: 1,
        phrase:phrase.to_string(),
        name: "Maštěné Ředkvičky".to_string(),
        position: FILLOVA_X_BROZIKOVA_NODE_ID
    }
}

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

impl From<db::Item> for api::Item
{
    fn from(value: db::Item) -> Self
    {
        api::Item{
            r#type: value.r#type.clone(),
            url: value.url.clone(),
            level: value.level,
            label: value.label.clone(),
            description: value.description.clone()
        }
    }
}
// candidate for traits?

fn way_osm_to_api(w: &osm::Way)->api::Way
{
    api::Way{
        id: w.id.to_string(),
        nodes:w.nodes.iter().map(|n| n.to_string()).collect()
    }
}

fn node_osm_to_api(n: &osm::Node)->api::Node
{
    api::Node{
        id: n.id.to_string(),
        y:n.lat.clone(),
        x:n.lon.clone(),
        r#type:n.r#type.clone(),
        data: "<none>".to_string()
    }
}


pub fn initialize()
{
    get_osm();
}

fn get_osm() ->  &'static osm::Osm
{
    &OSM_STATIC
}

fn create_osm() -> TmouResult<osm::Osm>
{
    println!("reading OSM File");
    let fname = env::current_dir()?.join("pubfiles/tiles/osmdata.xml");
    println!("OSM Filename: {}", fname.display());
    let osm = read_osm_from_file(fname.to_str().unwrap());
    println!("Finished reading");
    osm
}

lazy_static!
{
    static ref OSM_STATIC:osm::Osm =
    {
        let value = create_osm().unwrap();
        value
    };
}
