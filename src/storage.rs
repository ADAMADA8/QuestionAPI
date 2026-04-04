use tokio::sync::Mutex;

pub(crate) static CURRENT_QUESTION: Mutex<u8> = Mutex::const_new(0);
