use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering},
};

use axum::{
    Json, Router,
    body::{Body, to_bytes},
    extract::State,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::get,
};
use serde_json::Value;
use suggestion_service::{AppState, Snapshot, TermFrequency, router};
use tower::ServiceExt;

async fn latest(State(mode): State<Arc<AtomicU8>>) -> impl IntoResponse {
    match mode.load(Ordering::Relaxed) {
        1 => StatusCode::NOT_FOUND.into_response(),
        2 => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error":"offline"})),
        )
            .into_response(),
        _ => (
            StatusCode::OK,
            Json(Snapshot {
                revision: 7,
                terms: vec![TermFrequency {
                    term: "rust async".into(),
                    frequency: 9,
                }],
            }),
        )
            .into_response(),
    }
}

async fn request_json(app: &Router, uri: &str) -> (StatusCode, Value) {
    let request = Request::builder().uri(uri).body(Body::empty()).unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json = if body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&body).unwrap()
    };
    (status, json)
}

#[tokio::test]
async fn refresh_loads_new_snapshot_and_keeps_it_on_endpoint_failure() {
    let mode = Arc::new(AtomicU8::new(0));
    let app = Router::new()
        .route("/internal/v1/snapshots/latest", get(latest))
        .with_state(mode.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let state = AppState::new(format!("http://{address}"));
    assert!(state.refresh().await.unwrap());
    assert_eq!(state.revision().await, 7);
    assert_eq!(state.suggest("rust").await[0].query, "rust async");

    let api = router(state.clone());
    let (status, suggestions) = request_json(&api, "/api/v1/suggestions?prefix=ru").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(suggestions["suggestions"][0]["query"], "rust async");

    let (status, empty) = request_json(&api, "/api/v1/suggestions?prefix=%20%20").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(empty["suggestions"].as_array().unwrap().len(), 0);

    let (status, invalid) = request_json(&api, "/api/v1/suggestions?prefix=caf%C3%A9").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(
        invalid["error"],
        "prefix may contain only English letters and spaces"
    );

    let (status, health) = request_json(&api, "/healthz").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(health["snapshotRevision"], 7);

    assert!(!state.refresh().await.unwrap());
    mode.store(1, Ordering::Relaxed);
    assert!(!state.refresh().await.unwrap());
    assert_eq!(state.revision().await, 7);

    mode.store(2, Ordering::Relaxed);
    assert!(state.refresh().await.is_err());
    assert_eq!(state.revision().await, 7);
    assert_eq!(state.suggest("rust").await[0].query, "rust async");
}
