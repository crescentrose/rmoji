use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct RmojiError {
    details: String
}

impl RmojiError {
   pub fn new(details: &str) -> RmojiError {
       RmojiError { details: details.to_string() }
   }
}

impl fmt::Display for RmojiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for RmojiError {
    fn description(&self) -> &str {
        &self.details
    }
}
