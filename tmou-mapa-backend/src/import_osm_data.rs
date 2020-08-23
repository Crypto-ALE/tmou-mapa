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

fn import_osm(path: &String) -> TmouResult<()>
{
    println!("Reading OSM data from {}", path);
    let osm = read_osm_from_file(path)?;
    let db_nodes: Vec<db::Node> = osm.nodes.into_iter().map(|(_,n)| 
        db::Node{
            id: n.id, 
            lat: n.lat, 
            lon: n.lon, 
            type_: n.r#type}).collect();
    println!("Reading database url");
    let dbs_str = env::var("ROCKET_DATABASES")?;
    println!("Deserializing {}", dbs_str);
    let re = Regex::new(r#".*"([^"]*)".*"#).unwrap();
    let url = re.captures_iter(&dbs_str).nth(0).unwrap()[1].to_string();
    println!("Connecting to db (url={})", url);
    let conn = PgConnection::establish(&url).unwrap();

    println!("Inserting {} nodes into db", db_nodes.len());
    match insert_into(nodes::nodes).values(db_nodes).execute(&conn)
    {
        Err(e) => println!("Failed with {}; continuing...", e.to_string()),
        _ => ()
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
    println!("Inserting {} ways2nodes into db", ways2nodes.len());
    insert_into(ways_nodes::ways_nodes).values(ways2nodes).execute(&conn)?;

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
    match res
    {
        Ok(_) => println!("Finished successfully"),
        Err(e) => println!("Failed {}", e.message)
    }
}