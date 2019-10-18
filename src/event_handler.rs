use log::info;
use serenity::{
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};

pub struct DiscordHandler;

impl EventHandler for DiscordHandler {
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
