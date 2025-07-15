mod auth;
mod config;
mod db;
mod docs;
mod middleware;
mod models;
mod routes;
mod utils;

use axum::Router;
use config::Config;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::docs::api_doc::ApiDoc;
use crate::routes::routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let config: Config = Config::init();

    let pool: sqlx::Pool<sqlx::Postgres> = db::init_db(&config.database_url).await;

    let app: Router = Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(routes(pool));

    println!("🚀 Server running on http://{}", config.addr());

    let listener: TcpListener = tokio::net::TcpListener::bind(config.addr()).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
