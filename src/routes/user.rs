use crate::handlers::user::*;
use axum::routing::{get, post};
use axum::Router;

pub(crate) fn user_router() -> Router {
    Router::new()
        .route("/can_start", get(can_start))
        .route("/start", get(start))
        .route("/send_answer", post(send_answer))
}
