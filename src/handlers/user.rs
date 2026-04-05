use crate::storage::CURRENT_QUESTION;
use crate::QUESTIONS;
use anyhow::Result;
use axum::http::{HeaderMap, StatusCode};
use tokio::fs;
use uuid::Uuid;

pub(crate) async fn can_start() -> Result<String, StatusCode> {
    let uuid = fs::read_to_string("session.txt").await.unwrap();

    if uuid.is_empty() {
        return Ok(true.to_string());
    }

    Ok(false.to_string())
}

pub(crate) async fn start() -> Result<String, StatusCode> {
    let uuid = fs::read_to_string("session.txt").await.unwrap();

    if !uuid.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    *CURRENT_QUESTION.lock().await = 0;
    let uuid = Uuid::new_v4().to_string();
    fs::write("session.txt", uuid.as_bytes()).await.unwrap();
    Ok(uuid)
}

pub(crate) async fn check_answer(
    headers: HeaderMap,
    body: String,
) -> Result<&'static str, StatusCode> {
    let key = headers
        .get("Key")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .to_string();
    let key = Uuid::parse_str(&key).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let uuid = fs::read_to_string("session.txt").await.unwrap();
    let uuid = Uuid::parse_str(&uuid).unwrap();
    if key != uuid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let current = *CURRENT_QUESTION.lock().await as usize;
    let questions = QUESTIONS.get().unwrap();
    let question = questions.get(current).ok_or(StatusCode::NOT_FOUND)?;

    if question.answer != body {
        return Ok("false");
    }

    *CURRENT_QUESTION.lock().await += 1;
    if questions.len() == current + 1 {
        return Ok("true");
    }

    Ok(&questions.get(current + 1).unwrap().question)
}
