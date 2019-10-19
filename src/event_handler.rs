use log::info;
use serenity::{
    model::{
        channel::Message,
        event::{MessageUpdateEvent, ResumedEvent},
        gateway::Ready,
        id::{ChannelId, MessageId},
    },
    prelude::*,
};

use crate::db::{MessageState, PgPool};

pub struct DiscordHandler {
    // DiscordHandler holds an immutable reference
    // to a PgPool object because we need to have
    // access to the database in Discord events
    // for logging purposes.
    pub database_pool: PgPool,
}

impl EventHandler for DiscordHandler {
    fn message(&self, _: Context, message: Message) {
        // Save message to database.
        MessageState::create(message, &self.database_pool.get().unwrap());
    }

    fn message_delete(&self, _: Context, _: ChannelId, message_id: MessageId) {
        // Mark corresponding message states as deleted in the database.
        MessageState::mark_as_deleted(message_id, &self.database_pool.get().unwrap());
    }

    fn message_delete_bulk(&self, _: Context, _: ChannelId, message_ids: Vec<MessageId>) {
        // Mark corresponding message states as deleted in the database.
        let connection = self.database_pool.get().unwrap();
        for message_id in message_ids.iter() {
            MessageState::mark_as_deleted(*message_id, &connection);
        }
    }

    fn message_update(
        &self,
        _: Context,
        old_message: Option<Message>,
        new_message: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        // Save message state to database.
        MessageState::create(new_message.unwrap(), &self.database_pool.get().unwrap());
    }

    fn ready(&self, _: Context, ready: Ready) {
        info!(
            "{} with ID {} is connected!",
            ready.user.tag(),
            ready.user.id
        );
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Connection resumed!");
    }
}
