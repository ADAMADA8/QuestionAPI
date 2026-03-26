use crate::QUESTIONS;
use anyhow::Result;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use std::sync::{Arc, LazyLock};
use tokio::sync::RwLock;
use uuid::Uuid;

pub(crate) type SessionUuid = Arc<RwLock<Option<String>>>;
static UUID_STATE: LazyLock<SessionUuid> = LazyLock::new(|| Arc::new(RwLock::new(None)));

pub(crate) async fn can_start() -> Result<String, StatusCode> {
    let state = &*UUID_STATE.read().await;

    if state.is_some() {
        return Ok(false.to_string());
    }

    Ok(true.to_string())
}

pub(crate) async fn start() -> Result<String, StatusCode> {
    let state = UUID_STATE.read().await;

    if state.is_some() {
        return Err(StatusCode::BAD_REQUEST);
    }
    drop(state);

    let uuid = Uuid::new_v4().to_string();
    *UUID_STATE.write().await = Some(uuid.clone());
    Ok(uuid)
}

pub(crate) async fn reset_session() -> Result<StatusCode, StatusCode> {
    *UUID_STATE.write().await = None;
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub(crate) struct CheckAnswer {
    id: usize,
    answer: String,
}

pub(crate) async fn check_answer(Json(body): Json<CheckAnswer>) -> Result<String, StatusCode> {
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
