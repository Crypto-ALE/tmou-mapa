
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NodeAction
{
   pub nodeId: String,
   pub action: String
}

#[derive(Serialize, Deserialize)]
pub struct Node
{
   pub id: String,
   pub x: f32,
   pub y: f32,
   pub r#type: String,
   pub data: String
}


#[derive(Serialize, Deserialize)]
pub struct Way
{
   pub nodes: Vec<String>,
   pub name: String,
   pub closed: bool
}

#[derive(Serialize, Deserialize)]
pub struct Pois
{
   pub nodes: Vec<Node>,
   pub ways: Vec<Way>
}


#[derive(Serialize, Deserialize)]
pub struct Tile
{
   pub url: String,
   pub x: i32,
   pub y: i32,
   pub zoom: i32
}

#[derive(Serialize, Deserialize)]
pub struct Grid
{
   pub columns: i32,
   pub rows: i32,
   pub columnWidth: i32,
   pub rowHeight: i32,
   pub tiles: Vec<Tile>
}

#[derive(Serialize, Deserialize)]
pub struct TeamState
{
   pub name: String,
   pub ranking: i32,
   pub leader: String,
   pub timeBehind: String,
   pub position: String
}
