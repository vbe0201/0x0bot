extern crate chrono;

#[macro_use]
extern crate diesel;

extern crate dotenv;

extern crate log;

#[macro_use]
extern crate serenity;

use std::env;

use serenity::prelude::Client;

use db::create_pool;
use event_handler::DiscordHandler;

mod commands;
mod db;
mod event_handler;
mod utils;

fn main() {
    // In case a `.env` file is present, export variables from it.
    // Otherwise assume the environment already provides them.
    dotenv::dotenv().ok();

    // Acquire the previously exported variables to run the bot.
    let token = env::var("DISCORD_TOKEN")
        .expect("Please set the DISCORD_TOKEN environment variable before running the bot.");
    let database_url = env::var("DATABASE_URL")
        .expect("Please set the DATABASE_URL environment variable before running the bot.");

    // Establish the database connection.
    let database_pool = create_pool(&database_url);

    // Create the Discord bot instance.
    let mut client = Client::new(&token, DiscordHandler { database_pool })
        .expect("An error occurred during client creation.");

    // Run the bot or print the connection errors, if any.
    if let Err(reason) = client.start() {
        println!("Failed to connect: {:?}", reason);
    }
}
