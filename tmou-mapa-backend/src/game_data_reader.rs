use std::fs::read_to_string;

use chrono::NaiveDateTime;

use crate::models::errors::*;
use crate::models::db::{Item, NodeToItem, Bonus};

pub struct GameData {
    pub items: Vec<Item>,
    pub nodes_items: Vec<NodeToItem>,
    pub bonuses: Vec<Bonus>,
}

pub fn read_game_data(path: &str) -> TmouResult<GameData> {
    println!("Reading Game data from {}", path);
    let xml = read_to_string(path)?;
    let doc = roxmltree::Document::parse(&xml)?;
    let game_node = doc.root().first_child().ok_or(TmouError {
        message: "no root node".to_string(),
        response: 404,
    })?;
    let game_name = parse_game(game_node)?;

    println!("Parsing data for {}", game_name);
    
    let items_node: Vec<roxmltree::Node> = game_node
        .children()
        .filter(|n| n.has_tag_name("items"))
        .collect();
    assert_eq!(items_node.len(), 1);
    let (items, nodes_items) = read_items(&items_node[0])?;

    let bonuses_node: Vec<roxmltree::Node> = game_node
        .children()
        .filter(|n| n.has_tag_name("bonuses"))
        .collect();
    assert_eq!(bonuses_node.len(), 1);
    let bonuses = read_bonuses(&bonuses_node[0])?;

    Ok(GameData {
        items,
        nodes_items,
        bonuses,
    })
}


fn read_items(items_node: &roxmltree::Node) -> TmouResult<(Vec<Item>, Vec<NodeToItem>)> {
    let mut items = Vec::new();
    let mut nodes_items = Vec::new();
    for n in items_node.children().filter(|c| c.is_element()) {
        match parse_item(n) {
            Ok((item, node_ids)) => {
                let name = item.name.clone();
                items.push(item);
                for node_id in node_ids {
                    nodes_items.push(NodeToItem {
                        node_id,
                        item_name: name.clone(),
                    })
                }
            }
            Err(e) => println!(
                "malformed node {}: {}, moving on",
                n.tag_name().name(),
                e.message
            ),
        }
    }
    Ok((items, nodes_items))
}

fn read_bonuses(bonuses_node: &roxmltree::Node) -> TmouResult<Vec<Bonus>> {
    let mut bonuses = Vec::new();
    for n in bonuses_node.children().filter(|c| c.is_element()) {
        match parse_bonus(n) {
            Ok(bonus) => bonuses.push(bonus),
            Err(e) => println!(
                "malformed node {}: {}, moving on",
                n.tag_name().name(),
                e.message
            ),
        }
    }

    Ok(bonuses)
}

fn parse_item<'a>(node: roxmltree::Node<'a, 'a>) -> TmouResult<(Item, Vec<i64>)> {
    let type_ = node
        .attribute("type")
        .ok_or(err("type not found"))?
        .to_string();
    let url = node
        .attribute("url")
        .ok_or(err("url not found"))?
        .to_string();
    let level = node
        .attribute("level")
        .and_then(|a| a.parse::<i16>().ok())
        .ok_or(err("missing or malformed level"))?;
    let name = node
        .attribute("name")
        .ok_or(err("name not found"))?
        .to_string();
    let description = node
        .attribute("description")
        .ok_or(err("description not found"))?
        .to_string();
    let mut nodes = Vec::new();
    for n in node
        .children()
        .filter(|c| c.is_element() && c.has_tag_name("node"))
    {
        match n.attribute("id").and_then(|a| a.parse::<i64>().ok()) {
            Some(id) => nodes.push(id),
            _ => (),
        }
    }

    let condition = node
        .children()
        .filter(|c| c.is_element() && c.has_tag_name("condition"))
        .next()
        .and_then(|e| Some(e.text()))
        .and_then(|s| Some(s.unwrap().to_string()));

    Ok((
        Item {
            type_,
            url,
            level,
            name: name,
            description: Some(description),
            condition: condition,
        },
        nodes,
    ))
}

fn parse_bonus<'a>(node: roxmltree::Node<'a, 'a>) -> TmouResult<Bonus> {
    let label = node
        .attribute("label")
        .ok_or(err("label not found"))?
        .to_string();
    let description = node
        .attribute("description")
        .ok_or(err("description not found"))?
        .to_string();
    let url = node
        .attribute("url")
        .ok_or(err("url not found"))?
        .to_string();
    let display_time = node
        .attribute("display-time")
        .and_then(|a| NaiveDateTime::parse_from_str(a, "%Y-%m-%dT%H:%M:%S%z").ok())
        .ok_or(err("missing or malformed display-time"))?;
    Ok(Bonus {
        label,
        description: Some(description),
        url,
        display_time,
    })
}

fn parse_game<'a>(node: roxmltree::Node<'a, 'a>) -> TmouResult<String> {
    assert!(node.has_tag_name("game"));
    match node.attribute("name") {
        Some(a) => Ok(a.to_string()),
        None => Err(err("game node does not have name")),
    }
}

fn err(message: &str) -> TmouError {
    TmouError {
        message: message.to_string(),
        response: 404,
    }
}
