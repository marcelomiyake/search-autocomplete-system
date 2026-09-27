use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;

use axum::{
    Json, Router,
    body::{Body, to_bytes},
    extract::State,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::get,
};
use index_builder_worker::{
    AppState, FrequenciesResponse, TermFrequency, build_once, connect, initialize, load_latest,
    persist_snapshot, router,
};
use serde_json::Value;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

async fn upstream_frequencies(
    State((fail, frequencies)): State<(Arc<AtomicBool>, FrequenciesResponse)>,
) -> impl IntoResponse {
    if fail.load(Ordering::Relaxed) {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }
    Json(frequencies).into_response()
}

async fn call(app: &Router, uri: &str) -> (StatusCode, Value) {
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
async fn postgres_snapshot_build_routes_and_failure_responses() {
    let Ok(url) = std::env::var("TEST_DATABASE_URL") else {
        eprintln!("skipping PostgreSQL integration: set TEST_DATABASE_URL");
        return;
    };
    let pool = connect(&url).await.expect("test PostgreSQL connection");
    initialize(&pool).await.expect("initialize schema");
    sqlx::query("DELETE FROM index_builder.snapshots")
        .execute(&pool)
        .await
        .expect("clear snapshots in the isolated test database");

    let app = router(AppState { pool: pool.clone() });
    let (status, empty) = call(&app, "/internal/v1/snapshots/latest").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(empty, Value::Null);

    let (status, health) = call(&app, "/healthz").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(health["status"], "ok");

    let minimum_revision = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let expected = FrequenciesResponse {
        revision: minimum_revision,
        terms: vec![TermFrequency {
            term: "snapshot integration".into(),
            frequency: 3,
        }],
    };
    let fail = Arc::new(AtomicBool::new(false));
    let analytics = Router::new()
        .route("/internal/v1/frequencies", get(upstream_frequencies))
        .with_state((fail.clone(), expected.clone()));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, analytics).await.unwrap() });

    let http = reqwest::Client::new();
    let analytics_url = format!("http://{address}");
    let built = build_once(&pool, &analytics_url, &http)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(built.revision, expected.revision);
    assert_eq!(built.terms[0].term, "snapshot integration");
    assert!(
        !persist_snapshot(&pool, &built)
            .await
            .expect("replay snapshot write")
    );
    assert_eq!(load_latest(&pool).await.unwrap(), Some(built.clone()));
    assert_eq!(
        build_once(&pool, &analytics_url, &http).await.unwrap(),
        Some(built.clone())
    );

    let (status, current) = call(&app, "/internal/v1/snapshots/latest").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(current["revision"], expected.revision);
    assert_eq!(current["terms"][0]["term"], "snapshot integration");

    fail.store(true, Ordering::Relaxed);
    assert!(build_once(&pool, &analytics_url, &http).await.is_err());

    let unavailable_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_millis(250))
        .connect_lazy("postgres://autocomplete@127.0.0.1:1/search_autocomplete_test")
        .unwrap();
    let unavailable_app = router(AppState {
        pool: unavailable_pool,
    });
    let (status, _) = call(&unavailable_app, "/internal/v1/snapshots/latest").await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);

    let (status, health) = call(&unavailable_app, "/healthz").await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(health["status"], "unavailable");
}
