#![allow(non_snake_case)]

use std::collections::HashMap;

use ::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NodeAction {
    pub nodeId: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub id: i64,
    pub x: f32,
    pub y: f32,
    pub r#type: String,
    pub data: String,
    pub tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub r#type: String, // puzzles | badge | message
    pub url: String,
    pub level: i16,
    pub name: String,
    pub description: String,
    pub timestamp: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemCreate {
    pub r#type: String,
    pub url: String,
    pub level: i16,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct ItemUpdate {
    pub r#type: Option<String>,
    pub url: Option<String>,
    pub level: Option<i16>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemWithNodes {
    pub nodeIds: Vec<i64>,
    pub r#type: String,
    pub url: String,
    pub level: i16,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct PuzzleName {
    pub puzzleName: String,
}

#[derive(Serialize, Deserialize)]
pub struct Bonus {
    pub url: String,
    pub label: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DiscoveryEvent {
    pub event: String, // checkpoint-visited | badge-found | nothing
    pub newItems: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
pub struct Items {
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
pub struct Phrase {
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Way {
    pub id: i64,
    pub nodes: Vec<i64>,
    pub tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Pois {
    pub nodes: Vec<Node>,
    pub ways: Vec<Way>,
}

#[derive(Serialize, Deserialize)]
pub struct Team {
    pub name: String,
    pub position: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TeamPosition {
    pub team_name: String,
    pub lat: f32,
    pub lon: f32,
    pub level: i16,
}

#[derive(Serialize, Deserialize)]
pub struct TeamInfo {
    pub state: Team,
    pub pois: Pois,
    pub items: Items,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub r#type: String, // fail | success | info
    pub timestamp: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct IncomingMessage {
    pub recipient_id: i32,
    pub message: Message,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct PuzzleResult {
    pub dead: bool,
    pub timestamp: chrono::NaiveDateTime,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamStanding {
    pub rank: u16,
    pub name: String,
    pub puzzles: HashMap<u16, PuzzleResult>,
    pub badge_count: u16,
    pub start_puzzles_solved: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Standings {
    pub standings: Vec<TeamStanding>,
}

#[derive(Serialize, Deserialize)]
pub struct PuzzleStats {
    pub name: String,
    pub solved_by: usize,
    pub first_team: Option<String>,
    pub first_time: Option<chrono::NaiveDateTime>,
    pub fastest_team: Option<String>,
    pub fastest_time: Option<String>, // duration not serializable
    pub median_time: Option<String>,  // duration not serializable
}

#[derive(Serialize, Deserialize)]
pub struct PuzzlesStats {
    pub stats: Vec<PuzzleStats>,
}

#[derive(Serialize, Deserialize)]
pub struct Skip {
    pub allowed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SkipResult {
    pub newItems: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
pub struct SkipAction {
    pub verified: bool,
}
