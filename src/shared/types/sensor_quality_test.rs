#[cfg(test)]
mod tests {
    use crate::shared::types::sensor_quality::SensorQuality;
    use std::str::from_utf8;

    #[test]
    fn temperature_quality() {
        let terrible_low = SensorQuality::temperature_quality(26);
        let terrible_high = SensorQuality::temperature_quality(i32::MAX);

        let good_low = SensorQuality::temperature_quality(18);
        let good_high = SensorQuality::temperature_quality(25);

        let depends_low = SensorQuality::temperature_quality(i32::MIN);
        let depends_high = SensorQuality::temperature_quality(17);

        assert_eq!(terrible_low, SensorQuality::Terrible);
        assert_eq!(terrible_high, SensorQuality::Terrible);
        assert_eq!(good_low, SensorQuality::Good);
        assert_eq!(good_high, SensorQuality::Good);
        assert_eq!(depends_low, SensorQuality::DependsOnContext);
        assert_eq!(depends_high, SensorQuality::DependsOnContext);
    }

    #[test]
    fn atmospheric_pressure_quality() {
        let depends_low = SensorQuality::atmospheric_pressure_quality(u32::MIN);
        let depends_high = SensorQuality::atmospheric_pressure_quality(u32::MAX);

        assert_eq!(depends_low, SensorQuality::DependsOnContext);
        assert_eq!(depends_high, SensorQuality::DependsOnContext);
    }

    #[test]
    fn co2_quality() {
        let terrible_low = SensorQuality::co2_quality(1000);
        let terrible_high = SensorQuality::co2_quality(u32::MAX);

        let bad_low = SensorQuality::co2_quality(800);
        let bad_high = SensorQuality::co2_quality(999);

        let good_low = SensorQuality::co2_quality(u32::MIN);
        let good_high = SensorQuality::co2_quality(799);

        assert_eq!(terrible_low, SensorQuality::Terrible);
        assert_eq!(terrible_high, SensorQuality::Terrible);

        assert_eq!(bad_low, SensorQuality::Bad);
        assert_eq!(bad_high, SensorQuality::Bad);

        assert_eq!(good_low, SensorQuality::Good);
        assert_eq!(good_high, SensorQuality::Good);
    }

    #[test]
    fn voc_quality() {
        let terrible_low = SensorQuality::voc_quality(2000);
        let terrible_high = SensorQuality::voc_quality(u32::MAX);

        let bad_low = SensorQuality::voc_quality(250);
        let bad_high = SensorQuality::voc_quality(1999);

        let good_low = SensorQuality::voc_quality(249);
        let good_high = SensorQuality::voc_quality(149);

        assert_eq!(terrible_low, SensorQuality::Terrible);
        assert_eq!(terrible_high, SensorQuality::Terrible);
        assert_eq!(bad_low, SensorQuality::Bad);
        assert_eq!(bad_high, SensorQuality::Bad);
        assert_eq!(good_low, SensorQuality::Good);
        assert_eq!(good_high, SensorQuality::Good);
    }

    #[test]
    fn humidity_quality() {
        let terrible_low_a = SensorQuality::humidity_quality(70);
        let terrible_high_a = SensorQuality::humidity_quality(u32::MAX);

        let bad_low_a = SensorQuality::humidity_quality(60);
        let bad_high_a = SensorQuality::humidity_quality(69);

        let good_low = SensorQuality::humidity_quality(30);
        let good_high = SensorQuality::humidity_quality(59);

        let bad_low_b = SensorQuality::humidity_quality(25);
        let bad_high_b = SensorQuality::humidity_quality(29);

        let terrible_low_b = SensorQuality::humidity_quality(u32::MIN);
        let terrible_high_b = SensorQuality::humidity_quality(24);

        assert_eq!(terrible_low_a, SensorQuality::Terrible);
        assert_eq!(terrible_high_a, SensorQuality::Terrible);

        assert_eq!(terrible_low_b, SensorQuality::Terrible);
        assert_eq!(terrible_high_b, SensorQuality::Terrible);

        assert_eq!(bad_low_a, SensorQuality::Bad);
        assert_eq!(bad_high_a, SensorQuality::Bad);

        assert_eq!(bad_low_b, SensorQuality::Bad);
        assert_eq!(bad_high_b, SensorQuality::Bad);

        assert_eq!(good_low, SensorQuality::Good);
        assert_eq!(good_high, SensorQuality::Good);
    }

    #[test]
    fn radon_quality() {
        let terrible_low = SensorQuality::radon_quality(150);
        let terrible_high = SensorQuality::radon_quality(u32::MAX);

        let bad_low = SensorQuality::radon_quality(100);
        let bad_high = SensorQuality::radon_quality(149);

        let good_low = SensorQuality::radon_quality(u32::MIN);
        let good_high = SensorQuality::radon_quality(99);

        assert_eq!(terrible_low, SensorQuality::Terrible);
        assert_eq!(terrible_high, SensorQuality::Terrible);

        assert_eq!(bad_low, SensorQuality::Bad);
        assert_eq!(bad_high, SensorQuality::Bad);

        assert_eq!(good_low, SensorQuality::Good);
        assert_eq!(good_high, SensorQuality::Good);
    }
}
