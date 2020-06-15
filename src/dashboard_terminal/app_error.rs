use std::io;
use std::io::Error;
use tui::Terminal;

pub type AppErrorResult<T> = Result<T, AppError>;

pub struct AppError {}

impl<T> Into<AppErrorResult<T>> for AppError {
    fn into(self) -> AppErrorResult<T> {
        Err(self)
    }
}

impl From<Error> for AppError {
    fn from(_: Error) -> Self {
        Self {}
    }
}
