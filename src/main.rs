mod config;
mod handlers;
mod routes;
mod services;
mod storage;

use crate::config::CONFIG;
use crate::services::app_router;
use anyhow::Result;
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let address = format!("{}:{}", CONFIG.host, CONFIG.port);
    let listener = tokio::net::TcpListener::bind(&address).await?;

    let questions_count = storage::init_questions().await?;
    println!("Обнаружено {} вопросов", questions_count);

    let items_count = storage::init_items().await?;
    println!("Обнаружено {} предметов", items_count);

    let app = app_router();
    println!("Веб сервер запущен на {}", address);
    axum::serve(listener, app).await?;
    Ok(())
}
