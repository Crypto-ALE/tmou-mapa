use crate::models::db::Team;
use diesel::prelude::*;
use diesel::dsl::{sql};
use diesel::insert_into;
use rocket_contrib::databases::diesel;

use crate::schema::teams::dsl as teams;
use crate::schema::messages::dsl as messages;
use crate::schema::messages_teams::dsl as messages_teams;
use crate::schema::nodes::dsl as nodes;
use crate::schema::ways_nodes::dsl as ways_nodes;
use crate::schema::nodes_items::dsl as nodes_items;
use crate::schema::items::dsl as items;
use crate::schema::bonuses::dsl as bonuses;
use crate::schema::teams_items::dsl as teams_items;
use crate::database::db::{DbControl, MessagesDbControl};
use crate::models::db;
use crate::models::errors;

// HOWTO debug query?
// use diesel::debug_query;
// use diesel::pg::Pg;
// let debug = debug_query::<Pg, _>(&query);
// println!("Insert query: {:?}", debug);

pub struct PostgresDbControl
{
    pub conn: crate::PostgresDbConn
}

impl PostgresDbControl
{
    pub fn new(conn: crate::PostgresDbConn) -> Self
    {
        PostgresDbControl{conn: conn}
    }
}

impl DbControl for PostgresDbControl
{


fn get_team(&self, id: i32) -> std::option::Option<models::db::Team>
{
    match teams::teams.filter(teams::id.eq(id))
        .limit(1)
        .first::<Team>(&*self.conn) {
            Ok(team) => Some(team),
            Err(_) => None
        }
}

fn update_team_position(&mut self, team: &models::db::Team, pos: i64) -> std::result::Result<Team, models::errors::TmouError>
{
    let query = diesel::update(team).set(teams::position.eq(pos));

    match query.get_result::<Team>(&*self.conn) {
            Ok(team) => Ok(team),
            Err(err) => Err(err.into())
    }
}

fn get_reachable_nodes(&self, seed: i64) -> std::result::Result<models::db::Pois, models::errors::TmouError>
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
    let w2n_level_1: Vec<models::db::WaysToNodes> = ways_nodes::ways_nodes
        .filter(ways_nodes::way_id.eq_any(ways_level_1))
        .select((ways_nodes::way_id, ways_nodes::node_id, ways_nodes::node_order))
        .order_by(ways_nodes::node_order)
        .load(&*self.conn)?;

    let nodes: Vec<models::db::Node> = nodes::nodes
        .filter(nodes::id.eq_any(w2n_level_1.iter().map(|w2n| w2n.node_id)))
        .select((nodes::id, nodes::lat, nodes::lon, nodes::type_))
        .load(&*self.conn)?;

    Ok(models::db::Pois{nodes: nodes, ways_to_nodes: w2n_level_1})
}

fn get_items_in_node(&self, node_id: i64) -> std::result::Result<std::vec::Vec<models::db::Item>, models::errors::TmouError>
{
    let items: Vec<models::db::Item> = nodes_items::nodes_items
        .filter(nodes_items::node_id.eq(node_id))
        .inner_join(items::items)
        .select((items::type_, items::url, items::level, items::name, items::description))
        .load(&*self.conn)?;
    Ok(items)
}

fn get_team_items(&self, team_id: i32) -> std::result::Result<std::vec::Vec<models::db::Item>, models::errors::TmouError>
{
    let items = teams_items::teams_items
        .filter(teams_items::team_id.eq(team_id))
        .inner_join(items::items)
        .select((items::type_, items::url, items::level, items::name, items::description))
        .load(&*self.conn)?;
    Ok(items)
}

fn get_team_items_with_timestamps(&self, team_id: i32) -> std::result::Result<std::vec::Vec<models::db::TeamItem>, models::errors::TmouError>
{
    type Tuple = (String,String,i16,String,Option<String>,Option<chrono::NaiveDateTime>);
    let items:Vec<Tuple> = teams_items::teams_items
        .filter(teams_items::team_id.eq(team_id))
        .inner_join(items::items)
        .select((items::type_, items::url, items::level, items::name, items::description, teams_items::timestamp))
        .load(&*self.conn)?;
    let from_tuple = |t: Tuple| models::db::TeamItem {type_:t.0, url:t.1, level:t.2, name:t.3, description:t.4, timestamp:t.5};
    let team_items:Vec<models::db::TeamItem> = items.into_iter().map(from_tuple).collect();
    Ok(team_items)
}



fn get_teams_items(&self) -> std::result::Result<std::vec::Vec<models::db::TeamStandingsItem>, models::errors::TmouError>
{
    let items:Vec<models::db::TeamStandingsItem> = teams::teams
        .left_join(teams_items::teams_items.inner_join(items::items.on(items::name.eq(teams_items::item_name))))
        .select((teams::name,
                 items::type_.nullable(),
                 items::level.nullable(),
                 items::name.nullable(),
                 items::description.nullable(),
                 teams_items::timestamp.nullable()))
        .load(&*self.conn)?;
    Ok(items)
}

fn get_items_teams(&self) -> std::result::Result<std::vec::Vec<models::db::ItemTeam>, models::errors::TmouError>
{
    let items:Vec<models::db::ItemTeam> = items::items
        .left_join(teams_items::teams_items.inner_join(teams::teams.on(teams::id.eq(teams_items::team_id))))
        .select((items::name,
                 items::type_,
                 items::level,
                 teams::name.nullable(),
                 teams_items::timestamp.nullable()))
        .load(&*self.conn)?;
    Ok(items)
}


fn put_team_items(&mut self, team_id: i32, items: std::vec::Vec<models::db::Item>) -> std::result::Result<(), models::errors::TmouError>
{
    let existing_records: Vec<String> = teams_items::teams_items
        .filter(teams_items::team_id.eq(team_id))
        .select(teams_items::item_name)
        .load(&*self.conn)?;
    let mut its = items.clone();
    //TODO Looks suboptimal - IMO this methoud should accept only items to update, uniq constraint
    //in DB should guard
    its.retain(|i| !existing_records.contains(&i.name));
    match its.len()
    {
        0 => Ok(()),
        _ =>
        {
            let records: Vec<models::db::TeamToItem> = its.iter()
            .map(|i| models::db::TeamToItem{team_id: team_id, item_name: i.name.clone(), timestamp: None})
            .collect();
            let query = insert_into(teams_items::teams_items).values(records);
            match query.get_result::<models::db::TeamToItem>(&*self.conn) {
                Ok(_) => Ok(()),
                Err(err) => Err(err.into()),
            }
        }

    }
}

fn get_teams_positions(&self) -> std::result::Result<std::vec::Vec<models::db::TeamPosition>, models::errors::TmouError>
{
    let teams_positions = teams::teams
        .left_join(teams_items::teams_items)
        .left_join(items::items.on(items::name.eq(teams_items::item_name)))
        .inner_join(nodes::nodes)
        .group_by((teams::name, nodes::lat, nodes::lon))
        .select((teams::name, nodes::lat, nodes::lon, sql("MAX(items.level)")))
        .load(&*self.conn);

    Ok(teams_positions?)
}

fn get_badge_labels(&self) -> std::result::Result<Vec<String>, models::errors::TmouError>
{
    let badges = items::items
        .filter(items::type_.eq("badge"))
        .select(items::name)
        .order_by(items::name)
        .load(&*self.conn)?;
    Ok(badges)
}

fn get_bonuses(&self) -> std::result::Result<std::vec::Vec<models::db::Bonus>, models::errors::TmouError> {
    let bonuses = bonuses::bonuses
        .filter(bonuses::display_time.lt(diesel::dsl::now))
        .select((bonuses::url, bonuses::label, bonuses::description.nullable(), bonuses::display_time))
        .order_by(bonuses::display_time)
        .load(&*self.conn)?;

    Ok(bonuses)
}

fn get_game_state_by_puzzles(&self) -> std::result::Result<std::vec::Vec<i64>, models::errors::TmouError> {
   let game_state: Vec<Option<i64>> = items::items
       .left_join(teams_items::teams_items.on(items::name.eq(teams_items::item_name)))
       .filter(items::type_.eq("puzzles").or(items::type_.eq("puzzles-fake")))
       .group_by(items::level)
       .select(sql("COUNT (DISTINCT teams_items.team_id)"))
       .order_by(items::level)
       .load(&*self.conn)?;

    Ok(game_state.iter().map(|c| c.unwrap_or(0)).collect())
}

fn get_dead_item_for_level(&self, level: i16) -> std::result::Result<models::db::Item, models::errors::TmouError> {
    let dead = items::items
        .filter(items::level.eq(level).and(items::type_.eq("dead")))
        .limit(1)
        .first(&*self.conn)?;

    Ok(dead)
}
}
//
// messages for this team id are broadcasted to all the teams
pub const BROADCAST_TEAM_ID: i32 = 0;

impl MessagesDbControl for PostgresDbControl
{
    fn get_messages(&self, team_id: i32, limit: Option<i64>) -> Option<Vec<models::db::Message>> {
        let mut query = messages_teams::messages_teams
            .filter(messages_teams::team_id.eq(team_id).or(messages_teams::team_id.eq(BROADCAST_TEAM_ID)))
            .inner_join(messages::messages)
            .select((messages::id, messages::content, messages::type_, messages::timestamp))
            .order_by(messages::timestamp.desc())
            .into_boxed();

        if let Some(l) = limit {
            query = query.limit(l);
        }

        match query.load(&*self.conn) {
            Ok(messages) => Some(messages),
            Err(diesel::result::Error::NotFound) => None,
            Err(_) => None,
        }
    }

    fn put_message(&self, message: models::db::WebMessage, teams_ids: Vec<i32>) -> std::result::Result<(), models::errors::TmouError> {
        let message_id = insert_into(messages::messages)
            .values(message)
            .returning(messages::id)
            .get_result(&*self.conn)?;
        let messages_teams: Vec<models::db::MessageToTeam> = teams_ids.into_iter()
            .map(|team_id| models::db::MessageToTeam {message_id, team_id}).collect();
        match insert_into(messages_teams::messages_teams).values(messages_teams).execute(&*self.conn) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
}



pub fn get_team_by_phrase(connection: &diesel::PgConnection, phr:&String, testers_only: bool) -> Option<Team> {
    let mut query = teams::teams.into_boxed().filter(teams::phrase.eq(phr));
    if testers_only {
        query = query.filter(teams::is_tester.eq(true));
    }
    match query
        .limit(1)
        .first::<Team>(connection) {
            Ok(team) => Some(team),
            Err(_) => None
        }
}

pub fn get_team_by_external_id(connection: &diesel::PgConnection, id: i32, testers_only: bool) -> std::option::Option<models::db::Team>
{
    let mut query = teams::teams.into_boxed().filter(teams::team_id.eq(id));
    if testers_only {
        query = query.filter(teams::is_tester.eq(true));
    }

    match query
        .limit(1)
        .first::<Team>(connection) {
            Ok(team) => Some(team),
            Err(_) => None
        }
}

pub fn put_team(connection: &diesel::PgConnection, team: models::db::WebTeam) -> std::result::Result<Team, models::errors::TmouError>
{
    let query = insert_into(teams::teams)
        .values((teams::team_id.eq(team.team_id), teams::name.eq(team.name), teams::phrase.eq(team.phrase)));

    match query.get_result::<Team>(connection) {
            Ok(team) => Ok(team),
            Err(err) => Err(err.into())
        }
}

pub fn get_all_teams(connection: &diesel::PgConnection) -> std::result::Result<Vec<Team>, models::errors::TmouError>
{
    teams::teams.order_by(teams::name).load(connection).or_else(|err| Err(err.into()))
}
