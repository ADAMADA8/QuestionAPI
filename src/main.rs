mod handlers;
mod services;

use crate::services::app_router;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::sync::OnceLock;
use tokio::fs;

#[derive(Debug, Deserialize)]
struct Config {
    host: String,
    port: String,
}

#[derive(Debug, Deserialize)]
pub struct QA {
    pub question: String,
    pub answer: String,
}
static QUESTIONS: OnceLock<Vec<QA>> = OnceLock::new();
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let content = fs::read_to_string("config.yml").await.with_context(|| "Ошибка чтения config.yml")?;
    let config: Config = serde_saphyr::from_str(content.as_str()).with_context(|| "config.yml неверно оформлен")?;
    println!("{:#?}", config);

    let app = app_router();

    let address = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&address).await?;

    let content = fs::read_to_string("questions.yml").await.with_context(|| "Ошибка чтения questions.yml")?;
    let parsed: Vec<QA> = serde_saphyr::from_str(content.as_str()).with_context(|| "questions.yml неверно оформлен")?;

    QUESTIONS.set(parsed).unwrap();
    for qa in QUESTIONS.get().unwrap() {
        println!("{} {}", qa.question, qa.answer);
    }

    println!("Веб сервер запущен на {}", address);
    axum::serve(listener, app).await?;
    Ok(())
}
