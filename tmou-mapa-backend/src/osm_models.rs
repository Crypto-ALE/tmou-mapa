use std::collections::HashMap;

pub struct Node
{
   pub id: String,
   pub lat: f32,
   pub lon: f32
}

pub struct Way
{
    pub id: String,
    pub nodes: Vec<String>,
}

pub struct Osm
{
    pub nodes: HashMap<String, Node>,
    pub ways: HashMap<String, Way>
}