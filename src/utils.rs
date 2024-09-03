use std::{error::Error as StdErr, result::Result as StdRes};

pub type Error = Box<dyn StdErr + Send + Sync>;
pub type Result<T> = StdRes<T, Error>;
