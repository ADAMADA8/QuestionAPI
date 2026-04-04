use crate::routes::user::user_router;
use axum::Router;

pub(crate) fn app_router() -> Router {
    Router::new()
        .nest("/api", user_router())
        .nest("/api/admin", crate::routes::admin::admin_router())
}
