use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use uuid::Uuid;

use crate::{
    auth::extractor::AuthUser,
    errors::AppError,
    models::user::{CreateUser, UpdateUser, User},
    AppState,
};

pub async fn list_users(
    _auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, AppError> {
    Ok(Json(state.store.list_users()))
}

pub async fn get_user(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
    state.store.get_user(&id).map(Json).ok_or(AppError::NotFound)
}

pub async fn create_user(
    _auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), AppError> {
    if state.store.email_exists(&body.email) {
        return Err(AppError::Conflict("email already exists".into()));
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
    Ok((StatusCode::CREATED, Json(state.store.create_user(user))))
}

pub async fn update_user(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateUser>,
) -> Result<Json<User>, AppError> {
    state
        .store
        .update_user(&id, |user| {
            if let Some(username) = body.username {
                user.username = username;
            }
            if let Some(email) = body.email {
                user.email = email;
            }
            user.updated_at = Utc::now();
        })
        .map(Json)
        .ok_or(AppError::NotFound)
}

pub async fn delete_user(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    if state.store.delete_user(&id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}
