use reqwest::blocking::Client;
use reqwest::header::ACCEPT;
use reqwest::StatusCode;
use urlencoding::encode;
use chrono::{NaiveDate, Local};

use super::error::{ProviderError, ProviderResult};
use super::models::weather_api::{WeatherApiReport, WeatherApiForecastError};
use super::providers_report::ProvidersReport;
use super::provider_trait::WeatherProvider;
pub struct WeatherApi{
    api_key: String,
}

impl WeatherApi{
    pub fn new(api_key: String) -> Self {
        Self {
            api_key
        }
    }
    fn prepare_url(&self, latitude: f64, longitude: f64, date: NaiveDate) -> String {
        let location = format!("{latitude},{longitude}");
        let location_encoded = encode(&location);
        let endpoint = if date >= Local::now().naive_local().date() {
            "forecast.json"
        } else {
            "history.json"
        };
        format!("https://api.weatherapi.com/v1/{}?q={}&days=1&dt={}&key={}", endpoint, location_encoded, date, self.api_key)
    }
}

impl WeatherProvider for WeatherApi {
    fn get_weather(&self, latitude: f64, longitude: f64, date: NaiveDate) -> ProviderResult<ProvidersReport> {
        let client = Client::new();
        let response = client.get(self.prepare_url(latitude, longitude, date))
            .header(ACCEPT, "application/json")
            .send().map_err(|err| ProviderError::RequestFailed(err.to_string()))?;
        match response.status(){
            StatusCode::OK => {
                let body: WeatherApiReport = response.json()
                    .map_err(|err| ProviderError::ParseError(err.to_string()))?;
                ProvidersReport::try_from(body)
            }
            _ => {
                let res: WeatherApiForecastError = response.json()
                    .map_err(|err| ProviderError::ParseError(err.to_string()))?;
                Err(ProviderError::ProviderMsgError(res.message))
            }
        }
    }
}


