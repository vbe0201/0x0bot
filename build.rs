use std::{env, io};

use diesel::*;
use diesel_migrations;
use dotenv::dotenv;

fn main() {
    // In case a `.env` file is present, export variables from it.
    // Otherwise assume the environment already provides them.
    dotenv().ok();

    // Establish connection to database.
    let database_url =
        env::var("DATABASE_URL").expect("Please set DATABASE_URL in the environment!");
    let connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Failed to connect to {}!", database_url));

    // Run migrations.
    let migrations_dir = diesel_migrations::find_migrations_directory().unwrap();
    diesel_migrations::run_pending_migrations_in_directory(
        &connection,
        migrations_dir.as_path(),
        &mut io::sink(),
    )
    .expect("Failed to run pending migrations!");

    // This should be rerun in case the migrations
    // directory structure was changed.
    println!("cargo:rerun-if-changed={}", migrations_dir.display());
}
