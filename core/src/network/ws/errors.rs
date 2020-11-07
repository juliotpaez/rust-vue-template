use std::error::Error;
use std::fmt::Display;

use serde::export::Formatter;

#[derive(Debug)]
pub struct ClientDisconnectedError {}

impl Display for ClientDisconnectedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Client disconnected")
    }
}

impl Error for ClientDisconnectedError {}