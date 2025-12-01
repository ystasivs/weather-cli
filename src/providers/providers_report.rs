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
            Wind          : {:.1} kph ({})",
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
        let day = value.forecast.forecastday.get(0)
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