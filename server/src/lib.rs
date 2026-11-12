//! Fretwise local server: serves the single-page app and stores songs in SQLite.
//!
//! Routes
//! - `GET  /`                 the app (`index.html`)
//! - `GET  /api/health`       `{ ok, version, storage }`
//! - `GET  /api/songs`        every song, newest first
//! - `POST /api/songs`        create a song
//! - `GET  /api/songs/{id}`   one song
//! - `PUT  /api/songs/{id}`   replace a song
//! - `DELETE /api/songs/{id}` delete a song
//! - `GET  /api/settings`     the app settings backup (or 404)
//! - `PUT  /api/settings`     store the app settings backup

pub mod db;

use axum::{
    extract::{DefaultBodyLimit, Path, State},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use db::{Db, SongInput, MAX_SETTINGS};
use serde_json::json;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub struct AppState {
    pub db: Mutex<Db>,
    /// Where `index.html` is read from on every request, so edits show up on reload.
    pub index_path: PathBuf,
    pub storage: String,
}

pub type Shared = Arc<AppState>;

/// Error type that turns into a JSON `{ "error": "..." }` response.
pub struct ApiError(StatusCode, String);

impl ApiError {
    fn bad(msg: impl Into<String>) -> Self {
        ApiError(StatusCode::BAD_REQUEST, msg.into())
    }
    fn not_found() -> Self {
        ApiError(StatusCode::NOT_FOUND, "not found".into())
    }
}

impl From<rusqlite::Error> for ApiError {
    fn from(e: rusqlite::Error) -> Self {
        tracing::error!("database error: {e}");
        ApiError(StatusCode::INTERNAL_SERVER_ERROR, "database error".into())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.0, Json(json!({ "error": self.1 }))).into_response()
    }
}

type ApiResult<T> = Result<T, ApiError>;

pub fn router(state: Shared) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/index.html", get(index))
        .route("/api/health", get(health))
        .route("/api/songs", get(list_songs).post(create_song))
        .route("/api/songs/{id}", get(get_song).put(update_song).delete(delete_song))
        .route("/api/settings", get(get_settings).put(put_settings))
        .layer(DefaultBodyLimit::max(MAX_SETTINGS + 4096))
        .with_state(state)
}

async fn index(State(st): State<Shared>) -> Response {
    match tokio::fs::read_to_string(&st.index_path).await {
        Ok(html) => (
            [(header::CACHE_CONTROL, "no-cache")],
            Html(html),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Could not read {}: {e}", st.index_path.display()),
        )
            .into_response(),
    }
}

async fn health(State(st): State<Shared>) -> Json<serde_json::Value> {
    Json(json!({ "ok": true, "version": env!("CARGO_PKG_VERSION"), "storage": st.storage }))
}

async fn list_songs(State(st): State<Shared>) -> ApiResult<Json<serde_json::Value>> {
    let songs = st.db.lock().unwrap().list_songs()?;
    Ok(Json(json!(songs)))
}

async fn get_song(State(st): State<Shared>, Path(id): Path<i64>) -> ApiResult<Json<serde_json::Value>> {
    let s = st.db.lock().unwrap().get_song(id)?.ok_or_else(ApiError::not_found)?;
    Ok(Json(json!(s)))
}

async fn create_song(
    State(st): State<Shared>,
    Json(input): Json<SongInput>,
) -> ApiResult<(StatusCode, Json<serde_json::Value>)> {
    let input = input.validate().map_err(ApiError::bad)?;
    let s = st.db.lock().unwrap().create_song(&input)?;
    Ok((StatusCode::CREATED, Json(json!(s))))
}

async fn update_song(
    State(st): State<Shared>,
    Path(id): Path<i64>,
    Json(input): Json<SongInput>,
) -> ApiResult<Json<serde_json::Value>> {
    let input = input.validate().map_err(ApiError::bad)?;
    let s = st.db.lock().unwrap().update_song(id, &input)?.ok_or_else(ApiError::not_found)?;
    Ok(Json(json!(s)))
}

async fn delete_song(State(st): State<Shared>, Path(id): Path<i64>) -> ApiResult<StatusCode> {
    if st.db.lock().unwrap().delete_song(id)? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::not_found())
    }
}

async fn get_settings(State(st): State<Shared>) -> ApiResult<Json<serde_json::Value>> {
    let (v, at) = st.db.lock().unwrap().get_settings()?.ok_or_else(ApiError::not_found)?;
    Ok(Json(json!({ "settings": v, "updated_at": at })))
}

async fn put_settings(
    State(st): State<Shared>,
    Json(v): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    if !v.is_object() {
        return Err(ApiError::bad("settings must be a JSON object"));
    }
    if v.to_string().len() > MAX_SETTINGS {
        return Err(ApiError::bad("settings are too large"));
    }
    let at = st.db.lock().unwrap().put_settings(&v)?;
    Ok(Json(json!({ "ok": true, "updated_at": at })))
}
