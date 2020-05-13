use std::io;
use tui::backend::CrosstermBackend;

mod app;
mod domain;
mod widgets;

use crate::dashboard_terminal::app::App;
use std::error::Error;
use std::path::PathBuf;

pub fn start_gui(data_dir_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    App::new(backend, data_dir_path)?.run()?;

    Ok(())
}
