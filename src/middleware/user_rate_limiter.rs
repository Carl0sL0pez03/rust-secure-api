use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use tokio::sync::Mutex;
use tower::{Layer, Service};

use crate::auth::handlers::AuthUser;

#[derive(Debug, Clone)]
pub struct UserRateLimiterLayer {
    state: Arc<Mutex<HashMap<String, Instant>>>,
    cooldown: Duration,
}

impl UserRateLimiterLayer {
    pub fn new(cooldown: Duration) -> Self {
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
            cooldown,
        }
    }
}

impl<S> Layer<S> for UserRateLimiterLayer {
    type Service = UserRateLimiterService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        UserRateLimiterService {
            inner,
            state: self.state.clone(),
            cooldown: self.cooldown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserRateLimiterService<S> {
    inner: S,
    state: Arc<Mutex<HashMap<String, Instant>>>,
    cooldown: Duration,
}

impl<S> Service<Request<Body>> for UserRateLimiterService<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<axum::BoxError> + Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let state: Arc<Mutex<HashMap<String, Instant>>> = self.state.clone();
        let cooldown: Duration = self.cooldown;
        let mut inner: S = self.inner.clone();

        Box::pin(async move {
            let auth_user: Option<AuthUser> = req.extensions().get::<AuthUser>().cloned();

            if let Some(auth_user) = auth_user {
                let now = Instant::now();
                let mut map = state.lock().await;

                let last_access = map.get(&auth_user.user_id).copied();
                let reset_seconds = if let Some(last) = last_access {
                    let elapsed = now.duration_since(last);
                    if elapsed < cooldown {
                        let retry_after = cooldown - elapsed;

                        return Ok(axum::http::Response::builder()
                            .status(StatusCode::TOO_MANY_REQUESTS)
                            .header("Retry-After", retry_after.as_secs().to_string())
                            .header("X-RateLimit-Limit", "1")
                            .header("X-RateLimit-Remaining", "0")
                            .header("X-RateLimit-Reset", retry_after.as_secs().to_string())
                            .body(Body::from(
                                "Rate limit for user reached. Try again later.",
                            ))
                            .unwrap());
                    }
                    0
                } else {
                    0
                };

                map.insert(auth_user.user_id.clone(), now);

                let response: axum::http::Response<Body> = inner.call(req).await?;
                let mut response: axum::http::Response<Body> = response;

                response
                    .headers_mut()
                    .insert("X-RateLimit-Limit", "1".parse().unwrap());
                response
                    .headers_mut()
                    .insert("X-RateLimit-Remaining", "0".parse().unwrap());
                response.headers_mut().insert(
                    "X-RateLimit-Reset",
                    reset_seconds.to_string().parse().unwrap(),
                );

                return Ok(response);
            }

            inner.call(req).await
        })
    }
}
