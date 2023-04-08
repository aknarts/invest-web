pub fn calculate_rate(value: f64, earning: f64) -> f64 {
    (earning * 12.0) / value
}

pub fn calculate_pa_earnings(value: f64, earning: f64) -> Option<String> {
    if value.le(&0.1) || earning.le(&0.1) {
        None
    } else {
        Some(format!("{:.1}", (calculate_rate(value, earning) * 100.0)))
    }
}

pub fn calculate_total_earnings(
    value: f64,
    earning: f64,
    maturity: Option<time::Date>,
    expiration: Option<time::Date>,
) -> Option<String> {
    if value.le(&0.1) || earning.le(&0.1) {
        None
    } else {
        if maturity.is_none() || expiration.is_none() {
            None
        } else {
            #[allow(clippy::cast_precision_loss)]
            Some(format!(
                "{:.1}",
                (calculate_rate(value, earning)
                    * (maturity.unwrap() - expiration.unwrap()).whole_days() as f64
                    / 365.0)
                    * 100.0
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rate() {
        let rate = calculate_rate(100.0, 10.0);
        assert_eq!(rate, 1.2)
    }

    #[test]
    fn pa_rate() {
        let rate = calculate_pa_earnings(100.0, 10.0);
        assert_eq!(rate, Some("120.0".to_string()))
    }

    #[test]
    fn pa_rate_invalid() {
        let rate = calculate_pa_earnings(0.0, 10.0);
        assert_eq!(rate, None)
    }

    #[test]
    fn pa_rate_invalid2() {
        let rate = calculate_pa_earnings(10.0, 0.0);
        assert_eq!(rate, None)
    }

    #[test]
    fn total_rate() {
        let rate = calculate_total_earnings(
            100.0,
            10.0,
            Some(
                time::Date::from_calendar_date(2024, time::Month::April, 1)
                    .unwrap_or(time::Date::MIN),
            ),
            Some(
                time::Date::from_calendar_date(2023, time::Month::April, 1)
                    .unwrap_or(time::Date::MIN),
            ),
        );
        assert_eq!(rate, Some("120.3".to_string()))
    }

    #[test]
    fn total_rate_invalid() {
        let rate = calculate_total_earnings(
            0.0,
            10.0,
            Some(
                time::Date::from_calendar_date(2023, time::Month::April, 1)
                    .unwrap_or(time::Date::MIN),
            ),
            Some(
                time::Date::from_calendar_date(2023, time::Month::April, 1)
                    .unwrap_or(time::Date::MIN),
            ),
        );
        assert_eq!(rate, None)
    }

    #[test]
    fn total_rate_invalid2() {
        let rate = calculate_total_earnings(
            100.0,
            0.0,
            Some(
                time::Date::from_calendar_date(2023, time::Month::April, 1)
                    .unwrap_or(time::Date::MIN),
            ),
            Some(
                time::Date::from_calendar_date(2023, time::Month::April, 1)
                    .unwrap_or(time::Date::MIN),
            ),
        );
        assert_eq!(rate, None)
    }

    #[test]
    fn total_rate_invalid3() {
        let rate = calculate_total_earnings(
            100.0,
            10.0,
            None,
            Some(
                time::Date::from_calendar_date(2023, time::Month::April, 1)
                    .unwrap_or(time::Date::MIN),
            ),
        );
        assert_eq!(rate, None)
    }

    #[test]
    fn total_rate_invalid4() {
        let rate = calculate_total_earnings(
            100.0,
            10.0,
            Some(
                time::Date::from_calendar_date(2023, time::Month::April, 1)
                    .unwrap_or(time::Date::MIN),
            ),
            None,
        );
        assert_eq!(rate, None)
    }
}
