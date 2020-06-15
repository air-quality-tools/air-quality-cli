#[derive(PartialEq, Eq, Debug)]
pub enum SensorQuality {
    Good,
    Bad,
    Terrible,
    DependsOnContext,
}

impl SensorQuality {
    pub fn temperature_quality(value: i32) -> SensorQuality {
        match value {
            26..=i32::MAX => SensorQuality::Terrible,
            18..=25 => SensorQuality::Good,
            i32::MIN..=17 => SensorQuality::DependsOnContext,
        }
    }

    pub fn humidity_quality(value: u32) -> SensorQuality {
        match value {
            70..=u32::MAX => SensorQuality::Terrible,
            60..=69 => SensorQuality::Bad,
            30..=59 => SensorQuality::Good,
            25..=29 => SensorQuality::Bad,
            u32::MIN..=24 => SensorQuality::Terrible,
        }
    }

    pub fn atmospheric_pressure_quality(_value: u32) -> SensorQuality {
        SensorQuality::DependsOnContext
    }

    pub fn co2_quality(value: u32) -> SensorQuality {
        match value {
            1000..=u32::MAX => SensorQuality::Terrible,
            800..=999 => SensorQuality::Bad,
            u32::MIN..=799 => SensorQuality::Good,
        }
    }

    pub fn voc_quality(value: u32) -> SensorQuality {
        match value {
            2000..=u32::MAX => SensorQuality::Terrible,
            250..=1999 => SensorQuality::Bad,
            u32::MIN..=249 => SensorQuality::Good,
        }
    }

    pub fn radon_quality(value: u32) -> SensorQuality {
        match value {
            150..=u32::MAX => SensorQuality::Terrible,
            100..=149 => SensorQuality::Bad,
            u32::MIN..=99 => SensorQuality::Good,
        }
    }

    pub fn worst_sensor_quality(sensor_data_list: Vec<SensorQuality>) -> SensorQuality {
        if sensor_data_list
            .iter()
            .any(|quality| quality == &SensorQuality::Terrible)
        {
            SensorQuality::Terrible
        } else if sensor_data_list
            .iter()
            .any(|quality| quality == &SensorQuality::Bad)
        {
            SensorQuality::Bad
        } else {
            SensorQuality::Good
        }
    }
}
