use std::fmt;

use super::models::{weather_api::WeatherApiReport, openweather_api::OpenWeatherDaily};
use super::error::ProviderError;

pub struct ProvidersReport {
    min_temp: f32,
    max_temp: f32,
    avg_temp: f32,
    pop: f32,
    summary: String,
    wind_speed: f32,
    humidity: f32,
    wind_direction: Option<i32>,
}

impl fmt::Display for ProvidersReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let wind_dir_str = match self.wind_direction {
            Some(deg) => format!("{}°", deg),
            None => "N/A".to_string(),
        };

        write!(
            f,
            "Weather Report:\n\
            Summary       : {}\n\
            Temperature   : min {:.1}°C | max {:.1}°C | avg {:.1}°C\n\
            Precipitation : {:.1}%\n\
            Humidity      : {:.1}%\n\
            Wind          : {:.1} m/sec ({})",
            self.summary,
            self.min_temp,
            self.max_temp,
            self.avg_temp,
            self.pop,
            self.humidity,
            self.wind_speed,
            wind_dir_str
        )
    }
}
impl TryFrom<WeatherApiReport> for ProvidersReport {
    type Error = ProviderError;
    fn try_from(value: WeatherApiReport) -> Result<Self, Self::Error> {
        let day = value.forecast.forecastday.first()
            .ok_or(ProviderError::ConvertionError("weather api".to_string(), "failed to get forecast".to_string()))?;
        Ok(Self{
            max_temp: day.day.maxtemp_c,
            min_temp: day.day.mintemp_c,
            avg_temp: day.day.avgtemp_c,
            pop: day.day.daily_chance_of_rain,
            summary: day.day.condition.text.clone(),
            wind_speed: day.day.maxwind_kph * 1000. / 3600., // km/h -> m/sec
            wind_direction: None,
            humidity: day.day.avghumidity,
        })
    }
}

impl TryFrom<OpenWeatherDaily> for ProvidersReport {
    type Error = ProviderError;
    fn try_from(value: OpenWeatherDaily) -> Result<Self, Self::Error> {
        Ok(Self{
            max_temp: value.temp.max,
            min_temp: value.temp.min,
            avg_temp: value.temp.day,
            pop: value.pop,
            summary: value.summary,
            wind_speed: value.wind_speed,
            humidity: value.humidity,
            wind_direction: Some(value.wind_deg),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::models::{weather_api::*, openweather_api::*};

    #[test]
    fn test_report_display_with_wind_direction() {
        let report = ProvidersReport {
            min_temp: 10.0,
            max_temp: 20.0,
            avg_temp: 15.0,
            pop: 50.5,
            summary: "Sunny".to_string(),
            wind_speed: 5.5,
            humidity: 70.0,
            wind_direction: Some(180),
        };
        let expected = "\
Weather Report:\n\
Summary       : Sunny\n\
Temperature   : min 10.0°C | max 20.0°C | avg 15.0°C\n\
Precipitation : 50.5%\n\
Humidity      : 70.0%\n\
Wind          : 5.5 m/sec (180°)";
        assert_eq!(format!("{}", report), expected);
    }

    #[test]
    fn test_try_from_weather_api_report() {
        let api_report = WeatherApiReport {
            forecast: WeatherApiForecast {
                forecastday: vec![WeatherApiForecastDay {
                    day: WeatherApiDay {
                        maxtemp_c: 25.0,
                        mintemp_c: 15.0,
                        avgtemp_c: 20.0,
                        maxwind_kph: 36.0, // 36 kph = 10 m/sec
                        avghumidity: 60.0,
                        daily_chance_of_rain: 10.0,
                        condition: WeatherApiCondition {
                            text: "Partly cloudy".to_string(),
                        },
                    },
                }],
            },
        };

        let report = ProvidersReport::try_from(api_report).unwrap();
        assert_eq!(report.max_temp, 25.0);
        assert_eq!(report.min_temp, 15.0);
        assert_eq!(report.avg_temp, 20.0);
        // Test kph to m/sec conversion: 36 kph * 1000 / 3600 = 10 m/sec
        assert!((report.wind_speed - 10.0).abs() < 0.001);
        assert_eq!(report.summary, "Partly cloudy");
    }

    #[test]
    fn test_try_from_open_weather_daily() {
        let ow_daily = OpenWeatherDaily {
            dt: 1672531200, // Example timestamp
            summary: "Clear sky".to_string(),
            temp: OpeWeatherTemperature {
                day: 18.0,
                min: 12.0,
                max: 22.0,
            },
            humidity: 65.0,
            wind_speed: 7.5,
            wind_deg: 90,
            pop: 0.1,
        };

        let report = ProvidersReport::try_from(ow_daily).unwrap();
        assert_eq!(report.max_temp, 22.0);
        assert_eq!(report.min_temp, 12.0);
        assert_eq!(report.avg_temp, 18.0);
        assert!((report.pop - 0.1).abs() < 0.001);
        assert_eq!(report.summary, "Clear sky");
        assert_eq!(report.wind_speed, 7.5);
        assert_eq!(report.wind_direction, Some(90));
    }
}