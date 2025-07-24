use sqlx::{
    Pool,
    sqlite::{Sqlite, SqliteConnectOptions, SqlitePoolOptions},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args.get(1).expect("Give database filename");

    let pool = force_sync(
        SqlitePoolOptions::new().connect_with(
            SqliteConnectOptions::new()
                .filename(filename)
                .create_if_missing(true),
        ),
    )
    .unwrap();

    let my_pool = MyPool { pool };
    my_pool.query(
        r#"
CREATE TABLE IF NOT EXISTS entity (
    id INT NOT NULL PRIMARY KEY,
    display_name VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(100) NOT NULL,
    entity_type VARCHAR(40) NOT NULL,
    contact_number VARCHAR(40) NOT NULL UNIQUE,
    email VARCHAR(400) NOT NULL UNIQUE,
    website_url VARCHAR(400),
    created_on INT NOT NULL,
    deleted_on INT
)
    "#,
    );

    my_pool.query(
        r#"
CREATE TABLE IF NOT EXISTS donation_post (
    id INT NOT NULL PRIMARY KEY,
    donator_entity_id INT NOT NULL,
    category VARCHAR(40) NOT NULL,
    pickup_location_latitude INT NOT NULL,
    pickup_location_longitude INT NOT NULL,
    pickup_city VARCHAR(40) NOT NULL,
    item_condition VARCHAR(40) NOT NULL,
    amount INT NOT NULL,
    measurement_unit VARCHAR(40) NOT NULL,
    description VARCHAR(400),
    created_on INT NOT NULL,
    deleted_on INT,
    completed_on INT
)
"#,
    );

    my_pool.query(
        r#"
CREATE TABLE IF NOT EXISTS request (
    id INT NOT NULL PRIMARY KEY,
    requester_entity_id INT NOT NULL,
    donation_post_id INT NOT NULL,
    intent VARCHAR(400),
    created_on INT NOT NULL,
    deleted_on INT,
    accepted_on INT,
    completed_on INT
)
"#,
    );

    my_pool.query(
        r#"
CREATE TABLE IF NOT EXISTS profile_image (
    id INT NOT NULL PRIMARY KEY,
    entity_id INT NOT NULL,
    image_url VARCHAR(100) NOT NULL
)
"#,
    );

    my_pool.query(
        r#"
CREATE TABLE IF NOT EXISTS donation_post_image (
    id INT NOT NULL PRIMARY KEY,
    donation_post_id INT NOT NULL,
    image_url VARCHAR(100) NOT NULL
)
"#,
    );

    println!("Database initialized.");
}

// There's probably a better idea, but eh, whatever, we gain nothing from async here anyways
fn force_sync<F: std::future::Future>(fut: F) -> F::Output {
    tokio::runtime::Runtime::new().unwrap().block_on(fut)
}

struct MyPool {
    pool: Pool<Sqlite>,
}

impl MyPool {
    fn query(&self, statement: &str) {
        force_sync(sqlx::query(statement).execute(&self.pool)).unwrap();
    }
}
