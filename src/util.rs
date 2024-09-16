use std::{error::Error, result::Result as StdRes};

pub type Err = Box<dyn Error + Send + Sync>;
pub type Result<T> = StdRes<T, Err>;
