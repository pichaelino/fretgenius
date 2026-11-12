use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use fretwise_server::{db::Db, router, AppState};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use tower::ServiceExt;

fn app() -> Router {
    let index = std::env::temp_dir().join("fretwise-test-index.html");
    std::fs::write(&index, "<title>Fretwise test</title>").unwrap();
    router(Arc::new(AppState {
        db: Mutex::new(Db::open_in_memory().unwrap()),
        index_path: index,
        storage: "memory".into(),
    }))
}

async fn call(app: &Router, method: &str, uri: &str, body: Option<Value>) -> (StatusCode, Value) {
    let mut req = Request::builder().method(method).uri(uri);
    let body = match body {
        Some(b) => {
            req = req.header("content-type", "application/json");
            Body::from(b.to_string())
        }
        None => Body::empty(),
    };
    let res = app.clone().oneshot(req.body(body).unwrap()).await.unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let v = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, v)
}

#[tokio::test]
async fn serves_the_app_and_health() {
    let app = app();
    let res = app.clone().oneshot(Request::get("/").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let html = res.into_body().collect().await.unwrap().to_bytes();
    assert!(String::from_utf8_lossy(&html).contains("Fretwise test"));

    let (s, v) = call(&app, "GET", "/api/health", None).await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(v["ok"], true);
}

#[tokio::test]
async fn song_lifecycle() {
    let app = app();
    let (s, v) = call(&app, "GET", "/api/songs", None).await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(v, json!([]));

    let (s, created) = call(
        &app,
        "POST",
        "/api/songs",
        Some(json!({"title": "Sunday", "chords": "Am ~Am F C@x32010 G", "bpm": 96, "time_sig": "3/4", "style": "travis", "legato": true})),
    )
    .await;
    assert_eq!(s, StatusCode::CREATED);
    let id = created["id"].as_i64().unwrap();
    assert_eq!(created["chords"], "Am ~Am F C@x32010 G");
    assert_eq!(created["key"], Value::Null);

    let (s, got) = call(&app, "GET", &format!("/api/songs/{id}"), None).await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(got["title"], "Sunday");

    let (s, upd) = call(
        &app,
        "PUT",
        &format!("/api/songs/{id}"),
        Some(json!({"title": "Sunday (v2)", "chords": "Am F", "key": {"t": 9, "mode": "minor"}})),
    )
    .await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(upd["title"], "Sunday (v2)");
    assert_eq!(upd["bpm"], 90, "missing fields fall back to defaults");
    assert_eq!(upd["key"]["mode"], "minor");

    let (_, list) = call(&app, "GET", "/api/songs", None).await;
    assert_eq!(list.as_array().unwrap().len(), 1);

    let (s, _) = call(&app, "DELETE", &format!("/api/songs/{id}"), None).await;
    assert_eq!(s, StatusCode::NO_CONTENT);
    let (s, _) = call(&app, "DELETE", &format!("/api/songs/{id}"), None).await;
    assert_eq!(s, StatusCode::NOT_FOUND);
    let (s, _) = call(&app, "GET", &format!("/api/songs/{id}"), None).await;
    assert_eq!(s, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn rejects_bad_input() {
    let app = app();
    let (s, v) = call(&app, "POST", "/api/songs", Some(json!({"title": " ", "chords": "Am"}))).await;
    assert_eq!(s, StatusCode::BAD_REQUEST);
    assert_eq!(v["error"], "title is empty");

    let (s, _) = call(&app, "POST", "/api/songs", Some(json!({"title": "x", "chords": "Am", "time_sig": "7/8"}))).await;
    assert_eq!(s, StatusCode::BAD_REQUEST);

    let (s, _) = call(&app, "PUT", "/api/songs/999", Some(json!({"title": "x", "chords": "Am"}))).await;
    assert_eq!(s, StatusCode::NOT_FOUND);

    let (s, _) = call(&app, "PUT", "/api/settings", Some(json!([1, 2]))).await;
    assert_eq!(s, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn settings_backup() {
    let app = app();
    let (s, _) = call(&app, "GET", "/api/settings", None).await;
    assert_eq!(s, StatusCode::NOT_FOUND);
    let (s, _) = call(&app, "PUT", "/api/settings", Some(json!({"theme": "light", "bpm": 120}))).await;
    assert_eq!(s, StatusCode::OK);
    let (s, v) = call(&app, "GET", "/api/settings", None).await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(v["settings"]["theme"], "light");
}
