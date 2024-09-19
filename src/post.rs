use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Post {
    pub text: String,
    pub media: Option<Vec<u8>>,
}
