#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate regex;

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

use api_models::{NodeAction, Items, TeamInfo};
use db::PostgresDbControl;

embed_migrations!("./migrations/");

#[database("postgres")]
pub struct PostgresDbConn(diesel::PgConnection);

#[get("/game")]
fn info_cookie(    
    team: db_models::Team,
    conn: PostgresDbConn
) -> Result<Json<TeamInfo>, Status> {
    info(team, conn)

}

#[get("/game/<secret_phrase>")]
fn info_phrase(    
    secret_phrase: &RawStr, 
    conn: PostgresDbConn
) -> Result<Json<TeamInfo>, Status> {
    match db::get_team_by_phrase(&*conn, &secret_phrase.to_string())
    {
        Some(team) => info(team, conn),
        None => Err(Status::NotFound)
    }
   
}



//#[get("/game/<secret_phrase>")]
fn info(
    team: db_models::Team,
    conn: PostgresDbConn
) -> Result<Json<TeamInfo>, Status> {
    // TODO: more concise way?
    println!("Debug team:{:?}", team);
    let db_ctrl = PostgresDbControl::new(conn);
    match game_controller::get_info(&db_ctrl, team) {
        Ok(info) => Ok(Json(info)),
        Err(_) => Err(Status::NotFound)
    }
}



#[post("/game", data = "<action>")]
fn go_cookie(
    action: Json<NodeAction>,
    team: db_models::Team,
    conn: PostgresDbConn
) -> Result<Json<TeamInfo>, Status> {
    go(action, team, conn)
}

#[post("/game/<secret_phrase>", data = "<action>")]
fn go_phrase(
    secret_phrase: &RawStr,
    action: Json<NodeAction>,
    conn: PostgresDbConn
) -> Result<Json<TeamInfo>, Status> {
    match db::get_team_by_phrase(&*conn, &secret_phrase.to_string())
    {
        Some(team) => go(action, team, conn),
        None => Err(Status::NotFound)
    }
}

//#[post("/game/<secret_phrase>", data = "<action>")]
fn go(
    action: Json<NodeAction>,
    team: db_models::Team,
    conn: PostgresDbConn
) -> Result<Json<TeamInfo>, Status> {
    let mut db_ctrl = PostgresDbControl::new(conn);
    match game_controller::go_to_node(& mut db_ctrl, team, action.nodeId) {
        Ok(info) => Ok(Json(info)),
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/game/discover")]
fn discover_cookie(
    team: db_models::Team,
    conn: PostgresDbConn
) -> Result<Json<Items>, Status> {
    discover(team, conn)
}

#[get("/game/<secret_phrase>/discover")]
fn discover_phrase(
    secret_phrase: &RawStr,
    conn: PostgresDbConn
) -> Result<Json<Items>, Status> {
    match db::get_team_by_phrase(&*conn, &secret_phrase.to_string())
    {
        Some(team) => discover(team, conn),
        None => Err(Status::NotFound)
    }
}



//#[get("/game/<secret_phrase>/discover")]
fn discover(
    team: db_models::Team,
    conn: PostgresDbConn
) -> Result<Json<Items>, Status> {
    println!("Debug team:{:?}", team);
    let db_ctrl = PostgresDbControl::new(conn);
    match game_controller::discover_node(&db_ctrl, team.position) {
        Ok(nc) => Ok(Json(nc)),
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/")]
fn index() -> Template {
    let context = std::collections::HashMap::<String, String>::new();
    Template::render("index", context)
}

#[get("/<secret_phrase>")]
fn team_index(secret_phrase: &RawStr) -> Template {
    let mut context = std::collections::HashMap::<String, String>::new();
    context.insert("secretPhrase".to_string(), secret_phrase.to_string());
    Template::render("index", context)
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
    //game_controller::initialize();
    rocket::ignite()
        .attach(PostgresDbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_migrations))
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from(static_dir))
        .mount(
            "/",
            routes![index, info_cookie, info_phrase, go_cookie, go_phrase, discover_cookie, discover_phrase, team_index],
        )
        .launch();
}

impl<'a, 'r> FromRequest<'a, 'r> for db_models::Team {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<db_models::Team, Self::Error> {
        let conn = request.guard::<PostgresDbConn>()?;
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
                    .or_else(|| Some(db::insert_team(&*conn, team_id, team_name, phrase)))
            })
            .or_forward(())
    }
}

