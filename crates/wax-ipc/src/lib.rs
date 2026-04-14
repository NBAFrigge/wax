use serde::{Deserialize, Serialize};

pub const SOCKET_PATH: &str = "/tmp/wax.sock";

#[derive(Serialize, Deserialize)]
pub enum Request {
    Get { n: usize },
    Clear,
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Clips(Vec<String>),
    Ok,
    Error(String),
}

