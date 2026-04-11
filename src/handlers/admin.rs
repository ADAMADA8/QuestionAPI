use crate::storage;
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
        state.inventory_ids.clear();
    });
    Ok(StatusCode::OK)
}

pub(crate) async fn add_inventory_item(body: String) -> Result<StatusCode, StatusCode> {
    let trimmed = body.trim();
    if trimmed.is_empty() || trimmed.split_whitespace().count() != 1 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let id = trimmed
        .parse::<usize>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    if storage::items().get(id).is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    storage::write(|state| state.inventory_ids.push(id));
    Ok(StatusCode::OK)
}

pub(crate) async fn questions() -> Result<Json<&'static Vec<storage::QA>>, StatusCode> {
    Ok(Json(storage::questions()))
}

pub(crate) async fn current_question() -> Result<Json<usize>, StatusCode> {
    let current = storage::read(|state| state.question_number);
    Ok(Json::from(current))
}
