#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;
use rocket::{Request, Data};
use rocket::fairing::{Fairing, Info, Kind};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use rocket::http::Status;

mod errors;
mod api_models;
mod db_models;
mod osm_models;
mod db_controller;
mod game_controller;
mod osm_reader;
mod osm_logic;
mod tests;


use api_models::{NodeAction, Pois, Grid, TeamState};




#[get("/game/<secret_phrase>/info")]
fn info(secret_phrase: &RawStr) -> Result<Json<TeamState>, Status>
{
    // TODO: more concise way?
    match game_controller::get_info(secret_phrase)
    {
        Ok(info) => Ok(Json(info)),
        Err(_) => Err(Status::NotFound)
    }
}

#[post("/game/<secret_phrase>/action", data="<action>")]
fn action(secret_phrase: &RawStr, action: Json<NodeAction>) -> Status 
{
    match action.action.as_str()
    {
        "go" => 
          match game_controller::go_to_node(secret_phrase, &action.nodeId)
          {
              Ok(_) => Status::Ok,
              Err(r) => rocket::http::Status::from(r)
          }, // check movement! error handling!
        "discover"  | "requestChat"  | "requestVideo" => Status::Ok,
        _ => Status::NotFound
    }
}


#[get("/game/<secret_phrase>/pois")]
fn pois(secret_phrase: &RawStr) -> Result<Json<Pois>, Status>
{
    match game_controller::get_pois(secret_phrase)
    {
        Ok(pois) => Ok(Json(pois)),
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/game/<secret_phrase>/grid")]
fn grid(secret_phrase: &RawStr) -> Result<Json<Grid>, Status> 
{
    match game_controller::get_grid(secret_phrase)
    {
        Ok(grid) => Ok(Json(grid)),
        Err(_) => Err(Status::NotFound)
    }
}


#[get("/game/<secret_phrase>")]
fn team_index(secret_phrase: &RawStr) -> Template 
{
    let context = std::collections::HashMap::<String,String>::new();
    Template::render("index", context)
}


#[get("/")]
fn index() -> String 
{
    format!("Become the legend!")
}

fn main() 
{
    rocket::ignite()
        .mount("/static", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .mount("/", routes![index, info, action, pois, grid, team_index])
        .attach(PhraseChecker)
        .attach(Template::fairing())
        .launch();
}

struct PhraseChecker;

impl Fairing for PhraseChecker 
{
    fn  info(&self) -> Info {
        Info {
            name: "Phrase Checker",
            kind: Kind::Request
        }
    }

    //TODO: Add response and return 404 for non-matching phrase
    fn on_request(&self, request: &mut Request, _: &Data) {
        println!("Fairing called");
        println!("Tajna fraze je: {:?}", request.uri().segments().nth(1));
    }
}
