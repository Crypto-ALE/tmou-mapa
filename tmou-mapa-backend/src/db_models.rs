use diesel::{Queryable, Identifiable, Insertable};
use serde::{Deserialize, Serialize};
use super::schema::*;
use chrono;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "nodes"]
pub struct Node {
    pub id: i64,
    pub lat: f32,
    pub lon: f32,
    pub type_: String
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug)]
#[table_name = "ways_nodes"]
pub struct WaysToNodes {
    pub way_id: i64,
    pub node_id: i64,
    pub node_order: i16
}


impl std::cmp::PartialEq for WaysToNodes // for unique()
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.way_id == other.way_id && self.node_id == other.node_id
    }
}
impl std::cmp::Eq for WaysToNodes {}

impl std::cmp::Ord for WaysToNodes // for dedup
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.way_id.cmp(&other.way_id)
        {
            std::cmp::Ordering::Equal => self.node_id.cmp(&other.node_id),
            o => o
        }
    }
}

impl std::cmp::PartialOrd for WaysToNodes // for dedup
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Pois {
    pub nodes: Vec<Node>,
    pub ways_to_nodes: Vec<WaysToNodes>,
}

#[derive(Serialize, Deserialize, Clone, Default, Queryable, Debug, Identifiable)]
pub struct Team {
    pub id: i32,
    pub team_id: i32,
    pub name: String,
    pub phrase: String,
    pub position: i64,
}

#[derive(Serialize, Deserialize, Clone, Default, Queryable, Insertable, Debug, PartialEq)]
#[table_name = "items"]
pub struct Item {
    pub type_: String, // puzzles | badge | message
    pub url: String,
    pub level: i16,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default, Queryable, Insertable)]
#[table_name = "nodes_items"]
pub struct NodeToItem {
    pub node_id: i64,
    pub item_name: String
}

#[derive(Clone, Default, Queryable, Insertable)]
#[table_name = "teams_items"]
pub struct TeamToItem {
    pub team_id: i32,
    pub item_name: String,
    pub timestamp: Option<chrono::NaiveDateTime>
}
