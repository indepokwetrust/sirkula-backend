use axum::Router;
use serde::Serialize;
use sqlx::{Pool, Sqlite, prelude::FromRow, sqlite::SqlitePoolOptions};
use std::net::SocketAddr;

#[derive(Clone)]
struct AppState {
    db: Pool<Sqlite>,
}

#[derive(Serialize, FromRow)]
struct Entity {
    id: i64,
    display_name: String,
    password_hash: String,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let db_uri = format!(
        "sqlite://{}",
        args.get(1).expect("Provide database file path")
    );

    let pool = SqlitePoolOptions::new().connect(&db_uri).await.unwrap();
    let state = AppState { db: pool };

    let app = Router::new().with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
