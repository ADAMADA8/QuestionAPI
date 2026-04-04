use crate::{QA, QUESTIONS};
use anyhow::Result;
use axum::http::StatusCode;
use axum::Json;

pub(crate) async fn get_questions() -> Result<Json<&'static Vec<QA>>, StatusCode> {
    Ok(Json(QUESTIONS.get().unwrap()))
}
