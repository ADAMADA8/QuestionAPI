use crate::handlers::user::*;
use axum::routing::get;
use axum::Router;

pub(crate) fn user_router() -> Router {
    Router::new()
        .route("/can_start", get(can_start))
        .route("/start", get(start))
        .route("/check_answer", get(check_answer))
}
