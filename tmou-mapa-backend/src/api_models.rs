#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NodeAction
{
   pub nodeId: i64
}

#[derive(Serialize, Deserialize)]
pub struct Node
{
   pub id: i64,
   pub x: f32,
   pub y: f32,
   pub r#type: String,
   pub data: String
}

#[derive(Serialize, Deserialize)]
pub struct Item
{
    pub r#type: String, // puzzles | badge | message
    pub url: String,
    pub level: i16,
    pub label: String,
    pub description: String,
}


#[derive(Serialize, Deserialize)]
pub struct Items
{
    pub items: Vec<Item>
}

#[derive(Serialize, Deserialize)]
pub struct Phrase
{
    pub value: String
}


#[derive(Serialize, Deserialize)]
pub struct Way
{
    pub id: i64,
    pub nodes: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Pois
{
   pub nodes: Vec<Node>,
   pub ways: Vec<Way>
}

#[derive(Serialize, Deserialize)]
pub struct TeamState
{
   pub name: String,
   pub ranking: i32,
   pub leader: String,
   pub timeBehind: String,
   pub position: i64
}

#[derive(Serialize, Deserialize)]
pub struct TeamInfo
{
    pub state: TeamState,
    pub pois: Pois,
    pub items: Items
}
