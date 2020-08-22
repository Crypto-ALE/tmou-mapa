use super::db_models::Team;
use diesel::prelude::*;
use diesel::insert_into;
use diesel::result::Error::NotFound;
use rocket_contrib::databases::diesel;

use super::schema::teams::dsl::*;
use super::db_controller::DbControl;
use super::db_models;
use super::errors;

// HOWTO debug query?
// use diesel::debug_query;
// use diesel::pg::Pg;
// let debug = debug_query::<Pg, _>(&query);
// // println!("Insert query: {:?}", debug);

pub struct PostgresDbControl
{
    pub conn: super::PostgresDbConn,
}

impl PostgresDbControl
{
    pub fn new(conn: super::PostgresDbConn) -> Self
    {
        PostgresDbControl{conn: conn}
    }
}

impl DbControl for PostgresDbControl
{

fn get_team(&self, id_param: i32) -> std::option::Option<db_models::Team> 
{ 
    get_team_by_id(&self.conn, id_param)
}

fn put_team(&mut self, team: db_models::Team) -> std::result::Result<(), errors::TmouError> 
{ 
    insert_team(&self.conn, team.id, team.name, team.phrase);
    Ok(())
}
fn get_pois_for_team(&self, _: &str) -> std::option::Option<std::vec::Vec<db_models::Poi>> { todo!() }
fn put_pois_for_team(&mut self, _: std::vec::Vec<db_models::Poi>) { todo!() }
fn update_team_position(&mut self, team: &db_models::Team, pos: i64) -> std::result::Result<(), errors::TmouError> 
{
    update_team_position(&self.conn, team, pos);
    Ok(())
}
}



// FIXME: team_id_param is weird, but how to do it better? team_id is taken by schema
pub fn get_team_by_id(connection: &diesel::PgConnection, team_id_param: i32) -> Option<Team> {
    match teams.filter(team_id.eq(team_id_param))
        .limit(1)
        .first::<Team>(connection) {
            Ok(team) => Some(team),
            Err(NotFound) => None,
            Err(err) => panic!("Something very bad with DB happened: {}", err),
        }
}

pub fn insert_team(connection: &diesel::PgConnection, team_id_param: i32, name_param: String, phrase_param: String) -> Team {
    let query = insert_into(teams)
        .values((team_id.eq(team_id_param), name.eq(name_param), phrase.eq(phrase_param)));

    match query.get_result(connection) {
            Ok(result) => result,
            Err(err) => panic!("Something very bad with DB happened: {}", err),
        }
}

pub fn update_team_position(connection: &diesel::PgConnection, team: &Team, new_position: i64) -> Team{
    let query = diesel::update(team).set(position.eq(new_position));

    match query.get_result(connection) {
            Ok(result) => result,
            Err(err) => panic!("Something very bad with DB happened: {}", err),
        }
}

pub fn get_team_by_phrase(connection: &diesel::PgConnection, phr:&String) -> Option<Team> {
    match teams.filter(phrase.eq(phr))
        .limit(1)
        .first::<Team>(connection) {
            Ok(team) => Some(team),
            Err(NotFound) => None,
            Err(err) => panic!("Something very bad with DB happened: {}", err),
        }
}
