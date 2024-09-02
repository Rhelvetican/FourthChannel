use std::error::Error as StdErr;

pub type Error = Box<dyn StdErr + Send + Sync>;
pub type HandlerResult<T> = Result<T, Error>;
