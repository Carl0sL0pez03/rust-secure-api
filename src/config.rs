use std::env;
use std::net::SocketAddr;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl Config {
    pub fn init() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL not found"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET not found"),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()
                .unwrap(),
        }
    }

    pub fn addr(&self) -> SocketAddr {
        ([0, 0, 0, 0], self.port).into()
    }
}
