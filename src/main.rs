use axum::{
    extract::{Json, Path, State},
    routing::{get, post, delete},
    Router,
};
use serde::Serialize;
use sqlx::{prelude::FromRow, sqlite::{SqlitePoolOptions}, Pool, Sqlite};
use std::net::SocketAddr;

// Shared app state holding user ids
#[derive(Clone)]
struct AppState {
    db: Pool<Sqlite>
}

#[derive(Serialize, FromRow)]
struct Entity {
    id: i64,
    display_name: String,
    password_hash: String
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let db_uri = format!("sqlite://{}", args.get(1).expect("Provide database file path"));

    let pool = SqlitePoolOptions::new().connect(&db_uri).await.unwrap();
    let state = AppState {db: pool};

    let app = Router::new()
        .route("/api/entity", get(list_users))
        .route("/api/entity/{id}", get(get_user))
        .route("/api/entity/register", post(register_user))
        .route("/api/entity/register", post(login_user))
        .route("/api/entity/{id}", delete(delete_user))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

// GET /api/list
async fn list_users(State(state): State<AppState>) -> Json<Vec<Entity>> {
    let rows = sqlx::query_as!(Entity, "SELECT id, username FROM users")
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

    Json(rows)
}

// POST /api/add with JSON body: { "id": 123 }
// #[derive(serde::Deserialize)]
// struct AddEntity {
//     id: i32,
// }

async fn register_user(
    State(state): State<AppState>,
    // Json(payload): Json<AddEntity>,
    body: String,
) -> &'static str {
    // Find max id (returns NULL if table is empty)
    let next_id = sqlx::query_scalar::<_, Option<i64>>("SELECT MAX(id) + 1 FROM users")
        .fetch_one(&state.db)
        .await
        .unwrap_or(Some(1))
        .unwrap_or(1);

    let res = sqlx::query!(
        "INSERT INTO users (id, username) VALUES (?, ?)",
        next_id,
        body
    )
    .execute(&state.db)
    .await;

    match res {
        Ok(_) => "Entity added",
        Err(_) => "Failed to add user",
    }
}

// DELETE /api/delete/:id
async fn delete_user(State(state): State<AppState>, Path(id): Path<i32>) -> &'static str {
    let res = sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(&state.db)
        .await;

    match res {
        Ok(_) => "Entity deleted",
        Err(_) => "Failed to delete user",
    }
}
