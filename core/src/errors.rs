use std::error::Error;
use std::fmt::Display;

use serde::export::Formatter;

#[derive(Debug)]
pub struct EmptyError {}

impl Display for EmptyError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Error for EmptyError {}