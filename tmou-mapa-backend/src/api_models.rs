#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NodeAction
{
   pub nodeId: i64
}

#[derive(Serialize, Deserialize, Debug)]
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
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct DiscoveryEvent
{
    pub event: String, // checkpoint-visited | badge-found | nothing
    pub newItems: Vec<Item>
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


#[derive(Serialize, Deserialize, Debug)]
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
pub struct Team
{
   pub name: String,
   pub position: i64
}

#[derive(Serialize, Deserialize)]
pub struct TeamInfo
{
    pub state: Team,
    pub pois: Pois,
    pub items: Items
}
