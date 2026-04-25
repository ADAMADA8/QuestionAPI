use crate::storage;
use crate::storage::StartSessionError;
use anyhow::Result;
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

pub(crate) async fn can_start() -> Result<Json<bool>, StatusCode> {
    let uuid = storage::read(|state| state.session_id.clone());
    Ok(Json::from(uuid.is_empty()))
}

pub(crate) async fn start(body: Json<String>) -> Result<Json<String>, StatusCode> {
    let pin = body.trim();
    if pin.len() != 6 || !pin.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let uuid = Uuid::new_v4().to_string();

    match storage::start_session(pin, &uuid) {
        Ok(()) => {}
        Err(StartSessionError::Unauthorized) => return Err(StatusCode::UNAUTHORIZED),
        Err(StartSessionError::AlreadyStarted) => return Err(StatusCode::BAD_REQUEST),
        Err(StartSessionError::Internal) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

    Ok(Json::from(uuid))
}

pub(crate) async fn current_question() -> Result<Json<String>, StatusCode> {
    let current = storage::read(|state| state.question_number);
    let question = storage::questions()
        .get(current)
        .ok_or(StatusCode::NOT_FOUND)?
        .question
        .clone();

    Ok(Json::from(question))
}

pub(crate) async fn send_answer(body: String) -> Result<Json<bool>, StatusCode> {
    let current = storage::read(|state| state.question_number);

    let questions = storage::questions();
    let question = questions.get(current).ok_or(StatusCode::NOT_FOUND)?;

    if question.answer != body {
        return Ok(Json::from(false));
    }

    storage::write(|state| state.question_number = current + 1)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json::from(true))
}

pub(crate) async fn inventory() -> Result<Json<Vec<storage::Item>>, StatusCode> {
    Ok(Json(storage::inventory()))
}

