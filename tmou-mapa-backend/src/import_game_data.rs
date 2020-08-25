#[macro_use] extern crate diesel;

mod db_models;
mod errors;
mod schema;

use diesel::prelude::*;
use diesel::insert_into;
use std::env;
use errors::*;
use schema::items::dsl as items;
use schema::nodes_items::dsl as nodes_items;
use std::fs::read_to_string;

use db_models as db;
use regex::Regex;

fn error<T>(message: &str) -> TmouResult<T>
{
    Err(TmouError{message:message.to_string(), response:404})
}

fn err(message: &str) -> TmouError
{
    TmouError{message:message.to_string(), response:404}
}


fn parse_game<'a>(node: roxmltree::Node<'a,'a>) -> TmouResult<String>
{
    assert!(node.has_tag_name("game"));
    match node.attribute("name")
    {
        Some(a) => Ok(a.to_string()),
        None => error("game node does not have name")
    }
}

fn parse_item<'a>(node: roxmltree::Node<'a,'a>) -> TmouResult<(db::Item, Vec<i64>)>
{
    let type_= node.attribute("type").ok_or(err("type not found"))?.to_string();
    let url = node.attribute("url").ok_or(err("url not found"))?.to_string();
    let level = node.attribute("level").and_then(|a| a.parse::<i16>().ok()).ok_or(err("missing or malformed level"))?;
    let name = node.attribute("name").ok_or(err("name not found"))?.to_string();
    let description = node.attribute("description").ok_or(err("description not found"))?.to_string();
    let mut nodes = Vec::new();
    for n in node.children().filter(|c| c.is_element() && c.has_tag_name("node"))
    {
        match n.attribute("id").and_then(|a| a.parse::<i64>().ok())
        {
            Some(id) => nodes.push(id),
            _ => ()
        }
    }
    Ok((db::Item{type_, url, level, name: name, description: Some(description)}, nodes))
}

fn import_game_data(path: &String) -> TmouResult<()>
{
    println!("Reading Game data from {}", path);
    let xml = read_to_string(path)?;
    let doc = roxmltree::Document::parse(&xml)?;
    let game_node = doc.root().first_child().ok_or(TmouError{message: "no root node".to_string(),response: 404})?;
    let game_name = parse_game(game_node)?;
    println!("Parsing data for {}", game_name);
    let items: Vec<roxmltree::Node> = game_node.children().filter(|n| n.has_tag_name("items")).collect();
    assert_eq!(items.len(), 1);
    
    let mut db_items = Vec::new();
    let mut db_nodes_items = Vec::new();
    for n in items[0].children().filter(|c| c.is_element())
    {
        match parse_item(n)
        {
            Ok((item, node_ids)) => 
            {
                let name = item.name.clone();
                db_items.push(item);
                for node_id in node_ids
                {
                    db_nodes_items.push(db::NodeToItem{node_id, item_name: name.clone()})
                }
            },
            Err(e) => println!("malformed node {}: {}, moving on", n.tag_name().name(), e.message)
        }
    }
    println!("Reading database url");
    let dbs_str = env::var("ROCKET_DATABASES")?;
    println!("Deserializing {}", dbs_str);
    let re = Regex::new(r#".*"([^"]*)".*"#).unwrap();
    let url = re.captures_iter(&dbs_str).nth(0).unwrap()[1].to_string();
    println!("Connecting to db (url={})", url);
    let conn = PgConnection::establish(&url).unwrap();
    println!("Inserting {} items into db", db_items.len());
    match insert_into(items::items).values(db_items).execute(&conn)
    {
        Err(e) => println!("Failed with {}; continuing...", e.to_string()),
        _ => ()
    }

    println!("Inserting {} nodes_items into db", db_nodes_items.len());
    insert_into(nodes_items::nodes_items).values(db_nodes_items).execute(&conn)?;
    Ok(())
}

fn usage() -> TmouResult<()>
{
    println!("Usage:");
    println!("import-game-data <game_data_file_path>");
    Ok(())
}



fn main() {
    println!("Game Data Importer");
    let args: Vec<String> = env::args().collect();
    let res = match args.len()
    {
        2 => import_game_data(&args[1]),
        _ => usage()
    };
    match res
    {
        Ok(_) => println!("Finished successfully"),
        Err(e) => println!("Failed {}", e.message)
    }
}