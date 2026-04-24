#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub jwt_expiry_hours: u64,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "change-me-in-production-secret-key".into()),
            jwt_expiry_hours: std::env::var("JWT_EXPIRY_HOURS")
                .ok()
                .and_then(|h| h.parse().ok())
                .unwrap_or(24),
        }
    }
}
