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

use crate::db::{DatabaseConnection, MessageState};

pub struct DiscordHandler;

impl EventHandler for DiscordHandler {
    fn message(&self, ctx: Context, message: Message) {
        // Acquire database pool from shared context data.
        let data = ctx.data.write();
        let pool = data.get::<DatabaseConnection>().unwrap();

        // Save message to database.
        MessageState::create(message, &pool.lock().unwrap());
    }

    fn message_delete(&self, ctx: Context, _: ChannelId, message_id: MessageId) {
        // Acquire database pool from shared context data.
        let data = ctx.data.write();
        let pool = data.get::<DatabaseConnection>().unwrap();

        // Mark corresponding message states as deleted in the database.
        MessageState::mark_as_deleted(message_id, &pool.lock().unwrap());
    }

    fn message_delete_bulk(&self, ctx: Context, _: ChannelId, message_ids: Vec<MessageId>) {
        // Acquire database pool from shared context data.
        let data = ctx.data.write();
        let pool = data.get::<DatabaseConnection>().unwrap();

        // Mark corresponding message states as deleted in the database.
        for message_id in message_ids.iter() {
            MessageState::mark_as_deleted(*message_id, &pool.lock().unwrap());
        }
    }

    fn message_update(
        &self,
        ctx: Context,
        _: Option<Message>,
        new_message: Option<Message>,
        _: MessageUpdateEvent,
    ) {
        // Acquire database pool from shared context data.
        let data = ctx.data.write();
        let pool = data.get::<DatabaseConnection>().unwrap();

        // Save message state to database.
        MessageState::create(new_message.unwrap(), &pool.lock().unwrap());
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
