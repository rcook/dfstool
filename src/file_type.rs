use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum FileType {
    TokenizedBasic,
    Unknown,
}
