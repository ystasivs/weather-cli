use chrono::{DateTime, NaiveDate};
use reqwest::StatusCode;
use reqwest::blocking::get;

use super::error::ProviderError;
use super::models::openweather_api::OpenWeatherReport;
use super::provider_trait::WeatherProvider;
use super::{ProviderResult, providers_report::ProvidersReport};

pub struct OpenWeather {
    api_key: String,
}

impl OpenWeather {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    fn prepare_url(&self, lat: f64, lon: f64) -> String {
        format!(
            "https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&appid={}&exclude=hourly,minutely&units=metric",
            lat, lon, self.api_key
        )
    }
}

impl WeatherProvider for OpenWeather {
    fn get_weather(
        &self,
        latitude: f64,
        longitude: f64,
        date: NaiveDate,
    ) -> ProviderResult<ProvidersReport> {
        let url = self.prepare_url(latitude, longitude);
        let response = get(url).map_err(|e| ProviderError::RequestFailed(e.to_string()))?;
        let status = response.status();

        match status {
            StatusCode::OK => {
                let resp_body: OpenWeatherReport = response
                    .json()
                    .map_err(|e| ProviderError::ParseError(e.to_string()))?;
                let day = resp_body
                    .daily
                    .into_iter()
                    .find(|el| {
                        let dt = DateTime::from_timestamp(el.dt, 0).unwrap_or_default();
                        let naive = dt.naive_local();
                        date == naive.date()
                    })
                    .ok_or(ProviderError::DateIsOutOfRange("open weather".to_string()))?;
                ProvidersReport::try_from(day)
            }
            _ => Err(ProviderError::ProviderMsgError(
                response
                    .text()
                    .unwrap_or("failed to extract text".to_string()),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_url() {
        let provider = OpenWeather::new("test_key".to_string());
        let url = provider.prepare_url(40.71, -74.01);
        let expected = "https://api.openweathermap.org/data/3.0/onecall?lat=40.71&lon=-74.01&appid=test_key&exclude=hourly,minutely&units=metric";
        assert_eq!(url, expected);
    }
}
