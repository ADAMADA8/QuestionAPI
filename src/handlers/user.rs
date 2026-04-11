use crate::storage;
use crate::QUESTIONS;
use anyhow::Result;
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;
use uuid::Uuid;

pub(crate) async fn can_start() -> Result<String, StatusCode> {
    let uuid = storage::read(|state| state.session_id.clone());

    if uuid.is_empty() {
        return Ok(true.to_string());
    }

    Ok(false.to_string())
}

pub(crate) async fn start() -> Result<Json<String>, StatusCode> {
    let current = storage::read(|state| state.session_id.clone());

    if !current.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let uuid = Uuid::new_v4().to_string();

    storage::write(|state| {
        state.session_id = Arc::from(uuid.as_str());
        state.question_number = 0;
    });

    Ok(Json::from(uuid))
}

pub(crate) async fn current_question() -> Result<Json<&'static str>, StatusCode> {
    let current = storage::read(|state| state.question_number);
    let current = &*QUESTIONS.get().unwrap().get(current).unwrap().question;
    Ok(Json::from(current))
}

pub(crate) async fn send_answer(body: String) -> Result<&'static str, StatusCode> {
    let current = storage::read(|state| state.question_number);

    let questions = QUESTIONS.get().unwrap();
    let question = questions.get(current).ok_or(StatusCode::NOT_FOUND)?;

    if question.answer != body {
        return Ok("false");
    }

    storage::write(|state| state.question_number = current + 1);
    Ok("true")
}
