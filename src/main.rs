extern crate chrono;

#[macro_use]
extern crate diesel;

extern crate log;

#[macro_use]
extern crate serenity;

use std::env;

use serenity::prelude::Client;

use self::event_handler::DiscordHandler;
use diesel::{Connection, PgConnection};

mod event_handler;

#[inline]
fn create_database_url() -> String {
    let user = env::var("POSTGRES_USER")
        .expect("Please set the POSTGRES_USER environment variable before running the bot.");
    let password = env::var("POSTGRES_PASSWORD")
        .expect("Please set the POSTGRES_PASSWORD environment variable before running the bot.");
    let host = env::var("POSTGRES_HOST")
        .expect("Please set the POSTGRES_HOST environment variable before running the bot.");
    let database = env::var("POSTGRES_DATABASE")
        .expect("Please set the POSTGRES_DATABASE environment variable before running the bot.");

    format!("postgresql://{}:{}@{}/{}", user, password, host, database)
}

fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Please set the DISCORD_TOKEN environment variable before running the bot.");

    // Currently unused.
    let postgres_connection = PgConnection::establish(&create_database_url())
        .expect("Failed to establish a connection to the database!");

    let mut client =
        Client::new(&token, DiscordHandler).expect("An error occurred during client creation.");

    if let Err(why) = client.start() {
        println!("Failed to connect: {:?}", why);
    }
}
