use diesel::{Queryable, Identifiable};
use serde::{Deserialize, Serialize};
use super::schema::*;

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub lat: f32,
    pub lon: f32,
    pub r#type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Way {
    pub name: String,
    pub closed: bool,
    pub nodes: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Poi {
    pub nodes: Vec<Node>,
    pub ways: Vec<Way>,
}

#[derive(Serialize, Deserialize, Clone, Default, Queryable, Debug, Identifiable)]
pub struct Team {
    pub id: i32,
    pub team_id: i32,
    pub name: String,
    pub phrase: String,
    pub position: i64,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct NodeContents {
    pub r#type: String,
    pub data: String,
}
