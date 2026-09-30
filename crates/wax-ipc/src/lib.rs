use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Where the socket lands when `XDG_RUNTIME_DIR` is unset. World-writable,
/// so a socket placed here is reachable by other local users.
const FALLBACK_RUNTIME_DIR: &str = "/tmp";

const SOCKET_NAME: &str = "wax.sock";

/// Resolve the socket path from an explicit runtime directory.
///
/// Split out from [`socket_path`] so the resolution rule can be tested without
/// mutating the process environment, which is process-global and racy across
/// parallel tests.
fn socket_path_in(runtime_dir: Option<&str>) -> PathBuf {
    runtime_dir
        .filter(|dir| !dir.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(FALLBACK_RUNTIME_DIR))
        .join(SOCKET_NAME)
}

pub fn socket_path() -> PathBuf {
    socket_path_in(std::env::var("XDG_RUNTIME_DIR").ok().as_deref())
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Request {
    Get { n: usize },
    Delete { text: String },
    Pin { text: String },
    Unpin { text: String },
    GetPinned,
    Clear,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Response {
    Clips(Vec<String>),
    Ok,
    Error(String),
}

#[cfg(test)]
mod tests;
