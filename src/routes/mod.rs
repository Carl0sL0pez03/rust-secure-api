use axum::{Router, routing::get};

use crate::db::DbPool;

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state(pool)
}

async fn health_check() -> &'static str {
    "API corriendo 🚀"
}
