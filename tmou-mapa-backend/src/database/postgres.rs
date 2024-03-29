use diesel::dsl::sql;
use diesel::insert_into;
use diesel::prelude::*;
use rocket_contrib::databases::diesel;

use crate::database::db::{Db, GameEditorDb, MessagesDb};
use crate::models::db::*;
use crate::models::errors::*;
use crate::models::schema::bonuses::dsl as bonuses;
use crate::models::schema::items::dsl as items;
use crate::models::schema::messages::dsl as messages;
use crate::models::schema::messages_teams::dsl as messages_teams;
use crate::models::schema::nodes::dsl as nodes;
use crate::models::schema::nodes_items::dsl as nodes_items;
use crate::models::schema::teams::dsl as teams;
use crate::models::schema::teams_items::dsl as teams_items;
use crate::models::schema::ways_nodes::dsl as ways_nodes;
use crate::models::schema::ways::dsl as ways;

// HOWTO debug query?
// use diesel::debug_query;
// use diesel::pg::Pg;
// let debug = debug_query::<Pg, _>(&query);
// println!("Insert query: {:?}", debug);

pub struct PostgresDb {
    pub conn: crate::PostgresDbConn,
}

impl PostgresDb {
    pub fn new(conn: crate::PostgresDbConn) -> Self {
        PostgresDb { conn: conn }
    }
}

impl Db for PostgresDb {
    fn get_team(&self, id: i32) -> std::option::Option<Team> {
        match teams::teams
            .filter(teams::id.eq(id))
            .limit(1)
            .first::<Team>(&*self.conn)
        {
            Ok(team) => Some(team),
            Err(_) => None,
        }
    }

    fn update_team_position(
        &mut self,
        team: &Team,
        pos: i64,
    ) -> std::result::Result<Team, TmouError> {
        let query = diesel::update(team).set(teams::position.eq(pos));

        match query.get_result::<Team>(&*self.conn) {
            Ok(team) => Ok(team),
            Err(err) => Err(err.into()),
        }
    }

    fn get_reachable_nodes(&self, seed: i64) -> std::result::Result<Pois, TmouError> {
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
        let w2n_level_1: Vec<WaysToNodes> = ways_nodes::ways_nodes
            .filter(ways_nodes::way_id.eq_any(ways_level_1))
            .select((
                ways_nodes::way_id,
                ways_nodes::node_id,
                ways_nodes::node_order,
            ))
            .order_by(ways_nodes::node_order)
            .load(&*self.conn)?;

            
        let ways: Vec<Way> = ways::ways
            .filter(ways::id.eq_any(w2n_level_1.iter().map(|w2n| w2n.way_id)))
            .select((ways::id, ways::tag))
            .load(&*self.conn)?;

        let nodes: Vec<Node> = nodes::nodes
            .filter(nodes::id.eq_any(w2n_level_1.iter().map(|w2n| w2n.node_id)))
            .select((nodes::id, nodes::lat, nodes::lon, nodes::type_, nodes::tag))
            .load(&*self.conn)?;

        Ok(Pois {
            nodes: nodes,
            ways_to_nodes: w2n_level_1,
            ways: ways
        })
    }

    fn get_items_in_node(
        &self,
        node_id: i64,
    ) -> std::result::Result<std::vec::Vec<Item>, TmouError> {
        let items: Vec<Item> = nodes_items::nodes_items
            .filter(nodes_items::node_id.eq(node_id))
            .inner_join(items::items)
            .select((
                items::type_,
                items::url,
                items::level,
                items::name,
                items::description,
                items::condition,
            ))
            .load(&*self.conn)?;
        Ok(items)
    }

    fn get_team_items(&self, team_id: i32) -> std::result::Result<std::vec::Vec<Item>, TmouError> {
        let items = teams_items::teams_items
            .filter(teams_items::team_id.eq(team_id))
            .inner_join(items::items)
            .select((
                items::type_,
                items::url,
                items::level,
                items::name,
                items::description,
                items::condition,
            ))
            .load(&*self.conn)?;
        Ok(items)
    }

    fn get_team_items_with_timestamps(
        &self,
        team_id: i32,
    ) -> std::result::Result<std::vec::Vec<TeamItem>, TmouError> {
        let team_items: Vec<TeamItem> = teams_items::teams_items
            .filter(teams_items::team_id.eq(team_id))
            .inner_join(items::items)
            .select((
                items::type_,
                items::url,
                items::level,
                items::name,
                items::description.nullable(),
                teams_items::timestamp.nullable(),
            ))
            .load(&*self.conn)?;
        Ok(team_items)
    }

    fn get_teams_items(&self) -> std::result::Result<std::vec::Vec<TeamStandingsItem>, TmouError> {
        let items: Vec<TeamStandingsItem> = teams::teams
            .left_join(
                teams_items::teams_items
                    .inner_join(items::items.on(items::name.eq(teams_items::item_name))),
            )
            .select((
                teams::name,
                items::type_.nullable(),
                items::level.nullable(),
                items::name.nullable(),
                items::description.nullable(),
                teams_items::timestamp.nullable(),
            ))
            .load(&*self.conn)?;
        Ok(items)
    }

    fn get_items_teams(&self) -> std::result::Result<std::vec::Vec<ItemTeam>, TmouError> {
        let items: Vec<ItemTeam> = items::items
            .left_join(
                teams_items::teams_items
                    .inner_join(teams::teams.on(teams::id.eq(teams_items::team_id))),
            )
            .select((
                items::name,
                items::type_,
                items::level,
                teams::name.nullable(),
                teams_items::timestamp.nullable(),
            ))
            .load(&*self.conn)?;
        Ok(items)
    }

    fn put_team_items(
        &mut self,
        team_id: i32,
        items: std::vec::Vec<Item>,
    ) -> std::result::Result<(), TmouError> {
        let existing_records: Vec<String> = teams_items::teams_items
            .filter(teams_items::team_id.eq(team_id))
            .select(teams_items::item_name)
            .load(&*self.conn)?;
        let mut its = items.clone();
        //TODO Looks suboptimal - IMO this methoud should accept only items to update, uniq constraint
        //in DB should guard
        its.retain(|i| !existing_records.contains(&i.name));
        match its.len() {
            0 => Ok(()),
            _ => {
                let records: Vec<TeamToItem> = its
                    .iter()
                    .map(|i| TeamToItem {
                        team_id: team_id,
                        item_name: i.name.clone(),
                        timestamp: None,
                    })
                    .collect();
                let query = insert_into(teams_items::teams_items).values(records);
                match query.get_result::<TeamToItem>(&*self.conn) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err.into()),
                }
            }
        }
    }

    fn get_teams_positions(&self) -> std::result::Result<std::vec::Vec<TeamPosition>, TmouError> {
        let teams_positions = teams::teams
            .left_join(teams_items::teams_items)
            .left_join(items::items.on(items::name.eq(teams_items::item_name)))
            .inner_join(nodes::nodes)
            .group_by((teams::name, nodes::lat, nodes::lon))
            .select((teams::name, nodes::lat, nodes::lon, sql("MAX(items.level)")))
            .load(&*self.conn);

        Ok(teams_positions?)
    }

    fn get_badge_labels(&self) -> std::result::Result<Vec<String>, TmouError> {
        let badges = items::items
            .filter(items::type_.eq("badge"))
            .select(items::name)
            .order_by(items::name)
            .load(&*self.conn)?;
        Ok(badges)
    }

    fn get_bonuses(&self) -> std::result::Result<std::vec::Vec<Bonus>, TmouError> {
        let bonuses = bonuses::bonuses
            .filter(bonuses::display_time.lt(diesel::dsl::now))
            .select((
                bonuses::url,
                bonuses::label,
                bonuses::description.nullable(),
                bonuses::display_time,
            ))
            .order_by(bonuses::display_time)
            .load(&*self.conn)?;

        Ok(bonuses)
    }

    fn get_game_state_by_puzzles(&self) -> std::result::Result<std::vec::Vec<i64>, TmouError> {
        let game_state: Vec<Option<i64>> = items::items
            .left_join(teams_items::teams_items.on(items::name.eq(teams_items::item_name)))
            .filter(
                items::type_
                    .eq("puzzles")
                    .or(items::type_.eq("puzzles-fake")),
            )
            .group_by(items::level)
            .select(sql("COUNT (DISTINCT teams_items.team_id)"))
            .order_by(items::level)
            .load(&*self.conn)?;

        Ok(game_state.iter().map(|c| c.unwrap_or(0)).collect())
    }

    fn get_dead_item_for_level(&self, level: i16) -> std::result::Result<Item, TmouError> {
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

impl MessagesDb for PostgresDb {
    fn get_messages(&self, team_id: i32, limit: Option<i64>) -> Option<Vec<Message>> {
        let mut query = messages_teams::messages_teams
            .filter(
                messages_teams::team_id
                    .eq(team_id)
                    .or(messages_teams::team_id.eq(BROADCAST_TEAM_ID)),
            )
            .inner_join(messages::messages)
            .select((
                messages::id,
                messages::content,
                messages::type_,
                messages::timestamp,
            ))
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

    fn put_message(
        &self,
        message: WebMessage,
        teams_ids: Vec<i32>,
    ) -> std::result::Result<(), TmouError> {
        let message_id = insert_into(messages::messages)
            .values(message)
            .returning(messages::id)
            .get_result(&*self.conn)?;
        let messages_teams: Vec<MessageToTeam> = teams_ids
            .into_iter()
            .map(|team_id| MessageToTeam {
                message_id,
                team_id,
            })
            .collect();
        match insert_into(messages_teams::messages_teams)
            .values(messages_teams)
            .execute(&*self.conn)
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
}

pub fn get_team_by_phrase(
    connection: &diesel::PgConnection,
    phr: &String,
    testers_only: bool,
) -> Option<Team> {
    let mut query = teams::teams.into_boxed().filter(teams::phrase.eq(phr));
    if testers_only {
        query = query.filter(teams::is_tester.eq(true));
    }
    match query.limit(1).first::<Team>(connection) {
        Ok(team) => Some(team),
        Err(_) => None,
    }
}

pub fn get_team_by_external_id(
    connection: &diesel::PgConnection,
    id: i32,
    testers_only: bool,
) -> std::option::Option<Team> {
    let mut query = teams::teams.into_boxed().filter(teams::team_id.eq(id));
    if testers_only {
        query = query.filter(teams::is_tester.eq(true));
    }

    match query.limit(1).first::<Team>(connection) {
        Ok(team) => Some(team),
        Err(_) => None,
    }
}

pub fn put_team(
    connection: &diesel::PgConnection,
    team: WebTeam,
) -> std::result::Result<Team, TmouError> {
    let query = insert_into(teams::teams).values((
        teams::team_id.eq(team.team_id),
        teams::name.eq(team.name),
        teams::phrase.eq(team.phrase),
    ));

    match query.get_result::<Team>(connection) {
        Ok(team) => Ok(team),
        Err(err) => Err(err.into()),
    }
}

pub fn get_all_teams(
    connection: &diesel::PgConnection,
) -> std::result::Result<Vec<Team>, TmouError> {
    teams::teams
        .order_by(teams::name)
        .load(connection)
        .or_else(|err| Err(err.into()))
}

impl GameEditorDb for PostgresDb {}
