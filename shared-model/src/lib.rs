use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileWriterArgs {
    pub content: String,
    pub should_error: bool,
}
