use crate::runner::error::RunnerErrorResult;
use std::path::PathBuf;

mod bluetooth;
pub mod error;
mod file_io;
mod parser;
mod python_dependency;
pub mod runner_loop;
mod sensor_io;

pub fn start_data_generator(data_dir_path: PathBuf, serial_number: u32) -> RunnerErrorResult<()> {
    Ok(run(data_dir_path, serial_number)?)
}

pub fn run(data_dir_path: PathBuf, serial_number: u32) -> RunnerErrorResult<()> {
    let runner = runner_loop::Runner::new(data_dir_path, serial_number)?;
    runner.run()?;
    Ok(())
}
