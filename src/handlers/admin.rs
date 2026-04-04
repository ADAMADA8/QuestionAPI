use crate::storage::CURRENT_QUESTION;
use crate::{QA, QUESTIONS};
use anyhow::Result;
use axum::http::StatusCode;
use axum::Json;
use tokio::fs;

pub(crate) async fn reset_session() -> Result<StatusCode, StatusCode> {
    fs::write("session.txt", b"").await.unwrap();
    *CURRENT_QUESTION.lock().await = 0;
    Ok(StatusCode::OK)
}

pub(crate) async fn questions() -> Result<Json<&'static Vec<QA>>, StatusCode> {
    Ok(Json(QUESTIONS.get().unwrap()))
}

pub(crate) async fn current_question() -> Result<String, StatusCode> {
    let current = *CURRENT_QUESTION.lock().await;
    Ok(current.to_string())
}
