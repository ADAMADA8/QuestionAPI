use crate::handlers::user::{can_start, check_answer, reset_session, start};
use axum::routing::{get, post};
use axum::Router;

pub(crate) fn user_router() -> Router {
    Router::new()
        .route("/can_start", get(can_start))
        .route("/start", get(start))
        .route("/reset_session", post(reset_session))
        .route("/check_answer", get(check_answer))
}
