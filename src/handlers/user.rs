use crate::QUESTIONS;
use anyhow::Result;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use serde::Deserialize;
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

    let uuid = Uuid::new_v4().to_string();
    fs::write("session.txt", uuid.as_bytes()).await.unwrap();
    Ok(uuid)
}

pub(crate) async fn reset_session() -> Result<StatusCode, StatusCode> {
    fs::write("session.txt", b"").await.unwrap();
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub(crate) struct CheckAnswer {
    id: usize,
    answer: String,
}

pub(crate) async fn check_answer(
    headers: HeaderMap,
    Json(body): Json<CheckAnswer>,
) -> Result<String, StatusCode> {
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

    let question = QUESTIONS
        .get()
        .unwrap()
        .get(body.id)
        .ok_or(StatusCode::NOT_FOUND)?;

    if question.answer != body.answer {
        return Ok(false.to_string());
    }

    Ok(true.to_string())
}
