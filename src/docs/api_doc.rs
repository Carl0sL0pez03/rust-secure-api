use crate::models::user::{LoginPayload, RegisterPayload, User};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::auth_routes::register,
        crate::routes::auth_routes::login,
        crate::routes::protected::me
    ),
    components(schemas(User, RegisterPayload, LoginPayload)),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "User", description = "Authenticated user endpoints")
    )
)]
pub struct ApiDoc;
