
use rocket::http::RawStr;
use std::vec::Vec;
use super::api_models::*;

const FILLOVA_X_BROZIKOVA_NODE_ID: &str = "3750367566";
const ZOOM: i32 = 17;
const CENTER_X: i32 = 71586;
const CENTER_Y: i32 = 44885;

pub fn get_pois(phrase: &RawStr) -> Pois
{
    let nodes = Vec::new();
    let ways = Vec::new();
    Pois{nodes,ways}
}

pub fn get_grid(phrase: &RawStr) -> Grid
{
    let mut tiles = Vec::new();
    for x in 0..3
    {
        for y in 0..3
        {
            let url = format!("/tiles/{}/{}/{}.png", ZOOM, CENTER_X - 1 + x, CENTER_Y - 1 + y);
            tiles.push(Tile{url, x, y, zoom: ZOOM});
        }
    }
    Grid{columns:3, rows:3, columnWidth:256, rowHeight:256, tiles}
}

pub fn get_info(phrase: &RawStr) -> TeamState
{
    TeamState{ranking: 2, leader: "Maštěné Ředkvičky".to_string(), timeBehind: "00:22:00".to_string(), position: FILLOVA_X_BROZIKOVA_NODE_ID.to_string()}
}

