use std::time::Duration;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tower_http::trace::TraceLayer;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TermFrequency {
    pub term: String,
    pub frequency: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Snapshot {
    pub revision: i64,
    pub terms: Vec<TermFrequency>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FrequenciesResponse {
    pub revision: i64,
    pub terms: Vec<TermFrequency>,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub fn is_newer_revision(candidate: i64, current: i64) -> bool {
    candidate > current
}

pub async fn initialize(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::raw_sql(include_str!("../migrations/0001_init.sql"))
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn persist_snapshot(pool: &PgPool, snapshot: &Snapshot) -> Result<bool, sqlx::Error> {
    let inserted = sqlx::query("INSERT INTO index_builder.snapshots (revision, snapshot) VALUES ($1, $2) ON CONFLICT (revision) DO NOTHING")
        .bind(snapshot.revision).bind(serde_json::to_value(snapshot).expect("serialize snapshot"))
        .execute(pool).await?.rows_affected() == 1;
    Ok(inserted)
}

pub async fn load_latest(pool: &PgPool) -> Result<Option<Snapshot>, sqlx::Error> {
    let value = sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT snapshot FROM index_builder.snapshots ORDER BY revision DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;
    value
        .map(|value| {
            serde_json::from_value(value).map_err(|error| sqlx::Error::Decode(Box::new(error)))
        })
        .transpose()
}

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}

async fn latest(State(state): State<AppState>) -> impl IntoResponse {
    match load_latest(&state.pool).await {
        Ok(Some(snapshot)) => (StatusCode::OK, Json(snapshot)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(error) => {
            tracing::error!(%error, "failed to load latest snapshot");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
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
        .route("/internal/v1/snapshots/latest", get(latest))
        .route("/healthz", get(health))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

pub async fn build_once(
    pool: &PgPool,
    analytics_url: &str,
    http: &reqwest::Client,
) -> Result<Option<Snapshot>, Box<dyn std::error::Error + Send + Sync>> {
    let response = http
        .get(format!(
            "{}/internal/v1/frequencies",
            analytics_url.trim_end_matches('/')
        ))
        .send()
        .await?
        .error_for_status()?;
    let frequencies: FrequenciesResponse = response.json().await?;
    if let Some(current) = load_latest(pool).await?
        && !is_newer_revision(frequencies.revision, current.revision)
    {
        return Ok(Some(current));
    }
    let snapshot = Snapshot {
        revision: frequencies.revision,
        terms: frequencies.terms,
    };
    persist_snapshot(pool, &snapshot).await?;
    Ok(Some(snapshot))
}

pub async fn run_builder(pool: PgPool, analytics_url: String) {
    let http = reqwest::Client::new();
    let period = std::env::var("REFRESH_SECONDS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(15);
    let mut interval = tokio::time::interval(Duration::from_secs(period));
    loop {
        interval.tick().await;
        if let Err(error) = build_once(&pool, &analytics_url, &http).await {
            tracing::warn!(%error, "snapshot build failed; retaining the last valid snapshot");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_newer_snapshot_revisions_are_applied() {
        let current = Snapshot {
            revision: 7,
            terms: vec![TermFrequency {
                term: "rust".into(),
                frequency: 12,
            }],
        };
        let newer = Snapshot {
            revision: 8,
            terms: vec![TermFrequency {
                term: "rust".into(),
                frequency: 13,
            }],
        };
        assert!(is_newer_revision(newer.revision, current.revision));
        assert!(!is_newer_revision(current.revision, current.revision));
        assert!(!is_newer_revision(current.revision - 1, current.revision));
        assert_eq!(current.terms[0].term, "rust");
    }

    #[tokio::test]
    async fn builder_loop_keeps_running_after_upstream_errors() {
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://autocomplete@127.0.0.1:1/search_autocomplete_test")
            .unwrap();
        let result = tokio::time::timeout(
            Duration::from_millis(100),
            run_builder(pool, "http://127.0.0.1:1".into()),
        )
        .await;
        assert!(result.is_err(), "the periodic worker should keep retrying");
    }
}
