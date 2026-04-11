use crate::config::CONFIG;
use crate::handlers::admin::*;
use axum::body::Body;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::{get, post};
use axum::{middleware, Router};

async fn require_admin(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    match token {
        Some(t) if t == CONFIG.admin_token => Ok(next.run(req).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

pub(crate) fn admin_router() -> Router {
    Router::new()
        .route("/reset_session", post(reset_session))
        .route("/inventory/add", post(add_inventory_item))
        .route("/questions", get(questions))
        .route("/current_question", get(current_question))
        .layer(middleware::from_fn(require_admin))
}
