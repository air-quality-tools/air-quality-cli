use crate::runner::bluetooth::restart_bluetooth;
use crate::runner::parser::parse_raw_sensor_data;
use crate::runner::python_dependency::create_python_file;
use crate::shared::types::sensor_data::SensorData;
use chrono::prelude::*;
use log::info;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::Stdio;
use std::thread::sleep;
use std::time::Duration;
use std::{thread, time};
use tempfile::NamedTempFile;

const READER_SAMPLE_PERIOD_IN_SECONDS: u32 = 300;

#[derive(Debug)]
pub struct RunnerError;

impl From<std::io::Error> for RunnerError {
    fn from(_: std::io::Error) -> Self {
        RunnerError
    }
}

impl From<std::str::Utf8Error> for RunnerError {
    fn from(_: std::str::Utf8Error) -> Self {
        RunnerError
    }
}

pub async fn fetch_sensor_data(
    python_executable_path: &Path,
    serial_number: &u32,
) -> Result<SensorData, RunnerError> {
    let sensor_data_raw = generate_sensor_data(python_executable_path, serial_number)?;
    let time_now = chrono::Utc::now();
    Ok(parse_raw_sensor_data(time_now, &sensor_data_raw))
}

pub fn fetch_sensor_data_sync(
    python_executable_path: &Path,
    serial_number: &u32,
) -> Result<SensorData, RunnerError> {
    let sensor_data_raw = generate_sensor_data_retry(python_executable_path, serial_number)?;
    let time_now = chrono::Utc::now();
    Ok(parse_raw_sensor_data(time_now, &sensor_data_raw))
}

pub struct Runner {
    output_dir_path: PathBuf,
    python_executable: tempfile::NamedTempFile,
    devices_serial_number: Vec<u32>,
}

impl Runner {
    pub fn new(output_dir_path: PathBuf, serial_number: u32) -> Result<Self, Box<dyn Error>> {
        let python_executable = create_python_file()?;

        Ok(Runner {
            output_dir_path,
            python_executable,
            devices_serial_number: vec![serial_number],
        })
    }

    fn python_executable_path(&self) -> &Path {
        &self.python_executable.path()
    }

    pub fn run(&self) {
        info!(
            "Running Airthings sensor data for devices with serial number: {:?}",
            self.devices_serial_number
        );
        info!(
            "Sensor data will be added to dir: {:?}",
            self.output_dir_path
        );
        loop {
            self.devices_serial_number
                .iter()
                .for_each(|device_serial_number| {
                    let sensor_data_raw =
                        fetch_sensor_data_sync(self.python_executable_path(), device_serial_number);

                    if let Err(error) = &sensor_data_raw {
                        eprintln!(
                            "[{}][serial number: {}] Could not fetch sensor data. Is bluetooth enabled/on? Error: {:?}",
                            chrono::Utc::now(),
                            device_serial_number,
                            error
                        );
                    }

                    if let Ok(sensor_data) = sensor_data_raw {
                        println!("[serial number: {}] {}", device_serial_number, sensor_data.to_csv());
                        self.create_or_append_sensor_data_file(sensor_data, device_serial_number);
                    }
                });
            sleep(Duration::from_secs(60 * 5))
        }
    }

    fn create_or_append_sensor_data_file(
        &self,
        sensor_data: SensorData,
        device_serial_number: &u32,
    ) {
        let filename_date_formatted = sensor_data.timestamp().format("%Y-%m-%d");
        let string = format!(
            "waveplus_data_sn_{}_{}.txt",
            device_serial_number, filename_date_formatted
        );
        let filepath = self.output_dir_path.join(&string);

        let mut append_file = OpenOptions::new()
            .create_new(false)
            .append(true)
            .open(filepath.clone());

        match append_file.as_mut() {
            Ok(file) => {
                file.write(format!("{}\n", sensor_data.to_csv()).as_bytes());
            }
            Err(error) if error.kind() == ErrorKind::NotFound => {
                OpenOptions::new()
                    .create_new(true)
                    .write(true)
                    .open(filepath)
                    .as_mut()
                    .map(|file| {
                        file.write(
                            format!("{}\n", sensor_data.to_csv_with_header(device_serial_number))
                                .as_bytes(),
                        )
                    });
            }
            Err(_) => {}
        };
    }
}

fn append_data_to_file(filepath: &Path, data: String) {}

fn generate_sensor_data(
    python_executable_path: &Path,
    serial_number: &u32,
) -> Result<String, RunnerError> {
    let mut reader_process = Command::new("sudo")
        // .stdin(Stdio::null())
        .stdout(Stdio::piped())
        // .stderr(Stdio::inherit())
        // .env_clear() // TODO: LANG=en_US.UTF-8, PYTHONIOENCODING=utf-8
        .arg("-k")
        .arg("--")
        .arg("python")
        .arg(python_executable_path)
        // .arg("./read_waveplus.py")
        .arg(serial_number.to_string())
        .arg(READER_SAMPLE_PERIOD_IN_SECONDS.to_string())
        .spawn()?;

    let output = reader_process.wait_with_output()?;
    let output_str = std::str::from_utf8(&output.stdout)?;

    if output_str.lines().count() <= 5 {
        return Err(RunnerError);
    }

    Ok(output_str.to_string())
}

fn generate_sensor_data_retry(
    python_executable_path: &Path,
    serial_number: &u32,
) -> Result<String, RunnerError> {
    if let Ok(sensor_data) = generate_sensor_data(python_executable_path, serial_number) {
        Ok(sensor_data)
    } else {
        eprint!("Error when generating sensor data. Probably Bluetooth related so restarting the bluetooth service and trying again. ");
        restart_bluetooth().unwrap();
        generate_sensor_data(python_executable_path, serial_number)
    }
}
