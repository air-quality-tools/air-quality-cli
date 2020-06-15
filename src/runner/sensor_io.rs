use crate::runner::bluetooth::restart_bluetooth;
use crate::runner::error::RunnerError;
use crate::runner::parser::parse_raw_sensor_data;
use crate::shared::types::sensor_data::SensorData;
use std::path::Path;
use std::process::{Command, Stdio};
use std::str::from_utf8;
use std::thread::sleep;
use std::time::Duration;

const READER_SAMPLE_PERIOD_IN_SECONDS: u32 = 300;

pub fn fetch_sensor_data(
    python_executable_path: &Path,
    serial_number: u32,
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

fn generate_sensor_data_retry(
    python_executable_path: &Path,
    serial_number: u32,
) -> Result<String, RunnerError> {
    let max_error_passes: u8 = 3;

    for error_pass in 0..=max_error_passes {
        if error_pass > 0 {
            eprintln!("Error when generating sensor data. Probably Bluetooth related so restarting the bluetooth service and trying again. ");
            restart_bluetooth().unwrap();
            eprintln!("Will wait for {} seconds", error_pass);
            sleep(Duration::from_secs(error_pass as u64))
        }

        let generated_sensor_data_raw =
            generate_sensor_data_raw(python_executable_path, serial_number);

        if let Ok(sensor_data_raw) = generated_sensor_data_raw {
            return Ok(sensor_data_raw);
        } else {
            eprintln!(
                "[GENERATE SENSOR DATA ERROR] {:#?}",
                generated_sensor_data_raw.err()
            )
        }
    }

    Err(RunnerError::new(format!(
        "Failed to generate sensor data. Stopped after {}",
        max_error_passes
    )))
}

fn generate_sensor_data_raw(
    python_executable_path: &Path,
    serial_number: u32,
) -> Result<String, RunnerError> {
    let reader_process = Command::new("sudo")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("-k")
        .arg("--")
        .arg("python")
        .arg(python_executable_path)
        .arg(serial_number.to_string())
        .arg(READER_SAMPLE_PERIOD_IN_SECONDS.to_string())
        .spawn()?;

    let output = reader_process.wait_with_output()?;

    if output.status.success() == false {
        return Err(RunnerError::new(
            "Failed to get input from the python script".to_string(),
        ));
    }

    let output_str = from_utf8(&output.stdout)?;

    Ok(output_str.to_owned())
}
