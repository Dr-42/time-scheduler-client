use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    Client(String),
    Server(String),
    LoginExpired,
}
