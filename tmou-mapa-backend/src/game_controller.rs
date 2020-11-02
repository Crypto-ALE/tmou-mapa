
use super::api_models as api;
use super::db_models as db;
use super::db_controller::{DbControl, MessagesDbControl};
use super::errors::*;
use itertools::*;
use super::discovery as disc;
use chrono::Utc;
use super::skip;
use super::message_controller::send_message_to_team;

// const FILLOVA_X_BROZIKOVA_NODE_ID: i64 = 3750367566;

////////////////////////////////////////////////////////////////////
/// Interface
////////////////////////////////////////////////////////////////////

pub fn get_pois_for_team(db_control: &impl DbControl, position: i64) -> TmouResult<api::Pois>
{
    let db_pois = db_control.get_reachable_nodes(position)?;
    let nodes: Vec<api::Node> = db_pois.nodes
        .iter()
        .map(|n| n.into())
        .collect();
    let ways: Vec<api::Way> = db_pois.ways_to_nodes
        .into_iter()
        .map(|w2n| (w2n.way_id, w2n.node_id))
        .into_group_map()
        .into_iter()
        .map(|(k,v)| api::Way{id: k, nodes: v })
        .collect();
    Ok(api::Pois{nodes,ways})
}

fn get_team_state(db_control: &impl DbControl, id: i32) -> TmouResult<api::Team>
{
    match db_control.get_team(id)
    {
        Some(t) => Ok((&t).into()),
        None => Err(TmouError{message:"Team does not exist".to_string(), response:404})
    }
}

fn get_items_for_team(db_control: &impl DbControl, id: i32) -> TmouResult<api::Items>
{
    let db_items = db_control.get_team_items_with_timestamps(id)?;
    let items = db_items.iter().map(|i| i.into()).collect();
    Ok(api::Items{items})
}

pub fn get_info(db_control: &impl DbControl, team: db::Team) -> TmouResult<api::TeamInfo>
{
    let state = get_team_state(db_control, team.id)?;
    let pois = get_pois_for_team(db_control, team.position)?;
    let items = get_items_for_team(db_control, team.id)?;
    Ok(api::TeamInfo{state: state, pois: pois, items: items})
}

pub fn get_bonuses(db_control: &impl DbControl) -> TmouResult<Vec<api::Bonus>> {
    let db_bonuses = db_control.get_bonuses()?;
    Ok(db_bonuses.iter().map_into().collect())
}

pub fn go_to_node(db_control: & mut impl DbControl, team: db::Team, pos: i64) -> TmouResult<api::TeamInfo>
{
    let updated_team = db_control.update_team_position(&team, pos)?;
    get_info(db_control, updated_team)
}

fn items_to_api_items(items: &disc::Items) -> Vec<api::Item>
{
    items.iter().map(|i| i.into()).collect()
}

fn event_to_api_event(event: &disc::EventType) -> String
{
    match event
    {
        disc::EventType::CheckpointStartVisited => "checkpoint-start-visited".to_string(),
        disc::EventType::PuzzlesFound=> "puzzles-found".to_string(),
        disc::EventType::BadgeFound => "badge-found".to_string(),
        disc::EventType::Nothing => "nothing".to_string()
    }
}

fn has_new_puzzle(items: & disc::Items) -> bool
{
    items.iter().any(|i| i.type_ == "puzzles".to_string())
}

fn send_puzzle_welcome_message<T: DbControl + MessagesDbControl>( 
    db_control: & mut T,
    inventory: disc::Items, 
    team:db::Team) -> TmouResult<()>
{
    let game_state = db_control.get_game_state_by_puzzles()?;
    let msg = disc::get_puzzle_welcome_message(game_state, inventory)?;
    let api_msg = api::Message{content: msg, r#type: String::from("info"), timestamp:None};
    send_message_to_team(db_control, team, api_msg)
}

pub fn discover_node<T: DbControl + MessagesDbControl>(
    db_control: & mut T,
    team: db::Team) -> TmouResult<api::DiscoveryEvent>
{
    let node_contents = db_control.get_items_in_node(team.position)?;
    let team_inventory = db_control.get_team_items(team.id)?;
    let evt = disc::discover_node(Utc::now(), &team_inventory, &node_contents)?;
    let inventory = evt.updated_inventory.clone();
    db_control.put_team_items(team.id, evt.updated_inventory)?;
    if has_new_puzzle(&evt.newly_discovered_items)
    {
        send_puzzle_welcome_message(db_control, inventory, team)?;
    }
    let api_event = event_to_api_event(&evt.event);
    let api_newly_discovered_items = items_to_api_items(&evt.newly_discovered_items);
    Ok(api::DiscoveryEvent{event: api_event, newItems: api_newly_discovered_items})
}

pub fn discover_post(
    db_control: & mut impl DbControl,
    team: db::Team,
    puzzle_name: &String) -> TmouResult<Vec<api::Item>>
{
    let node_contents = db_control.get_items_in_node(team.position)?;
    let team_inventory = db_control.get_team_items(team.id)?;
    let updated = disc::discover_fake_puzzle(Utc::now(), &team_inventory, &node_contents, &puzzle_name)?;
    db_control.put_team_items(team.id, updated.clone())?;
    let api_updated = items_to_api_items(&updated);
    Ok(api_updated)
}

pub fn is_skip_allowed(db_control: & impl DbControl, team: db::Team) -> TmouResult<api::Skip> {
    let team_items = db_control.get_team_items(team.id)?;
    let grouped_items = team_items.iter().map(|item| (item.type_.clone(), item)).into_group_map();
    // TODO what about puzzles fakes?
    let max_puzzle_level = grouped_items.get("puzzles").unwrap_or(&Vec::new()).iter().map(|item| item.level as usize).max().unwrap_or(0);
    let badges_count = grouped_items.get("badge").and_then(|bdgs| Some(bdgs.len())).unwrap_or(0);
    let game_state = db_control.get_game_state_by_puzzles()?;
    let allowed = skip::is_allowed(max_puzzle_level, badges_count, game_state)?;

    return Ok(api::Skip {
        allowed,
    })
}

////////////////////////////////////////////////////////////////////
/// Implementation details
////////////////////////////////////////////////////////////////////

impl From<&db::Team> for api::Team
{
    fn from(value: &db::Team) -> Self
    {
        api::Team{
            name: value.name.clone(),
            position:value.position
        }
    }
}

impl From<&db::Item> for api::Item
{
    fn from(value: &db::Item) -> Self
    {
        api::Item{
            r#type: value.type_.clone(),
            url: value.url.clone(),
            level: value.level,
            name: value.name.clone(),
            description: match &value.description { Some(d) => d.clone(), None => "".to_string()},
            timestamp:None
        }
    }
}

impl From<&db::TeamItem> for api::Item
{
    fn from(value: &db::TeamItem) -> Self
    {
        api::Item{
            r#type: value.type_.clone(),
            url: value.url.clone(),
            level: value.level,
            name: value.name.clone(),
            description: match &value.description { Some(d) => d.clone(), None => "".to_string()},
            timestamp: value.timestamp
        }
    }
}

impl From<&db::Node> for api::Node
{
    fn from(value: &db::Node) -> Self
    {
        api::Node{
            id: value.id,
            y:value.lat.clone(),
            x:value.lon.clone(),
            r#type:value.type_.clone(),
            data: "<none>".to_string()
        }
    }
}

impl From<&db::Bonus> for api::Bonus
{
    fn from(value: &db::Bonus) -> Self
    {
        api::Bonus{
            url: value.url.clone(),
            label: value.label.clone(),
            description: value.description.clone(),
        }
    }
}
