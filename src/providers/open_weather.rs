use chrono::{DateTime, NaiveDate};
use reqwest::blocking::get;
use reqwest::StatusCode;

use super::error::ProviderError;
use super::{ProviderResult, providers_report::ProvidersReport};
use super::models::openweather_api::OpenWeatherReport;
use super::provider_trait::WeatherProvider;

pub struct OpenWeather {
    api_key: String
}

impl OpenWeather {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    fn prepare_url(&self, lat: f64, lon: f64) -> String {
        format!("https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&appid={}&exclude=hourly,minutely&units=metric", lat, lon, self.api_key)
    }
}

impl WeatherProvider for OpenWeather {
    fn get_weather(&self, latitude: f64, longitude: f64, date: NaiveDate) -> ProviderResult<ProvidersReport> {
        let url = self.prepare_url(latitude, longitude);
        let response = get(url).map_err(|e| ProviderError::RequestFailed(e.to_string()))?;
        let status = response.status();

        match status {
            StatusCode::OK => {
                let resp_body: OpenWeatherReport = response.json().map_err(|e| ProviderError::ParseError(e.to_string()))?;
                let day = resp_body.daily.into_iter().find(
                    |el| {
                        let dt = DateTime::from_timestamp(el.dt,0).unwrap_or_default();
                        let naive = dt.naive_local();
                        date == naive.date()
                    }
                ).ok_or(ProviderError::DateIsOutOfRange("open weather".to_string()))?;
                ProvidersReport::try_from(day)
            },
            _ => {
                Err(ProviderError::ProviderMsgError(response.text().unwrap_or("failed to extract text".to_string())))
            }
        }
    }
}
