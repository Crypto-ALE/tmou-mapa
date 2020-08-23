use super::db_models::Team;
use diesel::prelude::*;
use diesel::insert_into;
use diesel::result::Error::NotFound;
use rocket_contrib::databases::diesel;

use super::schema::teams::dsl as teams;
use super::schema::nodes::dsl as nodes;
use super::schema::ways_nodes::dsl as ways_nodes;
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

fn get_team(&self, id: i32) -> std::option::Option<db_models::Team> 
{ 
    match teams::teams.filter(teams::team_id.eq(id))
        .limit(1)
        .first::<Team>(&*self.conn) {
            Ok(team) => Some(team),
            Err(NotFound) => None,
            Err(err) => panic!("Something very bad with DB happened: {}", err),
        }
}

fn put_team(&mut self, team: db_models::Team) -> std::result::Result<Team, errors::TmouError> 
{ 
    let query = insert_into(teams::teams)
        .values((teams::team_id.eq(team.id), teams::name.eq(team.name), teams::phrase.eq(team.phrase)));

    match query.get_result::<Team>(&*self.conn) {
            Ok(team) => Ok(team),
            Err(err) => panic!("Something very bad with DB happened: {}", err),
        }
}

fn update_team_position(&mut self, team: &db_models::Team, pos: i64) -> std::result::Result<(), errors::TmouError> 
{
    let query = diesel::update(team).set(teams::position.eq(pos));

    match query.get_result::<Team>(&*self.conn) {
            Ok(_) => Ok(()),
            Err(err) => panic!("Something very bad with DB happened: {}", err),
        }
}

fn get_reachable_nodes(&self, seed: i64) -> std::result::Result<db_models::Pois, errors::TmouError> 
{ 
    let ways_level_0: Vec<i64> = ways_nodes::ways_nodes
        .filter(ways_nodes::node_id.eq(seed))
        .select(ways_nodes::way_id)
        .load(&*self.conn)?;
    let nodes_level_0: Vec<i64> = ways_nodes::ways_nodes
        .filter(ways_nodes::way_id.eq_any(ways_level_0))
        .select(ways_nodes::node_id)
        .load(&*self.conn)?;
    let ways_level_1: Vec<i64> = ways_nodes::ways_nodes
        .filter(ways_nodes::node_id.eq_any(nodes_level_0))
        .select(ways_nodes::way_id)
        .load(&*self.conn)?;
    let w2n_level_1: Vec<db_models::WaysToNodes> = ways_nodes::ways_nodes
        .filter(ways_nodes::way_id.eq_any(ways_level_1))
        .select((ways_nodes::way_id, ways_nodes::node_id, ways_nodes::node_order))
        .order_by(ways_nodes::node_order)
        .load(&*self.conn)?;
    
    let nodes: Vec<db_models::Node> = nodes::nodes
        .filter(nodes::id.eq_any(w2n_level_1.iter().map(|w2n| w2n.node_id)))
        .select((nodes::id, nodes::lat, nodes::lon, nodes::type_))
        .load(&*self.conn)?;

    Ok(db_models::Pois{nodes: nodes, ways_to_nodes: w2n_level_1})
}

}

pub fn get_team_by_phrase(connection: &diesel::PgConnection, phr:&String) -> Option<Team> {
    match teams::teams.filter(teams::phrase.eq(phr))
        .limit(1)
        .first::<Team>(connection) {
            Ok(team) => Some(team),
            Err(NotFound) => None,
            Err(err) => panic!("Something very bad with DB happened: {}", err),
        }
}
