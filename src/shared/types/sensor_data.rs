use crate::shared::types::sensor_quality::SensorQuality;
use chrono::TimeZone;
use std::iter;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct SensorData {
    timestamp: chrono::DateTime<chrono::Utc>,
    temperature_in_celsius: f32,
    humidity_in_percent: f32,
    atmospheric_pressure: f32,
    co2: f32,
    voc: f32,
    radon_short_term_average: f32,
    radon_long_term_average: f32,
}

impl SensorData {
    pub fn new(
        timestamp: chrono::DateTime<chrono::Utc>,
        temperature_in_celsius: f32,
        humidity_in_percent: f32,
        atmospheric_pressure: f32,
        co2: f32,
        voc: f32,
        radon_short_term_average: f32,
        radon_long_term_average: f32,
    ) -> SensorData {
        SensorData {
            timestamp,
            temperature_in_celsius,
            humidity_in_percent,
            atmospheric_pressure,
            co2,
            voc,
            radon_short_term_average,
            radon_long_term_average,
        }
    }
}

/// Getters
impl SensorData {
    pub fn timestamp(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.timestamp
    }

    pub fn temperature_in_celsius(&self) -> f32 {
        self.temperature_in_celsius
    }

    pub fn humidity_in_percent(&self) -> f32 {
        self.humidity_in_percent
    }

    pub fn atmospheric_pressure(&self) -> f32 {
        self.atmospheric_pressure
    }

    pub fn co2(&self) -> f32 {
        self.co2
    }

    pub fn voc(&self) -> f32 {
        self.voc
    }

    pub fn radon_short_term_average(&self) -> f32 {
        self.radon_short_term_average
    }

    pub fn radon_long_term_average(&self) -> f32 {
        self.radon_long_term_average
    }
}

/// Quality
impl SensorData {
    pub fn temperature_quality(&self) -> SensorQuality {
        SensorQuality::temperature_quality(self.temperature_in_celsius.round() as i32)
    }

    pub fn humidity_quality(&self) -> SensorQuality {
        SensorQuality::humidity_quality(self.humidity_in_percent.round() as u32)
    }

    pub fn atmospheric_pressure_quality(&self) -> SensorQuality {
        SensorQuality::atmospheric_pressure_quality(self.atmospheric_pressure.round() as u32)
    }

    pub fn co2_quality(&self) -> SensorQuality {
        SensorQuality::co2_quality(self.co2.round() as u32)
    }

    pub fn voc_quality(&self) -> SensorQuality {
        SensorQuality::voc_quality(self.voc.round() as u32)
    }

    pub fn radon_short_term_quality(&self) -> SensorQuality {
        SensorQuality::radon_quality(self.radon_short_term_average.round() as u32)
    }

    pub fn radon_long_term_quality(&self) -> SensorQuality {
        SensorQuality::radon_quality(self.radon_long_term_average.round() as u32)
    }

    pub fn worst_sensor_quality(&self) -> SensorQuality {
        let list = vec![
            self.temperature_quality(),
            self.radon_short_term_quality(),
            self.voc_quality(),
            self.humidity_quality(),
            self.co2_quality(),
            self.atmospheric_pressure_quality(),
        ];

        SensorQuality::worst_sensor_quality(list)
    }
}

/// CSV functions
impl SensorData {
    pub fn to_csv(&self) -> String {
        let elements: [String; 9] = [
            format!("{}", self.timestamp),
            self.temperature_in_celsius.to_string(),
            self.humidity_in_percent.to_string(),
            self.atmospheric_pressure.to_string(),
            self.co2.to_string(),
            self.voc.to_string(),
            self.radon_short_term_average.to_string(),
            self.radon_long_term_average.to_string(),
            "".to_string(),
        ];

        elements.join(",").to_string()
    }

    pub fn to_csv_with_header(&self, device_serial_number: &u32) -> String {
        [csv_header(device_serial_number), self.to_csv()]
            .join("\n")
            .to_string()
    }

    pub fn from_csv_line(csv_line: &str) -> Option<Self> {
        let mut splitted_data = csv_line.split(",");
        let timestamp: chrono::DateTime<chrono::Utc> =
            chrono::DateTime::from_str(splitted_data.next().unwrap()).unwrap();

        Some(Self {
            timestamp,
            temperature_in_celsius: splitted_data.next().unwrap().parse::<f32>().unwrap(),
            humidity_in_percent: splitted_data.next().unwrap().parse::<f32>().unwrap(),
            atmospheric_pressure: splitted_data.next().unwrap().parse::<f32>().unwrap(),
            co2: splitted_data.next().unwrap().parse::<f32>().unwrap(),
            voc: splitted_data.next().unwrap().parse::<f32>().unwrap(),
            radon_short_term_average: splitted_data.next().unwrap().parse::<f32>().unwrap(),
            radon_long_term_average: splitted_data.next().unwrap().parse::<f32>().unwrap(),
        })
    }
}

pub fn csv_header(device_serial_number: &u32) -> String {
    [
        "Timestamp",
        "Temperature (C)",
        "Humidity (%)",
        "Atmospheric pressure (mbar)",
        "CO2 (ppm)",
        "TVOC (ppb)",
        "Radon short-term average (Bq/m3)",
        "Radon long-term average (Bq/m3)",
        device_serial_number.to_string().as_str(),
    ]
    .join(",")
    .to_string()
}

pub fn latest_entry_from_file(raw_data_file: &str) -> Option<SensorData> {
    raw_data_file
        .lines()
        .last()
        .map(|line| SensorData::from_csv_line(line))
        .flatten()
}
