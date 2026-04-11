use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, LazyLock, OnceLock, RwLock};
use tokio::fs;

const STATE_PATH: &str = "state.json";

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct QA {
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct Item {
    pub name: String,
    pub description: String,
    pub image: String,
}

static QUESTIONS: OnceLock<Vec<QA>> = OnceLock::new();
static ITEMS: OnceLock<Vec<Item>> = OnceLock::new();

#[derive(Serialize, Deserialize, Clone, Default)]
pub(crate) struct SessionState {
    pub session_id: Arc<str>,
    pub question_number: usize,
    #[serde(default)]
    pub inventory_ids: Vec<usize>,
}

static STATE: LazyLock<RwLock<SessionState>> = LazyLock::new(|| {
    let session = std::fs::read(STATE_PATH)
        .ok()
        .and_then(|data| serde_json::from_slice(&data).ok())
        .unwrap_or_default();
    RwLock::new(session)
});

pub(crate) fn read<F, R>(f: F) -> R
where
    F: FnOnce(&SessionState) -> R,
{
    f(&STATE.read().unwrap())
}

pub(crate) fn write<F>(f: F)
where
    F: FnOnce(&mut SessionState),
{
    f(&mut STATE.write().unwrap());
    persist();
}

fn persist() {
    let data = serde_json::to_vec(&*STATE.read().unwrap()).unwrap();
    let tmp = format!("{}.tmp", STATE_PATH);
    std::fs::write(&tmp, data).unwrap();
    std::fs::rename(&tmp, STATE_PATH).unwrap();
}

pub(crate) async fn init_questions() -> Result<usize> {
    let parsed: Vec<QA> = load_yaml_vec(
        "questions.yml",
        "Ошибка чтения questions.yml",
        "questions.yml неверно оформлен",
    )
    .await?;

    let count = parsed.len();
    QUESTIONS.set(parsed).expect("QUESTIONS уже инициализирован");
    Ok(count)
}

pub(crate) async fn init_items() -> Result<usize> {
    let parsed: Vec<Item> = load_yaml_vec(
        "items.yml",
        "Ошибка чтения items.yml",
        "items.yml неверно оформлен",
    )
    .await?;

    let count = parsed.len();
    ITEMS.set(parsed).expect("ITEMS уже инициализирован");
    Ok(count)
}

pub(crate) fn questions() -> &'static Vec<QA> {
    QUESTIONS.get().expect("QUESTIONS не инициализирован")
}

pub(crate) fn items() -> &'static Vec<Item> {
    ITEMS.get().expect("ITEMS не инициализирован")
}

pub(crate) fn inventory() -> Vec<Item> {
    read(|state| {
        state
            .inventory_ids
            .iter()
            .filter_map(|id| items().get(*id).cloned())
            .collect()
    })
}

async fn load_yaml_vec<T>(path: &str, read_ctx: &str, parse_ctx: &str) -> Result<Vec<T>>
where
    T: DeserializeOwned,
{
    let content = fs::read_to_string(path)
        .await
        .with_context(|| read_ctx.to_string())?;

    let parsed = serde_saphyr::from_str(content.as_str())
        .with_context(|| parse_ctx.to_string())?;

    Ok(parsed)
}

