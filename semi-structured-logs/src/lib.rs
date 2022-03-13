// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.

#![allow(unused)]
use std::fmt;

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let cap_level: String = level.clone().to_string().to_uppercase();
    format!("[{}]: {}", cap_level, message)
}
pub fn info(message: &str) -> String {
    format!("[INFO]: {}", message)
}
pub fn warn(message: &str) -> String {
    format!("[WARNING]: {}", message)
}
pub fn error(message: &str) -> String {
    format!("[ERROR]: {}", message)
}
