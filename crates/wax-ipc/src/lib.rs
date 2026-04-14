use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub fn socket_path() -> PathBuf {
    std::env::var("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
        .join("wax.sock")
}

#[derive(Serialize, Deserialize)]
pub enum Request {
    Get { n: usize },
    Delete { text: String },
    Clear,
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Clips(Vec<String>),
    Ok,
    Error(String),
}
