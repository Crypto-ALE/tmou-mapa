#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

use rocket::fairing::AdHoc;
use rocket::http::RawStr;
use rocket::http::{Status};
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket::Rocket;
use rocket_contrib::database;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::env;

mod api_models;
mod db;
mod db_controller;
mod db_models;
mod errors;
mod game_controller;
mod map_contents;
mod osm_logic;
mod osm_models;
mod osm_reader;
mod tests;
mod schema;

use api_models::{NodeAction, /*Pois, Grid, */ NodeContents, TeamInfo};

embed_migrations!("./migrations/");

#[database("postgres")]
pub struct PostgresDbConn(diesel::PgConnection);

#[get("/game/<secret_phrase>")]
fn info(secret_phrase: &RawStr, team: db_models::Team) -> Result<Json<TeamInfo>, Status> {
    // TODO: more concise way?
    println!("Debug team:{:?}", team);
    match game_controller::get_info(secret_phrase) {
        Ok(info) => Ok(Json(info)),
        Err(_) => Err(Status::NotFound),
    }
}

#[post("/game/<secret_phrase>", data = "<action>")]
fn go(
    secret_phrase: &RawStr,
    action: Json<NodeAction>,
    team: db_models::Team,
) -> Result<Json<TeamInfo>, Status> {
    game_controller::go_to_node(secret_phrase, action.nodeId)?;
    info(secret_phrase, team)
}

#[get("/game/<secret_phrase>/discover")]
fn discover(secret_phrase: &RawStr) -> Result<Json<NodeContents>, Status> {
    let state = game_controller::get_team_state(secret_phrase)?;
    match game_controller::discover_node(secret_phrase, state.position) {
        Ok(nc) => Ok(Json(nc)),
        Err(_) => Err(Status::NotFound),
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
fn team_index(secret_phrase: &RawStr) -> Template {
    let mut context = std::collections::HashMap::<String, String>::new();
    context.insert("secretPhrase".to_string(), secret_phrase.to_string());
    Template::render("index", context)
}

#[get("/")]
fn index() -> String {
    format!("Become the legend!")
}

fn run_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = PostgresDbConn::get_one(&rocket).expect("database connection");
    match embedded_migrations::run_with_output(&*conn, &mut std::io::stdout()) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            panic!("Failed to run database migrations: {:?}", e);
        }
    }
}

fn main() {
    let static_dir = match env::current_dir() {
        Ok(x) => x.join("static"),
        _ => panic!("Cannot access current directory"),
    };
    game_controller::initialize();
    rocket::ignite()
        .attach(PostgresDbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_migrations))
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from(static_dir))
        .mount(
            "/",
            routes![index, info, go, discover, /*pois, grid, */ team_index],
        )
        .launch();
}

impl<'a, 'r> FromRequest<'a, 'r> for db_models::Team {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<db_models::Team, Self::Error> {
        let conn = request.guard::<PostgresDbConn>()?;
        // FLOW:
        // Is there a cookie?
        //  No - go to phrase
        //  Yes - is it valid?
        //    No - go to phrase
        //    Yes - extract id and name, search team by id
        //    Team found - return it
        //    Team not found - first-time access, create a new team
        request
            .cookies()
            .get("TMOU_SSO_JWT")
            .and_then(|cookie| {
                println!("Some cookie");
                let val: Result<String, _> = cookie.value().parse();
                println!("Debug: {:?}", val);
                // TODO: Extract data from JWT
                let team_id: i32 = 1;
                let team_name = "Maštěné Ředkvičky".to_string();
                let phrase = "MegaTajnáFráze".to_string();
                db::get_team_by_id(&*conn, team_id)
                    .or_else(|| db::insert_team(&*conn, team_id, team_name, phrase))
            })
            .or_else(|| {
                println!("No cookie, trying phrase");
                db::get_team_by_phrase(&*conn)
            })
            .or_forward(())
    }
}
