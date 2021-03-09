use chrono;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::models::schema::*;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "nodes"]
pub struct Node {
    pub id: i64,
    pub lat: f32,
    pub lon: f32,
    pub type_: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug)]
#[table_name = "ways_nodes"]
pub struct WaysToNodes {
    pub way_id: i64,
    pub node_id: i64,
    pub node_order: i16,
}

impl std::cmp::PartialEq for WaysToNodes // for unique()
{
    fn eq(&self, other: &Self) -> bool {
        self.way_id == other.way_id && self.node_id == other.node_id
    }
}
impl std::cmp::Eq for WaysToNodes {}

impl std::cmp::Ord for WaysToNodes // for dedup
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.way_id.cmp(&other.way_id) {
            std::cmp::Ordering::Equal => self.node_id.cmp(&other.node_id),
            o => o,
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

#[derive(Serialize, Deserialize, Clone, Default, Debug, Insertable)]
#[table_name = "teams"]
pub struct WebTeam {
    pub team_id: i32,
    pub name: String,
    pub phrase: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Queryable, Debug, Identifiable)]
pub struct Team {
    pub id: i32,
    pub team_id: i32,
    pub name: String,
    pub phrase: String,
    pub position: i64,
    pub is_tester: bool,
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

#[derive(Serialize, Deserialize, Clone, Queryable, Insertable, Debug)]
#[table_name = "bonuses"]
pub struct Bonus {
    pub url: String,
    pub label: String,
    pub description: Option<String>,
    pub display_time: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Default, Queryable, Insertable)]
#[table_name = "nodes_items"]
pub struct NodeToItem {
    pub node_id: i64,
    pub item_name: String,
}

#[derive(Clone, Default, Queryable, Insertable)]
#[table_name = "teams_items"]
pub struct TeamToItem {
    pub team_id: i32,
    pub item_name: String,
    pub timestamp: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, Default, Debug, PartialEq, Queryable)]
pub struct TeamItem {
    pub type_: String, // puzzles | badge | message
    pub url: String,
    pub level: i16,
    pub name: String,
    pub description: Option<String>,
    pub timestamp: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, Default, Debug, PartialEq, Queryable)]
pub struct TeamStandingsItem {
    pub team_name: String,
    pub type_: Option<String>, // puzzles | badge | message
    pub level: Option<i16>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub timestamp: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, Default, Debug, PartialEq, Queryable)]
pub struct TeamPosition {
    pub team_name: String,
    pub lat: f32,
    pub lon: f32,
    pub level: Option<i16>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, Insertable)]
#[table_name = "messages"]
pub struct WebMessage {
    pub content: String,
    pub type_: String, // success | fail | info
    // Preparation for future-appearing messages
    pub timestamp: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Clone, Default, Queryable)]
pub struct Message {
    pub id: i32,
    pub content: String,
    pub type_: String, // success | fail | info
    pub timestamp: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Clone, Default, Insertable)]
#[table_name = "messages_teams"]
pub struct MessageToTeam {
    pub message_id: i32,
    pub team_id: i32,
}

#[derive(Clone, Default, Debug, PartialEq, Queryable)]
pub struct TeamBadge {
    pub team_name: String,
    pub badge_name: Option<String>,
    pub timestamp: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, Default, Debug, PartialEq, Queryable)]
pub struct ItemTeam {
    pub item_name: String,
    pub item_type: String,
    pub item_level: i16,
    pub team_name: Option<String>,
    pub timestamp: Option<chrono::NaiveDateTime>,
}
