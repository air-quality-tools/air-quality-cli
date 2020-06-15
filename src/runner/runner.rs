use crate::runner::bluetooth::restart_bluetooth;
use crate::runner::parser::parse_raw_sensor_data;
use crate::runner::python_dependency::create_python_file;
use crate::shared::types::sensor_data::SensorData;
use crate::utils::timeout;
use chrono::prelude::*;
use log::info;
use std::error::Error;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::num::ParseFloatError;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::Stdio;
use std::thread::sleep;
use std::time::Duration;

const READER_SAMPLE_PERIOD_IN_SECONDS: u32 = 300;

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

pub fn fetch_sensor_data(
    python_executable_path: &Path,
    serial_number: &u32,
) -> Result<SensorData, RunnerError> {
    let sensor_data_raw = generate_sensor_data_retry(python_executable_path, serial_number)?;
    let time_now = chrono::Utc::now();

    parse_raw_sensor_data(time_now, &sensor_data_raw).map_err(|_| {
        RunnerError::new(format!(
            "failed to parse raw sensor data: {}",
            &sensor_data_raw
        ))
    })
}

pub struct Runner {
    output_dir_path: PathBuf,
    python_executable: tempfile::NamedTempFile,
    device_serial_number: u32,
}

impl Runner {
    pub fn new(output_dir_path: PathBuf, serial_number: u32) -> RunnerErrorResult<Runner> {
        let python_executable = create_python_file()?;

        Ok(Runner {
            output_dir_path,
            python_executable,
            device_serial_number: serial_number,
        })
    }

    fn python_executable_path(&self) -> &Path {
        &self.python_executable.path()
    }

    pub fn run(&self) -> RunnerErrorResult<()> {
        info!(
            "Running Airthings sensor data for devices with serial number: {:?}",
            self.device_serial_number
        );
        info!(
            "Sensor data will be added to dir: {:?}",
            self.output_dir_path
        );
        let device_serial_number = self.device_serial_number;
        loop {
            let sensor_data_raw =
                fetch_sensor_data(self.python_executable_path(), &device_serial_number);

            if let Err(error) = &sensor_data_raw {
                eprintln!(
                    "[{}][serial number: {}] Could not fetch sensor data. Is bluetooth enabled/on? Error: {:?}",
                    chrono::Utc::now(),
                    device_serial_number,
                    error
                );
            }

            if let Ok(sensor_data) = sensor_data_raw {
                println!(
                    "[serial number: {}] {}",
                    device_serial_number,
                    sensor_data.to_csv()
                );
                self.create_or_append_sensor_data_file(sensor_data, device_serial_number)?;
            }
            sleep(Duration::from_secs(60 * 5))
        }
    }

    fn create_or_append_sensor_data_file(
        &self,
        sensor_data: SensorData,
        device_serial_number: u32,
    ) -> RunnerErrorResult<()> {
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
}

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

    return smol::run(async {
        let output = timeout(Duration::from_secs(10), async {
            reader_process.wait_with_output()
        })
        .await?;

        let output_str = std::str::from_utf8(&output.stdout)?;

        if output_str.lines().count() <= 5 {
            return Err(RunnerError::new(
                "Failed to get input from the python script".to_string(),
            ));
        }

        Ok(output_str.to_string())
    });

    // let output = reader_process.wait_with_output()?;
    // let output_str = std::str::from_utf8(&output.stdout)?;
    //
    // if output_str.lines().count() <= 5 {
    //     return Err(RunnerError::new(
    //         "Failed to get input from the python script".to_string(),
    //     ));
    // }
    //
    // Ok(output_str.to_string())
}

fn generate_sensor_data_retry(
    python_executable_path: &Path,
    serial_number: &u32,
) -> Result<String, RunnerError> {
    let max_error_passes: u8 = 3;

    for error_pass in 0..=max_error_passes {
        if error_pass > 0 {
            eprintln!("Error when generating sensor data. Probably Bluetooth related so restarting the bluetooth service and trying again. ");
            restart_bluetooth().unwrap();
            eprintln!("Will wait for {} seconds", error_pass);
            sleep(Duration::from_secs(error_pass as u64))
        }

        let generated_sensor_data = generate_sensor_data(python_executable_path, serial_number);

        if let Ok(sensor_data) = generated_sensor_data {
            return Ok(sensor_data);
        } else {
            eprintln!(
                "[GENERATE SENSOR DATA ERROR] {:#?}",
                generated_sensor_data.err()
            )
        }
    }

    Err(RunnerError::new(format!(
        "Failed to generate sensor data. Stopped after {}",
        max_error_passes
    )))
}
