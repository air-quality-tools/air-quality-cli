use crate::runner::runner::RunnerError;
use crate::shared::types::sensor_data::{SensorData, SensorDataBuilder};
use std::num::ParseFloatError;

const RAW_DATA_SENSOR_INDEX: u32 = 5;

pub fn parse_raw_sensor_data(
    timestamp: chrono::DateTime<chrono::Utc>,
    raw_data: &str,
) -> Result<SensorData, RunnerError> {
    let line: &str = raw_data
        .lines()
        .nth(RAW_DATA_SENSOR_INDEX as usize)
        .ok_or_else(|| {
            RunnerError::new("Failed to read stdin data from Python reader script".to_owned())
        })?;

    let mut properties = line
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',');

    let property_error_message = format!("property error. Input line: {}", &line);

    let humidity_in_percent: f32 = trim_property(
        properties
            .next()
            .ok_or_else(|| RunnerError::new(property_error_message.clone()))?,
        " %rH",
    )?;
    let radon_short_term_average: f32 = trim_property(
        properties
            .next()
            .ok_or_else(|| RunnerError::new(property_error_message.clone()))?,
        " Bq/m3",
    )?;
    let radon_long_term_average: f32 = trim_property(
        properties
            .next()
            .ok_or_else(|| RunnerError::new(property_error_message.clone()))?,
        " Bq/m3",
    )?;
    let temperature_in_celsius: f32 = trim_property(
        properties
            .next()
            .ok_or_else(|| RunnerError::new(property_error_message.clone()))?,
        " degC",
    )?;
    let atmospheric_pressure: f32 = trim_property(
        properties
            .next()
            .ok_or_else(|| RunnerError::new(property_error_message.clone()))?,
        " hPa",
    )?;
    let co2: f32 = trim_property(
        properties
            .next()
            .ok_or_else(|| RunnerError::new(property_error_message.clone()))?,
        " ppm",
    )?;
    let voc: f32 = trim_property(
        properties
            .next()
            .ok_or_else(|| RunnerError::new(property_error_message.clone()))?,
        " ppb",
    )?;

    Ok(SensorDataBuilder {
        timestamp,
        temperature_in_celsius,
        humidity_in_percent,
        atmospheric_pressure,
        co2,
        voc,
        radon_short_term_average,
        radon_long_term_average,
    }
    .into())
}

fn trim_property(input: &str, remove_unit: &str) -> Result<f32, ParseFloatError> {
    input
        .trim()
        .trim_start_matches("'")
        .trim_end_matches("'")
        .trim_end_matches(remove_unit)
        .parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_raw_helper() {
        let raw_data = "\nPress ctrl+C to exit program\n\nDevice serial number: 2930027508\n[\'Humidity\', \'Radon ST avg\', \'Radon LT avg\', \'Temperature\', \'Pressure\', \'CO2 level\', \'VOC level\']\n[\'22.5 %rH\', \'1 Bq/m3\', \'6 Bq/m3\', \'22.58 degC\', \'1022.08 hPa\', \'476.0 ppm\', \'152.0 ppb\']\n";
        let sensor_data = parse_raw_sensor_data(chrono::Utc::now(), raw_data).unwrap();

        assert_eq!(sensor_data.humidity_in_percent().to_string(), "22.5");
        assert_eq!(sensor_data.radon_short_term_average().to_string(), "1");
        assert_eq!(sensor_data.radon_long_term_average().to_string(), "6");
        assert_eq!(sensor_data.temperature_in_celsius().to_string(), "22.58");
        assert_eq!(sensor_data.atmospheric_pressure().to_string(), "1022.08");
        assert_eq!(sensor_data.co2().to_string(), "476");
        assert_eq!(sensor_data.voc().to_string(), "152");
    }
}
