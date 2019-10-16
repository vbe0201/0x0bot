use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("Bot {} with ID {} is connected!", ready.user.tag(), ready.user.id);
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Please set the DISCORD_TOKEN environment variable before running the bot.");

    let mut client = Client::new(&token, Handler)
        .expect("An error occurred during client creation.");

    if let Err(why) = client.start() {
        println!("Failed to connect: {:?}", why);
    }
}
