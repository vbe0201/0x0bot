extern crate chrono;

#[macro_use]
extern crate diesel;

extern crate dotenv;

extern crate log;

#[macro_use]
extern crate serenity;

use std::env;

use serenity::prelude::Client;

use self::event_handler::DiscordHandler;

mod commands;
mod db;
mod event_handler;
mod utils;

fn main() {
    // In case a `.env` file is present, export variables from it.
    // Otherwise assume the environment already provides them.
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Please set the DISCORD_TOKEN environment variable before running the bot.");
    let database_url = env::var("DATABASE_URL")
        .expect("Please set the DATABASE_URL environment variable before running the bot.");

    // Currently unused.
    let database_pool = db::create_pool(database_url.as_str());

    let mut client = Client::new(&token, DiscordHandler { database_pool })
        .expect("An error occurred during client creation.");

    if let Err(why) = client.start() {
        println!("Failed to connect: {:?}", why);
    }
}
