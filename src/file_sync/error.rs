use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

pub type SynchronizeRunnerErrorResult<T> = Result<T, SynchronizeRunnerError>;

#[derive(Debug)]
pub struct SynchronizeRunnerError {
    message: Option<String>,
}

impl SynchronizeRunnerError {
    pub fn new(message: String) -> Self {
        Self {
            message: Some(message),
        }
    }
}

impl Default for SynchronizeRunnerError {
    fn default() -> Self {
        Self { message: None }
    }
}

impl fmt::Display for SynchronizeRunnerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(message) = &self.message {
            write!(f, "[FileSyncError] {}", message)
        } else {
            write!(
                f,
                "[FileSyncError] Synchronize runner error without a message"
            )
        }
    }
}

impl Error for SynchronizeRunnerError {}

impl From<std::io::Error> for SynchronizeRunnerError {
    fn from(err: std::io::Error) -> Self {
        Self::new(err.to_string())
    }
}

impl From<ssh2::Error> for SynchronizeRunnerError {
    fn from(err: ssh2::Error) -> Self {
        Self::new(err.to_string())
    }
}
