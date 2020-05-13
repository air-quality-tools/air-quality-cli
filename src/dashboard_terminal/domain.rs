use crate::shared::types::sensor_data;
use crate::shared::types::sensor_data::SensorData;
use std::fs::{DirEntry, OpenOptions};
use std::io::Read;
use std::path::PathBuf;
use std::{fs, io};

pub fn read_latest_sensor_data_from_directory(data_dir: &PathBuf) -> Option<SensorData> {
    let mut paths: Vec<_> = data_dir
        .read_dir()
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();

    paths.sort();
    paths.reverse();

    paths
        .first()
        .map(|filepath| {
            OpenOptions::new()
                .create_new(false)
                .read(true)
                .open(filepath)
                .as_mut()
                .map(|file| {
                    let mut sensor_data_raw = "".to_string();
                    file.read_to_string(&mut sensor_data_raw);

                    sensor_data::latest_entry_from_file(&sensor_data_raw)
                })
                .unwrap()
        })
        .flatten()
}
