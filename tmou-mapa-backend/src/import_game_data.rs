#[macro_use]
extern crate diesel;

mod models;

use std::env;
use std::fs::read_to_string;

use chrono::NaiveDateTime;
use diesel::insert_into;
use diesel::prelude::*;
use regex::Regex;

use models::db;
use models::errors::*;
use models::schema::bonuses::dsl as bonuses;
use models::schema::items::dsl as items;
use models::schema::nodes_items::dsl as nodes_items;

fn error<T>(message: &str) -> TmouResult<T> {
    Err(TmouError {
        message: message.to_string(),
        response: 404,
    })
}

fn err(message: &str) -> TmouError {
    TmouError {
        message: message.to_string(),
        response: 404,
    }
}

fn parse_game<'a>(node: roxmltree::Node<'a, 'a>) -> TmouResult<String> {
    assert!(node.has_tag_name("game"));
    match node.attribute("name") {
        Some(a) => Ok(a.to_string()),
        None => error("game node does not have name"),
    }
}

fn parse_item<'a>(node: roxmltree::Node<'a, 'a>) -> TmouResult<(db::Item, Vec<i64>)> {
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
        db::Item {
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

fn import_items(conn: &mut PgConnection, items_node: &roxmltree::Node) -> TmouResult<()> {
    let mut db_items = Vec::new();
    let mut db_nodes_items = Vec::new();
    for n in items_node.children().filter(|c| c.is_element()) {
        match parse_item(n) {
            Ok((item, node_ids)) => {
                let name = item.name.clone();
                db_items.push(item);
                for node_id in node_ids {
                    db_nodes_items.push(db::NodeToItem {
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

    println!("Inserting {} items into db", db_items.len());
    match insert_into(items::items).values(db_items).execute(conn) {
        Err(e) => println!("Failed with {}; continuing...", e.to_string()),
        _ => (),
    }

    println!("Inserting {} nodes_items into db", db_nodes_items.len());
    insert_into(nodes_items::nodes_items)
        .values(db_nodes_items)
        .execute(conn)?;
    Ok(())
}

fn parse_bonus<'a>(node: roxmltree::Node<'a, 'a>) -> TmouResult<db::Bonus> {
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
    Ok(db::Bonus {
        label,
        description: Some(description),
        url,
        display_time,
    })
}

fn import_bonuses(conn: &mut PgConnection, bonuses_node: &roxmltree::Node) -> TmouResult<()> {
    let mut db_bonuses = Vec::new();
    for n in bonuses_node.children().filter(|c| c.is_element()) {
        match parse_bonus(n) {
            Ok(bonus) => db_bonuses.push(bonus),
            Err(e) => println!(
                "malformed node {}: {}, moving on",
                n.tag_name().name(),
                e.message
            ),
        }
    }

    println!("Inserting {} bonuses into db", db_bonuses.len());
    match insert_into(bonuses::bonuses)
        .values(db_bonuses)
        .execute(conn)
    {
        Err(e) => println!("Failed with {}; continuing...", e.to_string()),
        _ => (),
    }

    Ok(())
}

fn get_db_connection() -> TmouResult<PgConnection> {
    println!("Reading database url");
    let dbs_str = env::var("ROCKET_DATABASES")?;
    println!("Deserializing {}", dbs_str);
    let re = Regex::new(r#".*"([^"]*)".*"#).unwrap();
    let url = re.captures_iter(&dbs_str).nth(0).unwrap()[1].to_string();
    println!("Connecting to db (url={})", url);
    Ok(PgConnection::establish(&url)?)
}

fn import_game_data(path: &String) -> TmouResult<()> {
    println!("Reading Game data from {}", path);
    let xml = read_to_string(path)?;
    let doc = roxmltree::Document::parse(&xml)?;
    let game_node = doc.root().first_child().ok_or(TmouError {
        message: "no root node".to_string(),
        response: 404,
    })?;
    let game_name = parse_game(game_node)?;

    let mut conn = get_db_connection()?;

    println!("Parsing data for {}", game_name);
    let items: Vec<roxmltree::Node> = game_node
        .children()
        .filter(|n| n.has_tag_name("items"))
        .collect();
    assert_eq!(items.len(), 1);
    import_items(&mut conn, &items[0])?;

    let bonuses: Vec<roxmltree::Node> = game_node
        .children()
        .filter(|n| n.has_tag_name("bonuses"))
        .collect();
    assert_eq!(bonuses.len(), 1);
    import_bonuses(&mut conn, &bonuses[0])?;

    Ok(())
}

fn usage() -> TmouResult<()> {
    println!("Usage:");
    println!("import-game-data <game_data_file_path>");
    Ok(())
}

fn main() {
    println!("Game Data Importer");
    let args: Vec<String> = env::args().collect();
    let res = match args.len() {
        2 => import_game_data(&args[1]),
        _ => usage(),
    };
    match res {
        Ok(_) => println!("Finished successfully"),
        Err(e) => println!("Failed {}", e.message),
    }
}
