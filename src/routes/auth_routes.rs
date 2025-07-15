use crate::auth::{jwt, password};
use crate::db::DbPool;
use crate::models::user::{LoginPayload, RegisterPayload, User};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use utoipa::path as openapi_path;

#[openapi_path(
    post,
    path = "/auth/register",
    request_body = RegisterPayload,
    responses(
        (status = 200, description = "User created!.", body = User)
    ),
    tag = "Auth"
)]
pub async fn register(
    State(pool): State<DbPool>,
    Json(payload): Json<RegisterPayload>,
) -> impl IntoResponse {
    let hash: String = match password::hash_password(&payload.password) {
        Ok(h) => h,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Hash error").into_response(),
    };

    let user: Result<User, sqlx::Error> = sqlx::query_as!(
        User,
        r#"INSERT INTO users (email, password)
        VALUES ($1, $2)
        RETURNING id, email, password, created_at"#,
        payload.email,
        hash
    )
    .fetch_one(&pool)
    .await;

    match user {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => {
            println!("Error creating user: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed").into_response()
        }
    }
}

#[openapi_path(
    post,
    path = "/auth/login",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "Successful login"),
        (status = 401, description = "Incorrect credentials")
    ),
    tag = "Auth"
)]
pub async fn login(
    State(pool): State<DbPool>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let user: Option<User> =
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", payload.email)
            .fetch_optional(&pool)
            .await
            .unwrap();

    if let Some(user) = user {
        if password::verify_password(&user.password, &payload.password).unwrap_or(false) {
            let token: String = jwt::generate_token(&user.id.to_string());
            return (StatusCode::OK, Json(serde_json::json!({ "token": token }))).into_response();
        }
    }

    (StatusCode::UNAUTHORIZED, "Incorrect credentials").into_response()
}
