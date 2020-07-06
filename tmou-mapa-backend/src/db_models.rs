
use serde::{Serialize, Deserialize};


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
    pub nodes: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct Poi
{
    pub nodes: Vec<Node>,
    pub ways: Vec<Way>
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Team 
{
    pub phrase: String,
    pub name: String,
    pub position: String,
}

