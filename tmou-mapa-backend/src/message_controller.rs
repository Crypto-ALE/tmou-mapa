use super::api_models as api;
use super::db_models as db;
use super::db_controller::{MessagesDbControl};
use super::errors::*;
use itertools::*;

////////////////////////////////////////////////////////////////////
/// Interface
////////////////////////////////////////////////////////////////////

pub fn get_messages_for_team(msg_control: & impl MessagesDbControl, team: db::Team, limit: Option<i64>) -> TmouResult<Vec<api::Message>>
{
    match msg_control.get_messages(team.id, limit) {
        Some(messages) => Ok(messages.iter().map_into().collect()),
        None => Ok(Vec::new()),
    }
}

////////////////////////////////////////////////////////////////////
/// Implementation details
////////////////////////////////////////////////////////////////////

impl From<&db::Message> for api::Message
{

    fn from(value: &db::Message) -> Self
    {
        api::Message{
            content: value.content.clone(),
            r#type: value.type_.clone(),
            timestamp: value.timestamp.clone(),
        }
    }
}
