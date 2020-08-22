use diesel::{Queryable, Identifiable, Insertable};
use serde::{Deserialize, Serialize};
use super::schema::*;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "nodes"]
pub struct Node {
    pub id: i64,
    pub lat: String,
    pub lon: String,
    pub type_: String
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

#[derive(Serialize, Deserialize, Clone, Default, Queryable)]
pub struct Item {
    pub r#type: String, // puzzles | badge | message
    pub url: String,
    pub level: i32,
    pub label: String,
    pub description: String,
}
