use reqwest::blocking::Client;
use reqwest::header::ACCEPT;
use reqwest::StatusCode;
use serde::Deserialize;

use super::error::{ProviderError, ProviderResult};
struct WeatherApi{
    api_key: String,
}

impl WeatherApi{
    fn new(api_key: String) -> Self {
        Self {
            api_key
        }
    }

    fn prepare_url(&self, location: &str, date: &str) -> String {
        format!("https://api.weatherapi.com/v1/forecast.json?q={}&days=1&dt={}&key={}", location, date, self.api_key)
    }

    pub fn get_weather(self, location: &str, date: &str) -> ProviderResult<WeatherApiReport> {
        let client = Client::new();
        let response = client.get(self.prepare_url(location, date))
            .header(ACCEPT, "application/json")
            .send().map_err(|err| ProviderError::RequestFailed(err.to_string()))?;
        match response.status(){
            StatusCode::OK => {
                response.json()
                    .map_err(|err| ProviderError::ParseError(err.to_string()))
            }
            _ => {
                let res: WeatherApiForecastError = response.json()
                    .map_err(|err| ProviderError::ParseError(err.to_string()))?;
                Err(ProviderError::ProviderMsgError(res.message))
            }
        }
    }
}

#[derive(Deserialize, Debug)]
struct WeatherApiReport {
    location: WeatherApiReportLocation,
    forecast: WeatherApiForecast
}

#[derive(Deserialize, Debug)]
struct WeatherApiForecast{
    forecastday: Vec<WeatherApiForecastDay>
}

#[derive(Deserialize, Debug)]
struct WeatherApiReportLocation {
    name: String,
    region: String,
    country: String,
    localtime: String,
}

#[derive(Deserialize, Debug)]
struct WeatherApiForecastDay{
    date: String,
    day: WeatherApiDay
}

#[derive(Deserialize, Debug)]
struct WeatherApiDay{
    maxtemp_c: f32,
    mintemp_c: f32,
    avgtemp_c: f32,
    maxwind_kph: f32,
    totalprecip_mm: f32,
    avghumidity: f32,
    maxvind_kph: f32,
    daily_will_it_snow: f32,
    daily_chance_of_rain: f32,
    condition: WeatherApiCondition
}

#[derive(Deserialize, Debug)]
struct WeatherApiCondition {
    text: String,
    icon: String,
    code: i32
}

#[derive(Deserialize, Debug)]
struct WeatherApiForecastError {
    message: String,
    code: i32
}
//todo Add astro