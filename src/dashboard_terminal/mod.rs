use std::io;
use tui::backend::CrosstermBackend;

mod app;
mod app_error;
mod domain;
mod widgets;

use crate::dashboard_terminal::app::App;
use crate::dashboard_terminal::app_error::AppErrorResult;
use std::error::Error;
use std::path::PathBuf;

pub fn start_gui(data_dir_path: PathBuf) -> AppErrorResult<()> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    App::new(backend, data_dir_path)?.run()?;

    Ok(())
}
