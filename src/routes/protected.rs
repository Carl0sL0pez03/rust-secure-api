use crate::auth::handlers::AuthUser;
use axum::{Json, response::IntoResponse};
use utoipa::path as openapi_path;

#[openapi_path(
    get,
    path = "/user/me",
    responses(
        (status = 200, description = "Authenticated user"),
        (status = 401, description = "Invalid or missing token")
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "User"
)]
pub async fn me(user: AuthUser) -> impl IntoResponse {
    Json(serde_json::json!({
        "message": "Authenticated user",
        "user_id": user.user_id
    }))
}
