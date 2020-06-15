use crate::runner::runner::{RunnerError, RunnerErrorResult};
use crate::shared::types::sensor_data;
use crate::shared::types::sensor_data::SensorData;
use std::fs::{DirEntry, OpenOptions};
use std::io::Read;
use std::path::PathBuf;

pub fn read_latest_sensor_data_from_directory(data_dir: &PathBuf) -> RunnerErrorResult<SensorData> {
    let mut paths: Vec<_> = data_dir
        .read_dir()
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();

    paths.sort();
    paths.reverse();

    let filepath = paths
        .first()
        .ok_or_else(|| RunnerError::new("Latest sensor data file not found".to_owned()))?;

    let mut file = OpenOptions::new()
        .create_new(false)
        .read(true)
        .open(filepath)?;

    let mut sensor_data_raw = "".to_string();
    file.read_to_string(&mut sensor_data_raw)?;

    let sensor_data = sensor_data::latest_entry_from_file(&sensor_data_raw).ok_or_else(|| {
        RunnerError::new("Latest sensor data not found or parsed (file found)".to_owned())
    })?;

    Ok(sensor_data)
}
