use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::result::Result as StdResult;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StartSessionError {
    Unauthorized,
    AlreadyStarted,
    Internal,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub(crate) struct SessionState {
    pub session_id: Arc<str>,
    #[serde(default)]
    pub pin_code: Arc<str>,
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
    let guard = match STATE.read() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    f(&guard)
}

pub(crate) fn write<F>(f: F) -> Result<()>
where
    F: FnOnce(&mut SessionState),
{
    let mut guard = match STATE.write() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    f(&mut guard);
    persist_locked(&guard)
}

fn persist_locked(state: &SessionState) -> Result<()> {
    let data = serde_json::to_vec(state).with_context(|| "Не удалось сериализовать state.json")?;
    let tmp = format!("{}.tmp", STATE_PATH);

    std::fs::write(&tmp, data).with_context(|| format!("Не удалось записать временный файл {tmp}"))?;
    std::fs::rename(&tmp, STATE_PATH)
        .with_context(|| format!("Не удалось заменить {STATE_PATH} временным файлом"))?;

    Ok(())
}

pub(crate) fn start_session(pin: &str, session_id: &str) -> StdResult<(), StartSessionError> {
    let mut guard = match STATE.write() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    if guard.pin_code.is_empty() || guard.pin_code.as_ref() != pin {
        return Err(StartSessionError::Unauthorized);
    }

    if !guard.session_id.is_empty() {
        return Err(StartSessionError::AlreadyStarted);
    }

    guard.session_id = Arc::from(session_id);
    guard.question_number = 0;
    guard.inventory_ids.clear();

    persist_locked(&guard).map_err(|_| StartSessionError::Internal)
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

