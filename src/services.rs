use crate::handlers::{can_start, check_answer, reset_session, start};
use axum::routing::{get, post};
use axum::Router;

pub(crate) fn app_router() -> Router {
    let app_routes = Router::new()
        .route("/can_start", get(can_start))
        .route("/start", get(start))
        .route("/reset_session", post(reset_session))
        .route("/check_answer", get(check_answer));

    Router::new().nest("/api", app_routes)
}
