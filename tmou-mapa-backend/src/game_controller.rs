
use rocket::http::RawStr;
use std::vec::Vec;
use super::api_models as api;
use super::db_models as db;
use super::db_controller::{DbControl,MemoryDbControl};
use super::errors::*;

const FILLOVA_X_BROZIKOVA_NODE_ID: &str = "3750367566";
const ZOOM: i32 = 17;
const CENTER_X: i32 = 71586;
const CENTER_Y: i32 = 44885;

pub fn get_pois(phrase: &RawStr) -> TmouResult<api::Pois>
{
    let nodes = Vec::new();
    let ways = Vec::new();
    Ok(api::Pois{nodes,ways})
}

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

pub fn get_info(phrase: &RawStr) -> TmouResult<api::TeamState>
{
    let mut ctrl = get_memory_db_control();
    match ctrl.get_team(phrase)
    {
        None => 
        {
            let t = get_default_team(phrase);
            ctrl.put_team(t.clone())?;
            Ok(db_to_api(&t))
        },
        Some(t) => Ok(db_to_api(t))
    }
}

fn get_default_team(phrase: &str) -> db::Team
{
    db::Team{
        phrase:phrase.to_string(), 
        name: "Maštěné Ředkvičky".to_string(), 
        position: FILLOVA_X_BROZIKOVA_NODE_ID.to_string()
    }
}

fn db_to_api(t: &db::Team)->api::TeamState
{
    api::TeamState{
        name: t.name.clone(), 
        position:t.position.clone(), 
        ranking: 2, 
        leader:"Bazinga".to_string(), 
        timeBehind:"00:22:00".to_string() 
    }
}

// temporary in-memory db

//static mut g_db_ctrl: Option<Box<MemoryDbControl>> = None;

fn get_memory_db_control() -> impl DbControl
{
    let mut ctrl = MemoryDbControl::new();
    ctrl.init(concat!(env!("CARGO_MANIFEST_DIR"), r"\memory_db.json"));
    ctrl
}

