use anyhow::Context;
use serde::Deserialize;
use std::sync::LazyLock;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) host: String,
    pub(crate) port: String,
    pub(crate) admin_token: String,
}

pub(crate) static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let content = std::fs::read_to_string("config.yml")
        .with_context(|| "Не удалось прочитать config.yml")
        .expect("config.yml не найден. Проверьте, что файл существует в рабочей директории");
    serde_saphyr::from_str(&content)
        .with_context(|| "config.yml неверно оформлен")
        .expect("Не удалось распарсить config.yml. Проверьте структуру файла")
});
