use serde::{Deserialize, Serialize};
use std::sync::{Arc, LazyLock, RwLock};

const STATE_PATH: &str = "state.json";

#[derive(Serialize, Deserialize, Clone, Default)]
pub(crate) struct SessionState {
    pub session_id: Arc<str>,
    pub question_number: usize,
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
