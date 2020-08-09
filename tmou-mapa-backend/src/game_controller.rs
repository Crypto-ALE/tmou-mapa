
use rocket::http::RawStr;
use std::vec::Vec;
use super::api_models as api;
use super::db_models as db;
use super::db_controller::{DbControl,MemoryDbControl};
use super::osm_models as osm;
use super::osm_reader::read_osm_from_file;
use super::osm_logic::*;
use super::errors::*;
use super::map_contents::*;
use std::env;


const FILLOVA_X_BROZIKOVA_NODE_ID: &str = "3750367566";
const ZOOM: i32 = 17;
const CENTER_X: i32 = 71586;
const CENTER_Y: i32 = 44885;

////////////////////////////////////////////////////////////////////
/// Interface
////////////////////////////////////////////////////////////////////

pub fn get_pois(phrase: &RawStr) -> TmouResult<api::Pois>
{
    let osm = get_osm();
    let team = get_team_state(&phrase)?;
    let osm_ways= get_reachable_ways_for_node_id(&osm, team.position.to_string());
    let osm_nodes = get_nodes_in_ways(&osm, &osm_ways);
    let nodes = osm_nodes.iter().map(|n| node_osm_to_api(n)).collect();
    let ways = osm_ways.iter().map(|w| way_osm_to_api(w)).collect();
    Ok(api::Pois{nodes,ways})
}

#[allow(unused)]
pub fn get_grid(phrase: &RawStr) -> TmouResult<api::Grid>
{
    let mut tiles = Vec::new();
    for x in 0..3
    {
        for y in 0..3
        {
            let url = format!("/tiles/{}/{}/{}.png", ZOOM, CENTER_X - 1 + x, CENTER_Y - 1 + y);
            tiles.push(api::Tile{url, x, y, zoom: ZOOM});
        }
    }
    Ok(api::Grid
        {
            columns:3,
            rows:3,
            columnWidth:256,
            rowHeight:256,
            tiles
        })
}

pub fn get_info(phrase: &RawStr) -> TmouResult<api::TeamInfo>
{
    let state = get_team_state(phrase)?;
    let pois = get_pois(phrase)?;

    Ok(api::TeamInfo{state: state, pois: pois})
}



pub fn get_team_state(phrase: &RawStr) -> TmouResult<api::TeamState>
{
    let mut ctrl = get_memory_db_control()?;
    let t = get_team_or_default(& mut ctrl, phrase)?;
    Ok(team_db_to_api(&t))
}

pub fn go_to_node(phrase: &RawStr, node_id: &String) -> TmouResult<()>
{
    let mut ctrl = get_memory_db_control()?;
    let mut t = get_team_or_default(& mut ctrl, phrase)?;
    t.position = node_id.to_string(); // CHECK!!!
    ctrl.put_team(t)
}

#[allow(unused)]
pub fn discover_node(phrase: &RawStr, node_id: &String) -> TmouResult<api::NodeContents>
{
    let nc = get_contents_of_node(node_id)?;
    Ok(node_contents_db_to_api(&nc))
}

////////////////////////////////////////////////////////////////////
/// Implementation details
////////////////////////////////////////////////////////////////////

fn get_team_or_default(ctrl: & mut impl DbControl, phrase: &RawStr) -> TmouResult<db::Team>
{
    match ctrl.get_team(phrase)
    {
        Some(t) => Ok(t),
        None =>
        {
            let t = get_default_team(phrase);
            ctrl.put_team(t.clone())?;
            Ok(t)
        }
    }

}


fn get_default_team(phrase: &str) -> db::Team
{
    db::Team{
        id: 1,
        team_id: 1,
        phrase:phrase.to_string(),
        name: "Maštěné Ředkvičky".to_string(),
        position: FILLOVA_X_BROZIKOVA_NODE_ID.to_string()
    }
}

fn team_db_to_api(t: &db::Team)->api::TeamState
{
    api::TeamState{
        name: t.name.clone(),
        position:t.position.clone(),
        ranking: 2,
        leader:"Bazinga".to_string(),
        timeBehind:"00:22:00".to_string()
    }
}

fn node_contents_db_to_api(nc: &db::NodeContents)->api::NodeContents
{
    api::NodeContents{
        r#type: nc.r#type.clone(),
        data: nc.data.clone()
    }
}

// candidate for traits?

fn way_osm_to_api(w: &osm::Way)->api::Way
{
    api::Way{
        id: w.id.clone(),
        nodes:w.nodes.clone()
    }
}

fn node_osm_to_api(n: &osm::Node)->api::Node
{
    api::Node{
        id: n.id.clone(),
        y:n.lat.clone(),
        x:n.lon.clone(),
        r#type:n.r#type.clone(),
        data: "<none>".to_string()
    }
}

// temporary in-memory db

//static mut g_db_ctrl: Option<Box<MemoryDbControl>> = None;

fn get_memory_db_control() -> TmouResult<impl DbControl>
{
    let mut ctrl = MemoryDbControl::new();
    let fname = env::current_dir()?.join("memory_db.json");
    ctrl.init(fname.to_str().unwrap())?;
    Ok(ctrl)
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
