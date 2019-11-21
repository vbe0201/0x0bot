use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use serenity::model::{channel::Message, id::MessageId};

use crate::db::{
    pool::PgPool,
    schema::message_states,
};

#[derive(AsChangeset, Queryable, Insertable)]
#[table_name = "message_states"]
pub struct MessageState {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub channel: i64,
    pub author: i64,
    pub content: Option<String>,
    pub attachments: Option<Vec<Vec<u8>>>,
    pub deleted: bool,
}

impl From<Message> for MessageState {
    fn from(message: Message) -> Self {
        MessageState {
            id: message.id.0 as i64,
            created_at: message.timestamp.naive_utc(),
            channel: message.channel_id.0 as i64,
            author: message.author.id.0 as i64,
            content: Option::from(message.content.clone()),
            attachments: Option::from(
                message
                    .attachments
                    .iter()
                    .map(|attachment| attachment.download().unwrap())
                    .collect::<Vec<Vec<u8>>>(),
            ),
            deleted: false,
        }
    }
}

impl MessageState {
    pub fn create(message: Message, connection: &PgPool) -> MessageState {
        let message_state = MessageState::from(message);

        diesel::insert_into(message_states::table)
            .values(&message_state)
            .execute(&connection.get().unwrap())
            .expect("Failed to create new message state!");

        message_state
    }

    pub fn get_original(message_id: MessageId, connection: &PgPool) -> MessageState {
        message_states::table
            .filter(message_states::id.eq(message_id.0 as i64))
            .order(message_states::created_at)
            .first(&connection.get().unwrap())
            .expect("Failed to retrieve the original message!")
    }

    pub fn mark_as_deleted(message_id: MessageId, connection: &PgPool) -> usize {
        diesel::update(message_states::table)
            .filter(message_states::id.eq(message_id.0 as i64))
            .set(message_states::deleted.eq(true))
            .execute(&connection.get().unwrap())
            .expect("Failed to update corresponding columns!")
    }
}
