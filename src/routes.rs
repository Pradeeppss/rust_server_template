use axum:: {routing::get, Router};

use crate::handlers::tasks;
use crate::states::AppState;


pub fn app(state: AppState) -> Router{
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/tasks", get(tasks::get_all).post(tasks::create))
        .route("/tasks/{id}", get(tasks::get_one).delete(tasks::delete).patch(tasks::update))
        .with_state(state)
}

async fn root() -> & 'static str {
    "server available"
}

async fn health() -> & 'static str {
    "server healthy"
}