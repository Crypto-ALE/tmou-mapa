#[macro_use] extern crate diesel;

mod db_models;
mod errors;
mod osm_models;
mod osm_reader;
mod schema;

use diesel::prelude::*;
use diesel::insert_into;
use std::env;
use errors::*;
use schema::nodes::dsl as nodes;
use schema::ways_nodes::dsl as ways_nodes;

use osm_reader::*;
use db_models as db;
use regex::Regex;
use chrono::Utc;

fn import_osm(path: &String) -> TmouResult<()>
{
    print!("{}: ", Utc::now());
    println!("Reading OSM data from {}", path);
    let osm = read_osm_from_file(path)?;
    let db_nodes: Vec<db::Node> = osm.nodes.into_iter().map(|(_,n)| 
        db::Node{
            id: n.id, 
            lat: n.lat, 
            lon: n.lon, 
            type_: n.r#type}).collect();
    print!("{}: ", Utc::now());
    println!("Reading database url");
    let dbs_str = env::var("ROCKET_DATABASES")?;
    print!("{}: ", Utc::now());
    println!("Deserializing {}", dbs_str);
    let re = Regex::new(r#".*"([^"]*)".*"#).unwrap();
    let url = re.captures_iter(&dbs_str).nth(0).unwrap()[1].to_string();
    print!("{}: ", Utc::now());
    println!("Connecting to db (url={})", url);
    let conn = PgConnection::establish(&url).unwrap();

    print!("{}: ", Utc::now());
    println!("Inserting {} nodes into db", db_nodes.len());
    let chunk_size = 6000; // postgress allows max(i32)
    for chunk in db_nodes.chunks(chunk_size)
    {
        print!("{}: ", Utc::now());
        println!("Inserting batch...");
        match insert_into(nodes::nodes).values(chunk).execute(&conn)
        {
            Err(e) => println!("Failed with {}; continuing...", e.to_string()),
            _ => ()
        }
    }

    let mut ways2nodes: Vec<db_models::WaysToNodes> = Vec::new();
    for (_,w) in osm.ways
    {
        for (i, n) in w.nodes.into_iter().enumerate()
        {
            ways2nodes.push(db_models::WaysToNodes{way_id: w.id, node_id:n, node_order: i as i16});
        }
    }

    ways2nodes.sort();
    ways2nodes.dedup();
    print!("{}: ", Utc::now());
    println!("Inserting {} ways2nodes into db", ways2nodes.len());
    for chunk in ways2nodes.chunks(chunk_size)
    {
        print!("{}: ", Utc::now());
        println!("Inserting batch...");
        match insert_into(ways_nodes::ways_nodes).values(chunk).execute(&conn)
        {
            Err(e) => println!("Failed with {}; continuing...", e.to_string()),
            _ => ()
        }
        
    }

    Ok(())
}

fn usage() -> TmouResult<()>
{
    println!("Usage:");
    println!("import-osm-data <osm_data_file_path>");
    Ok(())
}



fn main() {
    println!("OSM Data Importer");
    let args: Vec<String> = env::args().collect();
    let res = match args.len()
    {
        2 => import_osm(&args[1]),
        _ => usage()
    };
    print!("{}: ", Utc::now());
    match res
    {
        Ok(_) => println!("Finished successfully"),
        Err(e) => println!("Failed {}", e.message)
    }
}