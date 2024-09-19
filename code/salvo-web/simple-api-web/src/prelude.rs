pub use crate::error::Error;

pub use salvo::prelude::*;

pub type Result<T> = std::result::Result<T, Error>;
