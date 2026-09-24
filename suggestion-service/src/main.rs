use std::{env, time::Duration};
use suggestion_service::{AppState, router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
        .init();
    let state = AppState::new(
        env::var("INDEX_BUILDER_URL").unwrap_or_else(|_| "http://index-builder-worker:8080".into()),
    );
    let refresher = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            if let Err(error) = refresher.refresh().await {
                tracing::warn!(%error, "snapshot refresh failed; keeping the last valid index");
            }
        }
    });
    let addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into());
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind listener");
    tracing::info!(%addr, "suggestion service listening");
    axum::serve(listener, router(state))
        .await
        .expect("serve suggestion service");
}
