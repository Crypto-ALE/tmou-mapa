use std::collections::HashMap;

pub struct Node
{
   pub id: i64,
   pub lat: f32,
   pub lon: f32,
   pub r#type: String
}

pub struct Way
{
    pub id: i64,
    pub nodes: Vec<i64>,
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
    pub nodes: HashMap<i64, Node>,
    pub ways: HashMap<i64, Way>
}