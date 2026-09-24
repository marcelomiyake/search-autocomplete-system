use std::time::Duration;

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use query_analytics_service::{
    AppState, connect, initialize, list_frequencies, router, seed_sample_data,
};
use serde_json::{Value, json};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

fn json_request(method: &str, uri: &str, value: Value) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(value.to_string()))
        .unwrap()
}

async fn call(app: &Router, request: Request<Body>) -> (StatusCode, Value) {
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
async fn postgres_http_contract_validates_records_and_reports_failures() {
    let Ok(url) = std::env::var("TEST_DATABASE_URL") else {
        eprintln!("skipping PostgreSQL integration: set TEST_DATABASE_URL");
        return;
    };
    let pool = connect(&url).await.expect("test PostgreSQL connection");
    initialize(&pool).await.expect("initialize schema");

    seed_sample_data(&pool).await.unwrap();
    seed_sample_data(&pool).await.unwrap();
    let seeded = list_frequencies(&pool).await.unwrap();
    assert_eq!(
        seeded
            .terms
            .iter()
            .find(|term| term.term == "the sky is blue")
            .unwrap()
            .frequency,
        3
    );

    let app = router(AppState { pool: pool.clone() });
    let (status, health) = call(
        &app,
        Request::builder()
            .uri("/healthz")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(health["status"], "ok");

    let suffix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let letters = suffix
        .to_string()
        .bytes()
        .map(|digit| (b'a' + digit - b'0') as char)
        .collect::<String>();
    let query = format!("integration {letters}");
    let idempotency_key = format!("integration-{suffix}");
    let event_body = json!({
        "query": format!("  Integration   {letters}  "),
        "idempotency_key": idempotency_key,
    });

    let (status, first) = call(
        &app,
        json_request("POST", "/api/v1/query-events", event_body.clone()),
    )
    .await;
    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(first["accepted"], true);
    assert_eq!(first["query"], query);

    let (status, replay) = call(
        &app,
        json_request("POST", "/api/v1/query-events", event_body),
    )
    .await;
    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(replay["accepted"], false);
    assert_eq!(replay["revision"], first["revision"]);

    for (invalid, expected_message) in [
        (
            json!({ "query": "ok", "idempotency_key": " " }),
            "idempotency_key must contain 1 to 128 characters",
        ),
        (
            json!({ "query": "ok", "idempotency_key": "x".repeat(129) }),
            "idempotency_key must contain 1 to 128 characters",
        ),
        (
            json!({ "query": "café", "idempotency_key": "invalid-query" }),
            "query may contain only English letters and spaces",
        ),
        (
            json!({ "query": "   ", "idempotency_key": "empty-query" }),
            "query cannot be empty",
        ),
        (
            json!({ "query": "a".repeat(51), "idempotency_key": "long-query" }),
            "query must be at most 50 characters",
        ),
    ] {
        let (status, body) =
            call(&app, json_request("POST", "/api/v1/query-events", invalid)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(body["error"], expected_message);
    }

    let (status, frequencies) = call(
        &app,
        Request::builder()
            .uri("/internal/v1/frequencies")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        frequencies["terms"]
            .as_array()
            .unwrap()
            .iter()
            .any(|term| { term["term"] == query && term["frequency"] == 1 })
    );

    let unavailable_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_millis(250))
        .connect_lazy("postgres://autocomplete@127.0.0.1:1/search_autocomplete_test")
        .unwrap();
    let unavailable_app = router(AppState {
        pool: unavailable_pool,
    });

    let (status, _) = call(
        &unavailable_app,
        json_request(
            "POST",
            "/api/v1/query-events",
            json!({ "query": "valid query", "idempotency_key": "db-failure" }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);

    let (status, _) = call(
        &unavailable_app,
        Request::builder()
            .uri("/internal/v1/frequencies")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);

    let (status, health) = call(
        &unavailable_app,
        Request::builder()
            .uri("/healthz")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(health["status"], "unavailable");
}
