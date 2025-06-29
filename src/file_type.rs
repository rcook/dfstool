use serde::{Deserialize, Serialize};

// Ideally this would be MIME type
// Unfortunately there is no official MIME type for BBC BASIC
// text/plain is not appropriate, since these files are _not_
// text
// https://www.riscosopen.org/wiki/documentation/show/File%20Types
#[derive(Debug, Deserialize, Serialize)]
pub enum KnownFileType {
    #[serde(rename = "bbc-basic", alias = "TokenizedBasic")]
    BbcBasic,
    #[serde(rename = "other", alias = "Unknown")]
    Other,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FileType {
    Known(KnownFileType),
    Unknown(String),
}
