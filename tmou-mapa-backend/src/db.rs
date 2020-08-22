use super::db_models::Team;
use diesel::prelude::*;
use diesel::insert_into;
use diesel::result::Error::NotFound;
use rocket_contrib::databases::diesel;

use super::schema::teams::dsl::*;

// HOWTO debug query?
// use diesel::debug_query;
// use diesel::pg::Pg;
// let debug = debug_query::<Pg, _>(&query);
// // println!("Insert query: {:?}", debug);

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

pub fn insert_team(connection: &diesel::PgConnection, team_id_param: i32, name_param: String, phrase_param: String) -> Option<Team> {
    let query = insert_into(teams)
        .values((team_id.eq(team_id_param), name.eq(name_param), phrase.eq(phrase_param)));

    match query.get_result(connection) {
            Ok(result) => Some(result),
            Err(err) => panic!("Something very bad with DB happened: {}", err),
        }
}

pub fn update_team_position(connection: &diesel::PgConnection, team: &Team, new_position: i64) -> Option<Team>{
    let query = diesel::update(team).set(position.eq(new_position));

    match query.get_result(connection) {
            Ok(result) => Some(result),
            Err(err) => panic!("Something very bad with DB happened: {}", err),
        }
}

pub fn get_team_by_phrase(connection: &diesel::PgConnection) -> Option<Team> {
    match teams.filter(phrase.eq("ahoj".to_string()))
        .limit(1)
        .first::<Team>(connection) {
            Ok(team) => Some(team),
            Err(NotFound) => None,
            Err(err) => panic!("Something very bad with DB happened: {}", err),
        }
}
