use std::error::Error;

pub type Err = Box<dyn Error>;
pub type Res<T> = Result<T, Err>;
