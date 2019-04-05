use std::error;
use std::fmt;

use reqwest::Error as ReqwestError;

#[derive(Debug)]
pub enum CalendarError {
    Unknown,
    Connection(ReqwestError),
}

impl fmt::Display for CalendarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalendarError::Unknown => write!(f, "Unknown error"),
            CalendarError::Connection(error) => write!(f, "Connection error: {}", error),
        }
    }
}

impl error::Error for CalendarError {}
