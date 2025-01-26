pub mod meta;
pub mod pallete;
pub mod remote_iface;
pub mod sun;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    Client(String),
    Server(String),
    LoginExpired,
}
