use itertools::*;

use crate::database::db::{MessagesDbControl};
use crate::models::api as api;
use crate::models::db as db;
use crate::models::errors::*;

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

pub fn send_message_to_team(msg_control: & impl MessagesDbControl, team: db::Team, message: api::Message) -> TmouResult<()> {
    msg_control.put_message(message.into(), vec![team.id])
}

pub fn send_message_to_all_teams(msg_control: & impl MessagesDbControl, message: api::Message) -> TmouResult<()> {
    msg_control.put_message(message.into(), vec![0])
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

impl From<api::Message> for db::WebMessage
{

    fn from(value: api::Message) -> Self
    {
        db::WebMessage{
            content: value.content.clone(),
            type_: value.r#type.clone(),
            timestamp: value.timestamp,
        }
    }
}
