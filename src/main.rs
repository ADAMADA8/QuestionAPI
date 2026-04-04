mod config;
mod handlers;
mod routes;
mod services;
mod storage;

use crate::config::CONFIG;
use crate::services::app_router;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use tokio::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct QA {
    pub question: String,
    pub answer: String,
}
static QUESTIONS: OnceLock<Vec<QA>> = OnceLock::new();
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let address = format!("{}:{}", CONFIG.host, CONFIG.port);
    let listener = tokio::net::TcpListener::bind(&address).await?;

    let content = fs::read_to_string("questions.yml")
        .await
        .with_context(|| "Ошибка чтения questions.yml")?;

    let parsed: Vec<QA> = serde_saphyr::from_str(content.as_str())
        .with_context(|| "questions.yml неверно оформлен")?;

    println!("Было обнаружено {} вопросов", parsed.len());
    QUESTIONS.set(parsed).unwrap();

    let app = app_router();
    println!("Веб сервер запущен на {}", address);
    axum::serve(listener, app).await?;
    Ok(())
}
