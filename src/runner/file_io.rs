use crate::runner::error::RunnerErrorResult;
use crate::shared::types::sensor_data::SensorData;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;

pub fn create_or_append_sensor_data_file(
    filepath: &PathBuf,
    sensor_data: SensorData,
    device_serial_number: u32,
) -> RunnerErrorResult<()> {
    let mut append_file = OpenOptions::new()
        .create_new(false)
        .append(true)
        .open(filepath);

    match append_file.as_mut() {
        Ok(file) => {
            file.write_all(format!("{}\n", sensor_data.to_csv()).as_bytes())?;
        }
        Err(error) if error.kind() == ErrorKind::NotFound => {
            OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(filepath)
                .as_mut()
                .map(|file| {
                    file.write_all(
                        format!("{}\n", sensor_data.to_csv_with_header(device_serial_number))
                            .as_bytes(),
                    )
                })??;
        }
        Err(_) => {}
    };

    Ok(())
}
