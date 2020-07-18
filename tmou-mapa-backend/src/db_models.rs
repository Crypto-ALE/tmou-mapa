
use serde::{Serialize, Deserialize};
use std::collections::{HashSet, HashMap};


#[derive(Serialize, Deserialize)]
pub struct Node
{
    pub id: String,
    pub lat: f32,
    pub lon: f32,
    pub r#type: String
}

#[derive(Serialize, Deserialize)]
pub struct Way
{
    pub name: String,
    pub closed: bool,
    pub nodes: HashSet<String>
}

#[derive(Serialize, Deserialize)]
pub struct Poi
{
    pub nodes: HashMap<String, Node>,
    pub ways: HashMap<String, Way>
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Team 
{
    pub phrase: String,
    pub name: String,
    pub position: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct NodeContents 
{
    pub r#type: String,
    pub data: String,
}

