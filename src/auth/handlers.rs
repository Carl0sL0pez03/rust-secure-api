use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};

use crate::auth::jwt::decode_token;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, StatusCode> {
        let auth_header: Option<&str> = parts
            .headers
            .get("Authorization")
            .and_then(|h: &axum::http::HeaderValue| h.to_str().ok());

        if let Some(auth_header) = auth_header {
            if let Some(token) = auth_header.strip_prefix("Bearer ") {
                match decode_token(token) {
                    Ok(claims) => {
                        return Ok(AuthUser {
                            user_id: claims.sub,
                        });
                    }
                    Err(_) => return Err(StatusCode::UNAUTHORIZED),
                }
            }
        }

        Err(StatusCode::UNAUTHORIZED)
    }
}
