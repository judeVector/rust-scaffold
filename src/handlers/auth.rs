use axum::{extract::State, Json};
use chrono::Utc;
use uuid::Uuid;

use crate::{
    auth::{extractor::AuthUser, jwt::create_token},
    errors::AppError,
    models::user::{CreateUser, LoginRequest, LoginResponse, User},
    AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
    if state.store.email_exists(&body.email) {
        return Err(AppError::Conflict("email already registered".into()));
    }
    let password = body.password.clone();
    let password_hash = tokio::task::spawn_blocking(move || bcrypt::hash(password, bcrypt::DEFAULT_COST))
        .await
        .map_err(|_| AppError::Internal)?
        .map_err(|_| AppError::Internal)?;

    let now = Utc::now();
    let user = User {
        id: Uuid::new_v4(),
        username: body.username,
        email: body.email,
        password_hash,
        created_at: now,
        updated_at: now,
    };
    Ok(Json(state.store.create_user(user)))
}

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let user = state
        .store
        .get_user_by_email(&body.email)
        .ok_or(AppError::Unauthorized)?;

    let password = body.password.clone();
    let hash = user.password_hash.clone();
    let valid = tokio::task::spawn_blocking(move || bcrypt::verify(password, &hash))
        .await
        .map_err(|_| AppError::Internal)?
        .map_err(|_| AppError::Internal)?;

    if !valid {
        return Err(AppError::Unauthorized);
    }

    let token = create_token(user.id, &state.config.jwt_secret, state.config.jwt_expiry_hours)?;
    Ok(Json(LoginResponse { token, user }))
}

pub async fn me(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<User>, AppError> {
    state
        .store
        .get_user(&auth.id)
        .map(Json)
        .ok_or(AppError::NotFound)
}
