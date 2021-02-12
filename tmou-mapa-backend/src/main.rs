#![feature(proc_macro_hygiene, decl_macro)]
#![feature(bool_to_option)]
#![feature(is_sorted)]
#![feature(entry_insert)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

use std::env;

use chrono::{DateTime,FixedOffset};
use http_auth_basic::Credentials;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use log::{info, warn};
use rocket::config::Environment;
use rocket::fairing::AdHoc;
use rocket::http::RawStr;
use rocket::http::{Status, Header};
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome};
use rocket::response::{Responder, Redirect};
use rocket::Request;
use rocket::Rocket;
use rocket::State;
use rocket_contrib::database;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::{Deserialize, Serialize};
use slugify::slugify;

mod models;
mod controllers;
mod database;
mod rate_limiter;

use controllers::*;
use database::postgres::PostgresDbControl;
use models::*;
use rate_limiter::{RateLimiter, check_rate_limit};

embed_migrations!("./migrations/");

#[database("postgres")]
pub struct PostgresDbConn(diesel::PgConnection);

#[get("/game")]
fn info_cookie(
    _started: GameWasStarted,
    team: models::db::Team,
    conn: PostgresDbConn
) -> Result<Json<api::TeamInfo>, Status> {
    info(team, conn)
}

#[get("/game/<secret_phrase>")]
fn info_phrase(
    _admin: Admin,
    _started: GameWasStarted,
    secret_phrase: &RawStr,
    conn: PostgresDbConn
) -> Result<Json<models::api::TeamInfo>, Status> {
    match database::postgres::get_team_by_phrase(&*conn, &secret_phrase.to_string(), get_game_execution_mode() == "Test")
    {
        Some(team) => info(team, conn),
        None => Err(Status::NotFound)
    }
}

fn info(
    team: models::db::Team,
    conn: PostgresDbConn
) -> Result<Json<api::TeamInfo>, Status> {
    let db_ctrl = PostgresDbControl::new(conn);
    match game::get_info(&db_ctrl, team) {
        Ok(info) => Ok(Json(info)),
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/game/skip")]
fn skip_cookie(
    _started: GameWasStarted,
    team: models::db::Team,
    conn: PostgresDbConn
) -> Result<Json<api::Skip>, Status> {
    skip(team, conn)
}

#[get("/game/<secret_phrase>/skip")]
fn skip_phrase(
    _admin: Admin,
    _started: GameWasStarted,
    secret_phrase: &RawStr,
    conn: PostgresDbConn
) -> Result<Json<api::Skip>, Status> {
    match database::postgres::get_team_by_phrase(&*conn, &secret_phrase.to_string(), get_game_execution_mode() == "Test")
    {
        Some(team) => skip(team, conn),
        None => Err(Status::NotFound)
    }
}

fn skip(
    team: models::db::Team,
    conn: PostgresDbConn
) -> Result<Json<api::Skip>, Status> {
    let db_ctrl = PostgresDbControl::new(conn);
    match game::is_skip_allowed(&db_ctrl, &team) {
        Ok(skip) => Ok(Json(skip)),
        Err(e) => {
            warn!("Skip check failed: {}", e.message);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/game/skip", data="<action>")]
fn proceed_skip_cookie(
    _started: GameWasStarted,
    team: models::db::Team,
    action: Json<api::SkipAction>,
    conn: PostgresDbConn
) -> Result<Json<api::SkipResult>, Status> {
    proceed_skip(action, team, conn)
}

#[post("/game/<secret_phrase>/skip", data="<action>")]
fn proceed_skip_phrase(
    _admin: Admin,
    _started: GameWasStarted,
    secret_phrase: &RawStr,
    action: Json<api::SkipAction>,
    conn: PostgresDbConn
) -> Result<Json<api::SkipResult>, Status> {
    match database::postgres::get_team_by_phrase(&*conn, &secret_phrase.to_string(), get_game_execution_mode() == "Test")
    {
        Some(team) => proceed_skip(action, team, conn),
        None => Err(Status::NotFound)
    }
}

fn proceed_skip(
    action: Json<api::SkipAction>,
    team: models::db::Team,
    conn: PostgresDbConn,
) -> Result<Json<api::SkipResult>, Status> {
    match action.verified {
       false => Err(Status::BadRequest),
       true => {
            let mut db_ctrl = PostgresDbControl::new(conn);
            match game::skip_current_puzzle(&mut db_ctrl, team) {
                Ok(skip) => Ok(Json(skip)),
                Err(_) => Err(Status::NotFound)
            }
        }
    }
}


#[get("/messages?<limit>")]
fn messages_cookie(
    _started: GameWasStarted,
    team: models::db::Team,
    conn: PostgresDbConn,
    limit: Option<i64>
) -> Result<Json<Vec<api::Message>>, Status> {
    messages(team, conn, limit)
}

#[get("/messages/<secret_phrase>?<limit>")]
fn messages_phrase(
    _admin: Admin,
    _started: GameWasStarted,
    secret_phrase: &RawStr,
    conn: PostgresDbConn,
    limit: Option<i64>
) -> Result<Json<Vec<api::Message>>, Status> {
    match database::postgres::get_team_by_phrase(&*conn, &secret_phrase.to_string(), get_game_execution_mode() == "Test")
    {
        Some(team) => messages(team, conn, limit),
        None => Err(Status::NotFound)
    }
}

fn messages(
    team: models::db::Team, conn: PostgresDbConn, limit: Option<i64>
) -> Result<Json<Vec<api::Message>>, Status> {
    let db_ctrl = PostgresDbControl::new(conn);
    match controllers::message::get_messages_for_team(&db_ctrl, team, limit) {
        Ok(msgs) => Ok(Json(msgs)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/game", data = "<action>")]
fn go_cookie(
    _started: GameWasStarted,
    action: Json<api::NodeAction>,
    team: models::db::Team,
    conn: PostgresDbConn,
    rate_limiter: State<RateLimiter>
) -> Result<Json<api::TeamInfo>, Status> {
    go(action, team, conn, rate_limiter)
}

#[post("/game/<secret_phrase>", data = "<action>")]
fn go_phrase(
    _admin: Admin,
    _started: GameWasStarted,
    secret_phrase: &RawStr,
    action: Json<api::NodeAction>,
    conn: PostgresDbConn,
    rate_limiter: State<RateLimiter>
) -> Result<Json<api::TeamInfo>, Status> {
    match database::postgres::get_team_by_phrase(&*conn, &secret_phrase.to_string(), get_game_execution_mode() == "Test")
    {
        Some(team) => go(action, team, conn, rate_limiter),
        None => Err(Status::NotFound)
    }
}

//#[post("/game/<secret_phrase>", data = "<action>")]
fn go(
    action: Json<api::NodeAction>,
    team: models::db::Team,
    conn: PostgresDbConn,
    rate_limiter: State<RateLimiter>
) -> Result<Json<api::TeamInfo>, Status> {
    let mut db_ctrl = PostgresDbControl::new(conn);
    match check_rate_limit(rate_limiter.inner(), &team.id) {
        Ok(()) => match game::go_to_node(& mut db_ctrl, team, action.nodeId) {
                    Ok(info) => Ok(Json(info)),
                    Err(_) => Err(Status::NotFound)
                    },
        Err(()) => Err(Status::TooManyRequests)
    }

}

#[get("/game/discover")]
fn discover_cookie(
    _running: GameIsRunning,
    team: models::db::Team,
    conn: PostgresDbConn
) -> Result<Json<api::DiscoveryEvent>, Status> {
    discover(team, conn)
}

#[get("/game/<secret_phrase>/discover")]
fn discover_phrase(
    _admin: Admin,
    _running: GameIsRunning,
    secret_phrase: &RawStr,
    conn: PostgresDbConn
) -> Result<Json<api::DiscoveryEvent>, Status> {
    match database::postgres::get_team_by_phrase(&*conn, &secret_phrase.to_string(), get_game_execution_mode() == "Test")
    {
        Some(team) => discover(team, conn),
        None => Err(Status::NotFound)
    }
}


//#[get("/game/<secret_phrase>/discover")]
fn discover(
    team: models::db::Team,
    conn: PostgresDbConn
) -> Result<Json<api::DiscoveryEvent>, Status> {
    let mut db_ctrl = PostgresDbControl::new(conn);
    match game::discover_node(& mut db_ctrl, team) {
        Ok(nc) => Ok(Json(nc)),
        Err(_) => Err(Status::NotFound),
    }
}


#[post("/game/discover", data = "<puzzle_name>")]
fn discover_post_cookie(
    _running: GameIsRunning,
    team: models::db::Team,
    conn: PostgresDbConn,
    puzzle_name: Json<api::PuzzleName>
) -> Result<Json<Vec<api::Item>>, Status> {
    discover_post(team, conn, puzzle_name)
}

#[post("/game/<secret_phrase>/discover", data = "<puzzle_name>")]
fn discover_post_phrase(
    _admin: Admin,
    _running: GameIsRunning,
    secret_phrase: &RawStr,
    conn: PostgresDbConn,
    puzzle_name: Json<api::PuzzleName>
) -> Result<Json<Vec<api::Item>>, Status> {
    match database::postgres::get_team_by_phrase(&*conn, &secret_phrase.to_string(), get_game_execution_mode() == "Test")
    {
        Some(team) => discover_post(team, conn, puzzle_name),
        None => Err(Status::NotFound)
    }
}


//#[post("/game/<secret_phrase>/discover")]
fn discover_post(
    team: models::db::Team,
    conn: PostgresDbConn,
    puzzle_name: Json<api::PuzzleName>
) -> Result<Json<Vec<api::Item>>, Status> {
    let mut db_ctrl = PostgresDbControl::new(conn);
    match game::discover_post(& mut db_ctrl, team, &puzzle_name.puzzleName) {
        Ok(nc) => Ok(Json(nc)),
        Err(_) => Err(Status::NotFound),
    }
}


#[get("/game/bonuses")]
fn bonuses(
    _started: GameWasStarted,
    conn: PostgresDbConn,
) -> Result<Json<Vec<api::Bonus>>, Status> {
    let db_ctrl = PostgresDbControl::new(conn);
    match game::get_bonuses(&db_ctrl) {
        Ok(bonuses) => Ok(Json(bonuses)),
        Err(_) => Err(Status::InternalServerError),
    }
}

/* ----------
 * ADMIN
 ------------ */


#[get("/admin")]
fn admin(_admin: Admin, conn: PostgresDbConn) -> Template {
    let teams = database::postgres::get_all_teams(&*conn).unwrap_or_default();

    #[derive(Serialize, Deserialize)]
    struct AdminContext {
        broadcast_team_id: i32,
        teams: Vec<models::db::Team>
    }

    Template::render("admin", AdminContext{broadcast_team_id: database::postgres::BROADCAST_TEAM_ID, teams})
}


#[get("/admin/positions")]
fn admin_positions(_admin: Admin, conn: PostgresDbConn) -> Result<Json<Vec<api::TeamPosition>>, Status> {
    let db_ctrl = PostgresDbControl::new(conn);
    match admin::get_teams_positions(&db_ctrl)
    {
        Ok(positions) => Ok(Json(positions)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/messages",  data = "<message>")]
fn admin_send_message(_admin: Admin, conn: PostgresDbConn, message: Json<api::IncomingMessage>) -> Result<Status, Status> {
    let db_msg_control: PostgresDbControl = PostgresDbControl::new(conn);
    let res = match message.recipient_id {
        database::postgres::BROADCAST_TEAM_ID => controllers::message::send_message_to_all_teams(&db_msg_control, message.into_inner().message),
        _ => admin::unwrap_incoming_message(&db_msg_control, message.into_inner())
                .and_then(|(team, message)| {controllers::message::send_message_to_team(&db_msg_control, team, message)
        })
    };

    match res {
        Ok(_) => Ok(Status::Created),
        Err(err) => {warn!("Failed to send message: {}", err.message); Err(err.into())},
    }
}

#[get("/admin/standings")]
fn admin_standings(_admin: Admin, conn: PostgresDbConn) -> Result<Json<api::Standings>, Status> {
    let db_ctrl = PostgresDbControl::new(conn);
    match controllers::admin::get_teams_standings(&db_ctrl)
    {
        Ok(standings) => Ok(Json(standings)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/admin/puzzles-stats")]
fn puzzles_stats(_admin: Admin, conn: PostgresDbConn) -> Result<Json<api::PuzzlesStats>, Status> {
    let db_ctrl = PostgresDbControl::new(conn);
    match controllers::admin::get_puzzles_stats(&db_ctrl)
    {
        Ok(stats) => Ok(Json(stats)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/")]
fn index_cookie(_forced_https: ForcedHttps, team: models::db::Team, started: Option<GameWasStarted>, running: Option<GameIsRunning>) -> Template {
    let mut context = std::collections::HashMap::<String, String>::new();
    context.insert("teamName".to_string(), team.name);
    index(context, started, running)
}

#[get("/", rank=2)]
fn index_redirect() -> Redirect {
    let url: String = env::var("LOGIN_REDIRECT").unwrap_or("https://www.tmou.cz".to_string());
    Redirect::temporary(url)
}

#[get("/<secret_phrase>")]
fn team_index(_admin: Admin, _forced_https: ForcedHttps, started: Option<GameWasStarted>, running: Option<GameIsRunning>, secret_phrase: &RawStr, conn: PostgresDbConn) -> Result<Template, Redirect> {
    let mut context = std::collections::HashMap::<String, String>::new();
    match database::postgres::get_team_by_phrase(&*conn, &secret_phrase.to_string(), get_game_execution_mode() == "Test") {
        Some(team) => {
                context.insert("teamName".to_string(), team.name);
                context.insert("secretPhrase".to_string(), secret_phrase.to_string());
                Ok(index(context, started, running))
        },
        None => {
            let url: String = env::var("LOGIN_REDIRECT").unwrap_or("https://www.tmou.cz".to_string());
            Err(Redirect::temporary(url))
        }
    }
}

fn index(mut context: std::collections::HashMap<String, String>, started: Option<GameWasStarted>, running: Option<GameIsRunning>, ) -> Template {
    context.insert("main_game_url".to_string(), env::var("MAIN_GAME_URL").unwrap_or("https://www.tmou.cz/22/page".to_string()));
    match (started, running) {
        (Some(_), Some(_)) => Template::render("index", context),
        (Some(_), None) => {
            context.insert("gameFinished".to_string(), "1".to_string());
            Template::render("index", context)
        }
        (None, _) => {
            Template::render("not_started", context)
        }
    }

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

fn rocket() -> rocket::Rocket {
    let static_dir = match env::current_dir() {
        Ok(x) => x.join("static"),
        _ => panic!("Cannot access current directory"),
    };

    rocket::ignite()
        .manage(RateLimiter::new())
        .register(catchers![not_auth, force_https])
        .attach(PostgresDbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_migrations))
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from(static_dir))
        .mount(
            "/",
            routes![index_cookie,
                index_redirect,
                team_index,
                info_cookie,
                info_phrase,
                messages_cookie,
                messages_phrase,
                go_cookie,
                go_phrase,
                discover_cookie,
                discover_phrase,
                discover_post_cookie,
                discover_post_phrase,
                bonuses,
                skip_cookie,
                skip_phrase,
                proceed_skip_cookie,
                proceed_skip_phrase,
                admin,
                admin_positions,
                admin_send_message,
                admin_standings,
                puzzles_stats],
        )
}

fn main() {
    rocket().launch();
}


#[derive(Responder)]
struct BasicAuthResponder {
    inner: rocket::response::status::Unauthorized<()>,
    auth: Header<'static>,
}

#[catch(401)]
fn not_auth(_req: &Request) -> BasicAuthResponder {
        BasicAuthResponder {
        inner: rocket::response::status::Unauthorized(Some(())),
        auth: Header {
            name: rocket::http::uncased::Uncased{string: "WWW-Authenticate".into()},
            value: "Basic".into(),
        },
    }
}

#[catch(505)]
fn force_https(request: &Request) -> Redirect {
    Redirect::permanent(format!("https://{}{}", env::var("HOST").unwrap_or("i.tmou.cz".to_string()), request.uri().path()))
}

struct Admin {}

impl <'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Admin, Self::Error> {
        match env::var("BYPASS_AUTH") {
            Ok(_) => rocket::Outcome::Success(Admin{}),
            _ => request.headers().get_one("Authorization")
                .and_then(|auth| Credentials::from_header(auth.to_string()).ok())
                .and_then(|creds| (env::var("ADMIN_USERNAME").ok().eq(&Some(creds.user_id))).then_some(creds.password))
                .and_then(|password| (env::var("ADMIN_PASSWORD").ok().eq(&Some(password))).then_some(true))
                .and_then(|_is_auth| Some(rocket::Outcome::Success(Admin{})))
                .unwrap_or(rocket::Outcome::Failure((rocket::http::Status::Unauthorized, ())))
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for models::db::Team {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<models::db::Team, Self::Error> {
        #[derive(Debug, Serialize, Deserialize)]
        struct TeamWeb {
            tid: i32,
            tna: String
        }
        let conn = request.guard::<PostgresDbConn>()?;
        let testers_only = get_game_execution_mode() == "Test";

        request
            .cookies()
            .get("TMOU_SSO_JWT")
            .and_then(|cookie| {
                let val: Result<String, _> = cookie.value().parse();
                match env::var("JWT_TOKEN") {
                    Ok(secret) => decode::<TeamWeb>(&val.unwrap(), &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS512)).ok(),
                    Err(_) => {warn!("JWT_TOKEN env var not found."); None}
                }
            })
            .and_then(|team_web| {
                let team_id: i32 = team_web.claims.tid;
                let team_name = team_web.claims.tna;
                database::postgres::get_team_by_external_id(&*conn, team_id, testers_only)
                    .or_else(|| {
                        info!("Inserting team {}", &team_name);
                        let new_team = models::db::WebTeam {
                            name: team_name.clone(),
                            phrase: slugify!(&team_name),
                            team_id,
                        };
                        match database::postgres::put_team(&*conn, new_team) {
                            // when GAME MODE is TEST, cookie teams are inserted, but don't have an
                            //access
                            Ok(team) => match !testers_only {
                                true => Some(team),
                                false => None,
                            }
                            Err(err) => {warn!("Failed to insert team with error: {:?}", err); None}
                        }
                    })
            })
            .or_forward(())
    }
}

fn get_game_execution_mode () -> String {
    env::var("TMOU_GAME_EXECUTION_MODE").unwrap_or("Off".to_string())
}

// when TMOU_GAME_EXECUTION_MODE is:
// On: returns true
// Off: returns false
// Auto: checks the time condition provided as a comparison closure comp_fn
// time format: YYYY-MM-DDThh:mm:ss+/-offset, e. g. 2020-10-11-17T00:00+02:00
fn check_game_state<CompFn>(comp_fn: CompFn) -> bool
where CompFn: Fn(DateTime<FixedOffset>,DateTime<FixedOffset>)->bool {
    match get_game_execution_mode().as_ref() {
        "On" => true,
        "Auto" | "Test" =>
        {
            let from = env::var("TMOU_GAME_START")
                        .or_else(|_| panic!("Game mode is set to auto, but TMOU_GAME_START is not set"))
                        .and_then(|s| DateTime::parse_from_rfc3339(s.as_str()))
                        .unwrap_or_else(|_| panic!("Parsing TMOU_GAME_START failed!"));
                let to = env::var("TMOU_GAME_END")
                    .or_else(|_| panic!("Game mode is set to auto, but TMOU_GAME_END is not set"))
                    .and_then(|s| DateTime::parse_from_rfc3339(s.as_str()))
                    .unwrap_or_else(|_| panic!("Parsing TMOU_GAME_END failed!"));


            comp_fn(from, to)
        },
        _ => false // Off or not specified
    }
}


struct GameIsRunning {} // current date time is between TMOU_GAME_START and TMOU_GAME_END

impl <'a, 'r> FromRequest<'a, 'r> for GameIsRunning {
    type Error = ();

    fn from_request(_request: &'a Request<'r>) -> Outcome<GameIsRunning, Self::Error> {
        if check_game_state(datetime_operators::now_is_between)
        {  rocket::Outcome::Success(GameIsRunning{})}
        else
        {  rocket::Outcome::Failure((rocket::http::Status::Forbidden, ())) }
    }
}

struct GameWasStarted {} // current date time is greater than TMOU_GAME_START

impl <'a, 'r> FromRequest<'a, 'r> for GameWasStarted {
    type Error = ();

    fn from_request(_request: &'a Request<'r>) -> Outcome<GameWasStarted, Self::Error> {
        if check_game_state(datetime_operators::now_is_after_start)
        {  rocket::Outcome::Success(GameWasStarted{})}
        else
        {  rocket::Outcome::Failure((rocket::http::Status::Forbidden, ())) }
    }
}

struct ForcedHttps {}

impl <'a, 'r> FromRequest<'a, 'r> for ForcedHttps {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<ForcedHttps, Self::Error> {
        let is_prod = Environment::active().and_then(|env| Ok(env.is_prod())).unwrap_or(false);
        match (is_prod, request.headers().get_one("X-Forwarded-Proto")) {
            (true, Some("http")) => {
                rocket::Outcome::Failure((Status::HttpVersionNotSupported, ()))
            },
            _ => rocket::Outcome::Success(ForcedHttps{})
        }
    }
}





