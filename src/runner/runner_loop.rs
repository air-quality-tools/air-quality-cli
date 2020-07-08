use crate::runner::error::RunnerErrorResult;
use crate::runner::file_io::create_or_append_sensor_data_file;
use crate::runner::python_dependency::create_python_file;
use crate::runner::sensor_io::fetch_sensor_data;
use crate::shared::types::sensor_data::SensorData;
use log::info;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;

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
                fetch_sensor_data(self.python_executable_path(), device_serial_number);

            if let Err(error) = &sensor_data_raw {
                eprintln!(
                    "[{}][serial number: {}] Could not fetch sensor data. Is bluetooth enabled/on? Error: {:?}",
                    chrono::Utc::now(),
                    device_serial_number,
                    error
                );
                return Err(error.clone());
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
        let filepath = &self.output_dir_path.join(&string);

        Ok(create_or_append_sensor_data_file(
            &filepath,
            sensor_data,
            device_serial_number,
        )?)
    }
}
