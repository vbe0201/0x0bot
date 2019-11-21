use std::sync::{Arc, Mutex};

use typemap::Key;

pub use self::models::message_state::MessageState;
pub use self::pool::*;

mod models;
mod pool;
mod schema;

pub struct DatabaseConnection;

impl Key for DatabaseConnection {
    type Value = Arc<Mutex<PgPool>>;
}
