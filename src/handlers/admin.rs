use crate::{storage, QA, QUESTIONS};
use anyhow::Result;
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;
use tokio::fs;

pub(crate) async fn reset_session() -> Result<StatusCode, StatusCode> {
    fs::write("session.txt", b"").await.unwrap();
    storage::write(|state| {
        state.session_id = Arc::from("");
        state.question_number = 0;
    });
    Ok(StatusCode::OK)
}

pub(crate) async fn questions() -> Result<Json<&'static Vec<QA>>, StatusCode> {
    Ok(Json(QUESTIONS.get().unwrap()))
}

pub(crate) async fn current_question() -> Result<String, StatusCode> {
    let current = storage::read(|state| state.question_number);
    Ok(current.to_string())
}
