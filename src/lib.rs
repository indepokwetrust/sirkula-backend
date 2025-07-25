use sqlx::{Pool, Sqlite};

pub mod api;
pub mod database;

#[derive(Clone)]
pub struct AppState {
    db: Pool<Sqlite>,
}

impl AppState {
    pub fn new(db: Pool<Sqlite>) -> Self {
        AppState { db }
    }
    pub fn get_pool(&self) -> &Pool<Sqlite> {
        &self.db
    }
}
