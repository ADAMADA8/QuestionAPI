use crate::handlers::user::*;
use crate::storage;
use axum::body::Body;
use axum::extract::Request;
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::{from_fn, Next};
use axum::response::Response;
use axum::routing::{get, post};
use axum::Router;
use uuid::Uuid;

async fn require_key(
    headers: HeaderMap,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let key = headers
        .get("Key")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .to_string();
    let key = Uuid::parse_str(&key).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let uuid = storage::read(|state| state.session_id.clone());
    let uuid = Uuid::parse_str(&uuid).unwrap();

    if key != uuid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}

pub(crate) fn user_router() -> Router {
    let protected_routes = Router::new()
        .route("/send_answer", post(send_answer))
        .route("/current_question", get(current_question))
        .route("/inventory", get(inventory))
        .route_layer(from_fn(require_key));

    Router::new()
        .route("/can_start", get(can_start))
        .route("/start", get(start))
        .merge(protected_routes)
}
