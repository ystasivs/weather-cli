use thiserror::Error;

use crate::configs::{Config, ProviderConfig};
use crate::providers::WeatherProvider;
use crate::providers::{open_weather::OpenWeather, weather_api::WeatherApi};
#[derive(Error, Debug)]
pub enum ProviderBuilderError {
    #[error("no provider selected. read --help")]
    NoDefaultProvider,
    #[error("no config for provider. read --help")]
    ProvidersIsNotConfigured
}

pub struct ProviderBuilder;

impl ProviderBuilder {
    pub fn build_provider(config: Config) -> Result<Box<dyn WeatherProvider>, ProviderBuilderError> {
        let provider = config.default_provider.ok_or(ProviderBuilderError::NoDefaultProvider)?;
        let config = config.providers.get(&provider).ok_or(ProviderBuilderError::ProvidersIsNotConfigured)?;
        match config {
            ProviderConfig::OpenWeather { api_key} => Ok(Box::new(OpenWeather::new(api_key.clone()))),
            ProviderConfig::WeatherApi { api_key} => Ok(Box::new(WeatherApi::new(api_key.clone()))),
        }
    }
}