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

impl std::cmp::PartialEq for Way // for unique()
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.id == other.id
    }
}
impl std::cmp::Eq for Way {}

impl std::hash::Hash for Way {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

pub struct Osm
{
    pub nodes: HashMap<String, Node>,
    pub ways: HashMap<String, Way>
}