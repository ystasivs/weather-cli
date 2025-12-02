use chrono::{Local, NaiveDate};
use reqwest::StatusCode;
use reqwest::blocking::Client;
use reqwest::header::ACCEPT;
use urlencoding::encode;

use super::error::{ProviderError, ProviderResult};
use super::models::weather_api::{WeatherApiForecastError, WeatherApiReport};
use super::provider_trait::WeatherProvider;
use super::providers_report::ProvidersReport;
pub struct WeatherApi {
    api_key: String,
}

impl WeatherApi {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
    fn prepare_url(&self, latitude: f64, longitude: f64, date: NaiveDate) -> String {
        let location = format!("{latitude},{longitude}");
        let location_encoded = encode(&location);
        let endpoint = if date >= Local::now().naive_local().date() {
            "forecast.json"
        } else {
            "history.json"
        };
        format!(
            "https://api.weatherapi.com/v1/{}?q={}&days=1&dt={}&key={}",
            endpoint, location_encoded, date, self.api_key
        )
    }
}

impl WeatherProvider for WeatherApi {
    fn get_weather(
        &self,
        latitude: f64,
        longitude: f64,
        date: NaiveDate,
    ) -> ProviderResult<ProvidersReport> {
        let client = Client::new();
        let response = client
            .get(self.prepare_url(latitude, longitude, date))
            .header(ACCEPT, "application/json")
            .send()
            .map_err(|err| ProviderError::RequestFailed(err.to_string()))?;
        match response.status() {
            StatusCode::OK => {
                let body: WeatherApiReport = response
                    .json()
                    .map_err(|err| ProviderError::ParseError(err.to_string()))?;
                ProvidersReport::try_from(body)
            }
            _ => {
                let res: WeatherApiForecastError = response
                    .json()
                    .map_err(|err| ProviderError::ParseError(err.to_string()))?;
                Err(ProviderError::ProviderMsgError(res.message))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_url_for_future_date_uses_forecast() {
        let provider = WeatherApi::new("test_key".to_string());
        let future_date = NaiveDate::from_ymd_opt(9999, 1, 1).unwrap();
        let url = provider.prepare_url(51.5, -0.1, future_date);
        assert!(url.contains("forecast.json"));
        assert!(url.contains("q=51.5%2C-0.1"));
        assert!(url.contains("dt=9999-01-01"));
    }

    #[test]
    fn test_prepare_url_for_historical_date_uses_history() {
        let provider = WeatherApi::new("test_key".to_string());
        // Use a date guaranteed to be in the past
        let past_date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        let url = provider.prepare_url(51.5, -0.1, past_date);
        assert!(url.contains("history.json"));
        assert!(url.contains("dt=2000-01-01"));
    }
}
