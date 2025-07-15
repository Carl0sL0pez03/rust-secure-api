use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    response::Response as ResponseAxum,
};
use tokio::sync::Mutex;
use tower::{Layer, Service};

#[derive(Debug, Clone)]
pub struct RateLimitLayer {
    state: Arc<Mutex<HashMap<SocketAddr, Instant>>>,
    cooldown: Duration,
}

impl RateLimitLayer {
    pub fn new(cooldown: Duration) -> Self {
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
            cooldown,
        }
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimiterService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimiterService {
            inner,
            state: self.state.clone(),
            cooldown: self.cooldown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RateLimiterService<S> {
    inner: S,
    state: Arc<Mutex<HashMap<SocketAddr, Instant>>>,
    cooldown: Duration,
}

impl<S> Service<Request<Body>> for RateLimiterService<S>
where
    S: Service<Request<Body>, Response = ResponseAxum> + Clone + Send + 'static,
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
        let client_ip: SocketAddr = req
            .extensions()
            .get::<SocketAddr>()
            .cloned()
            .unwrap_or_else(|| "127.0.0.1:0".parse().unwrap());

        let state: Arc<Mutex<HashMap<SocketAddr, Instant>>> = self.state.clone();
        let cooldown: Duration = self.cooldown;
        let mut inner: S = self.inner.clone();

        Box::pin(async move {
            let now: Instant = Instant::now();

            {
                let mut map: tokio::sync::MutexGuard<'_, HashMap<SocketAddr, Instant>> =
                    state.lock().await;
                if let Some(last_call) = map.get(&client_ip) {
                    if now.duration_since(*last_call) < cooldown {
                        return Ok(Response::builder()
                            .status(StatusCode::TOO_MANY_REQUESTS)
                            .header("Retry-After", cooldown.as_secs().to_string())
                            .body(Body::from("⏳ Rate limit reached. Try again later."))
                            .unwrap());
                    }
                }

                map.insert(client_ip, now);
            }

            inner.call(req).await
        })
    }
}
