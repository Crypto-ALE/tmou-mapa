#[macro_use]
extern crate diesel;

mod models;
mod game_data_reader;
mod game_data_reader_tests;

use std::env;

use diesel::insert_into;
use diesel::prelude::*;
use regex::Regex;

use models::db;
use models::errors::*;
use models::schema::bonuses::dsl as bonuses;
use models::schema::items::dsl as items;
use models::schema::nodes_items::dsl as nodes_items;

use game_data_reader::{read_game_data};

fn import_items(conn: &mut PgConnection, db_items: Vec<db::Item>, db_nodes_items: Vec<db::NodeToItem>) -> TmouResult<()> {
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

fn import_bonuses(conn: &mut PgConnection, db_bonuses: Vec<db::Bonus>) -> TmouResult<()> {
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
    let game_data = read_game_data(path)?;
    let mut conn = get_db_connection()?;

    import_items(&mut conn, game_data.items, game_data.nodes_items)?;
    import_bonuses(&mut conn, game_data.bonuses)?;

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
