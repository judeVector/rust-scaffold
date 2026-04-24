use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use uuid::Uuid;

use crate::{auth::jwt::validate_token, errors::AppError, AppState};

pub struct AuthUser {
    pub id: Uuid,
}

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let token = auth.strip_prefix("Bearer ").ok_or(AppError::Unauthorized)?;
        let claims = validate_token(token, &state.config.jwt_secret)?;
        Ok(AuthUser { id: claims.sub })
    }
}
