#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

use rocket::http::RawStr;
use rocket::{Request, Data};
use rocket::fairing::{Fairing, Info, Kind};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use rocket::http::Status;
use std::env;

mod errors;
mod api_models;
mod db_models;
mod osm_models;
mod db_controller;
mod game_controller;
mod osm_reader;
mod osm_logic;
mod tests;
mod map_contents;

use api_models::{NodeAction, /*Pois, Grid, */NodeContents, TeamInfo};

#[get("/game/<secret_phrase>")]
fn info(secret_phrase: &RawStr) -> Result<Json<TeamInfo>, Status>
{
    // TODO: more concise way?
    match game_controller::get_info(secret_phrase)
    {
        Ok(info) => Ok(Json(info)),
        Err(_) => Err(Status::NotFound)
    }
}

#[post("/game/<secret_phrase>", data="<action>")]
fn go(secret_phrase: &RawStr, action: Json<NodeAction>) -> Result<Json<TeamInfo>,Status>
{
    game_controller::go_to_node(secret_phrase, &action.nodeId)?;
    info(secret_phrase)
}

#[get("/game/<secret_phrase>/discover")]
fn discover(secret_phrase: &RawStr) -> Result<Json<NodeContents>,Status> 
{
    let state = game_controller::get_team_state(secret_phrase)?;
    match game_controller::discover_node(secret_phrase, &state.position)
    {
        Ok(nc) => Ok(Json(nc)),
        Err(_) => Err(Status::NotFound)
    }
}

/*
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

*/

#[allow(unused)]
#[get("/<secret_phrase>")]
fn team_index(secret_phrase: &RawStr) -> Template 
{
    let mut context = std::collections::HashMap::<String,String>::new();
    context.insert("secretPhrase".to_string(), secret_phrase.to_string());
    Template::render("index", context)
}


#[get("/")]
fn index() -> String 
{
    format!("Become the legend!")
}

fn main() 
{
    let static_dir = match env::current_dir() {
        Ok(x) => x.join("static"),
        _ => panic!("Cannot access current directory")
    };

    game_controller::initialize();
    rocket::ignite()
        .mount("/static", StaticFiles::from(static_dir))
        .mount("/", routes![index, info, go, discover, /*pois, grid, */ team_index])
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
