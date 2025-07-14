use sqlx::{PgPool, postgres::PgPoolOptions};

pub type DbPool = PgPool;

pub async fn init_db(database_url: &str) -> DbPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("An error occurred while connecting to the database.")
}
