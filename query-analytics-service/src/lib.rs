use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

#[derive(Debug, Deserialize)]
pub struct RecordQuery {
    pub query: String,
    pub idempotency_key: String,
}

#[derive(Debug, Serialize)]
pub struct RecordResponse {
    pub accepted: bool,
    pub query: String,
    pub revision: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TermFrequency {
    pub term: String,
    pub frequency: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct FrequenciesResponse {
    pub revision: i64,
    pub terms: Vec<TermFrequency>,
}

const SAMPLE_SEARCHES: &[(&str, &str)] = &[
    ("demo-1", "rust"),
    ("demo-2", "rust"),
    ("demo-3", "rust async"),
    ("demo-4", "rust web"),
    ("demo-5", "rust systems"),
    ("demo-6", "rust book"),
    ("demo-7", "rust testing"),
    ("demo-8", "rust programming"),
    ("demo-9", "rust ownership"),
    ("demo-10", "rust cli"),
    ("demo-11", "rust"),
    ("demo-12", "rust web"),
    ("demo-13", "the sky is blue"),
    ("demo-14", "the sky is blue"),
    ("demo-15", "the sky is blue"),
    ("demo-16", "the sky is clear"),
    ("demo-17", "the sky is clear"),
    ("demo-18", "the sky is beautiful"),
    ("demo-19", "the sky is beautiful"),
    ("demo-20", "the sky is falling"),
    ("demo-21", "the sky is vast"),
];

#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
}

pub fn normalize_query(input: &str) -> Result<String, &'static str> {
    let normalized = input
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase();
    if normalized.is_empty() {
        return Err("query cannot be empty");
    }
    if normalized.len() > 50 {
        return Err("query must be at most 50 characters");
    }
    if !normalized
        .bytes()
        .all(|b| b.is_ascii_lowercase() || b == b' ')
    {
        return Err("query may contain only English letters and spaces");
    }
    Ok(normalized)
}

pub async fn initialize(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::raw_sql(include_str!("../migrations/0001_init.sql"))
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn seed_sample_data(pool: &PgPool) -> Result<(), sqlx::Error> {
    for &(idempotency_key, query) in SAMPLE_SEARCHES {
        record_event(
            pool,
            &RecordQuery {
                query: query.to_owned(),
                idempotency_key: idempotency_key.to_owned(),
            },
        )
        .await?;
    }
    Ok(())
}

pub async fn record_event(
    pool: &PgPool,
    event: &RecordQuery,
) -> Result<RecordResponse, sqlx::Error> {
    let normalized = normalize_query(&event.query).expect("validated query passed to record_event");
    let mut tx = pool.begin().await?;
    let inserted = sqlx::query_scalar::<_, String>(
        "INSERT INTO query_analytics.query_events (idempotency_key, normalized_query) VALUES ($1, $2) ON CONFLICT (idempotency_key) DO NOTHING RETURNING normalized_query"
    ).bind(&event.idempotency_key).bind(&normalized).fetch_optional(&mut *tx).await?;
    let accepted = inserted.is_some();
    let recorded_query = if let Some(recorded_query) = inserted {
        sqlx::query("INSERT INTO query_analytics.query_frequencies (normalized_query, frequency) VALUES ($1, 1) ON CONFLICT (normalized_query) DO UPDATE SET frequency = query_analytics.query_frequencies.frequency + 1")
            .bind(&recorded_query).execute(&mut *tx).await?;
        sqlx::query("UPDATE query_analytics.service_state SET revision = revision + 1 WHERE singleton = TRUE").execute(&mut *tx).await?;
        recorded_query
    } else {
        sqlx::query_scalar::<_, String>(
            "SELECT normalized_query FROM query_analytics.query_events WHERE idempotency_key = $1",
        )
        .bind(&event.idempotency_key)
        .fetch_one(&mut *tx)
        .await?
    };
    let revision = sqlx::query_scalar::<_, i64>(
        "SELECT revision FROM query_analytics.service_state WHERE singleton = TRUE",
    )
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(RecordResponse {
        accepted,
        query: recorded_query,
        revision,
    })
}

pub async fn list_frequencies(pool: &PgPool) -> Result<FrequenciesResponse, sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await?;
    let revision = sqlx::query_scalar::<_, i64>(
        "SELECT revision FROM query_analytics.service_state WHERE singleton = TRUE",
    )
    .fetch_one(&mut *tx)
    .await?;
    let rows = sqlx::query_as::<_, (String, i64)>("SELECT normalized_query, frequency FROM query_analytics.query_frequencies ORDER BY normalized_query")
        .fetch_all(&mut *tx).await?;
    tx.commit().await?;
    Ok(FrequenciesResponse {
        revision,
        terms: rows
            .into_iter()
            .map(|(term, frequency)| TermFrequency { term, frequency })
            .collect(),
    })
}

async fn post_event(
    State(state): State<AppState>,
    Json(event): Json<RecordQuery>,
) -> impl IntoResponse {
    if event.idempotency_key.trim().is_empty() || event.idempotency_key.len() > 128 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "idempotency_key must contain 1 to 128 characters",
            }),
        )
            .into_response();
    }
    if normalize_query(&event.query).is_err() {
        let message = normalize_query(&event.query).unwrap_err();
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: message }),
        )
            .into_response();
    }
    match record_event(&state.pool, &event).await {
        Ok(response) => (StatusCode::ACCEPTED, Json(response)).into_response(),
        Err(error) => {
            tracing::error!(%error, "failed to persist query event");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "query event could not be saved",
                }),
            )
                .into_response()
        }
    }
}

async fn frequencies(State(state): State<AppState>) -> impl IntoResponse {
    match list_frequencies(&state.pool).await {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(error) => {
            tracing::error!(%error, "failed to list query frequencies");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "query frequencies are unavailable",
                }),
            )
                .into_response()
        }
    }
}

async fn health(State(state): State<AppState>) -> impl IntoResponse {
    match sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.pool)
        .await
    {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"status":"ok"}))).into_response(),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"status":"unavailable"})),
        )
            .into_response(),
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/query-events", post(post_event))
        .route("/internal/v1/frequencies", get(frequencies))
        .route("/healthz", get(health))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_committed_search_terms() {
        assert_eq!(normalize_query("  Rust   async ").unwrap(), "rust async");
        assert!(normalize_query("   ").is_err());
        assert!(normalize_query("naïve").is_err());
        assert!(normalize_query(&"a".repeat(51)).is_err());
    }
}
