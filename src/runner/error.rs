use std::error::Error;
use std::fmt;
use std::num::ParseFloatError;

pub type RunnerErrorResult<T> = Result<T, RunnerError>;

#[derive(Debug)]
pub struct RunnerError {
    pub message: Option<String>,
}

impl Default for RunnerError {
    fn default() -> Self {
        Self { message: None }
    }
}

impl RunnerError {
    pub fn new(message: String) -> Self {
        Self {
            message: Some(message),
        }
    }
}

impl<T> Into<RunnerErrorResult<T>> for RunnerError {
    fn into(self) -> RunnerErrorResult<T> {
        Err(self)
    }
}

impl From<std::io::Error> for RunnerError {
    fn from(_: std::io::Error) -> Self {
        RunnerError::default()
    }
}

impl From<&mut std::io::Error> for RunnerError {
    fn from(_: &mut std::io::Error) -> Self {
        RunnerError::default()
    }
}

impl From<std::str::Utf8Error> for RunnerError {
    fn from(_: std::str::Utf8Error) -> Self {
        RunnerError::default()
    }
}

impl From<ParseFloatError> for RunnerError {
    fn from(_: ParseFloatError) -> Self {
        RunnerError::new("parse float error".to_string())
    }
}

impl fmt::Display for RunnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(message) = &self.message {
            write!(f, "[RunnerError]! {}", message)
        } else {
            write!(f, "Runner error without a message")
        }
    }
}

impl Error for RunnerError {}
