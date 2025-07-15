pub mod auth_routes;
pub mod protected;

use std::time::Duration;

use crate::{
    db::DbPool,
    middleware::{rate_limit::RateLimitLayer, user_rate_limiter::UserRateLimiterLayer},
};
use auth_routes::{login, register};
use axum::{
    Router,
    routing::{get, post},
};
use protected::me;

pub fn routes(pool: DbPool) -> Router {
    let rate_limit_layer: RateLimitLayer = RateLimitLayer::new(Duration::from_secs(5));
    let user_rl_layer: UserRateLimiterLayer = UserRateLimiterLayer::new(Duration::from_secs(3));

    let auth_routes: Router<sqlx::Pool<sqlx::Postgres>> = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .layer(rate_limit_layer);

    let protected_routes: Router<sqlx::Pool<sqlx::Postgres>> =
        Router::new().route("/me", get(me)).layer(user_rl_layer);

    Router::new()
        .nest("/auth", auth_routes)
        .nest("/user", protected_routes)
        .with_state(pool)
}
