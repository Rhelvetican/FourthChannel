use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Post {
    pub id: i64,
    pub hash: Vec<u8>,
    pub content: PostContent,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostContent {
    pub msg: String,
    pub media: Option<Vec<u8>>,
}
