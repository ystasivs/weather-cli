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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::argparser::ProviderName;
    use crate::configs::Config;
    use std::collections::HashMap;

    fn make_test_config(default_provider: Option<ProviderName>, configured_provider: Option<(ProviderName, ProviderConfig)>) -> Config {
        let mut providers = HashMap::new();
        if let Some((name, config)) = configured_provider {
            providers.insert(name, config);
        }
        Config {
            default_provider,
            providers,
        }
    }

    #[test]
    fn test_build_provider_open_weather_success() {
        let config = make_test_config(
            Some(ProviderName::OpenWeather),
            Some((ProviderName::OpenWeather, ProviderConfig::OpenWeather { api_key: "ow_key".to_string() }))
        );
        let result = ProviderBuilder::build_provider(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_build_provider_weather_api_success() {
        let config = make_test_config(
            Some(ProviderName::WeatherApi),
            Some((ProviderName::WeatherApi, ProviderConfig::WeatherApi { api_key: "wa_key".to_string() }))
        );
        let result = ProviderBuilder::build_provider(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_build_provider_no_default_provider() {
        let config = make_test_config(
            None,
            Some((ProviderName::WeatherApi, ProviderConfig::WeatherApi { api_key: "wa_key".to_string() }))
        );
        let result = ProviderBuilder::build_provider(config);
        assert!(matches!(result, Err(ProviderBuilderError::NoDefaultProvider)));
    }

    #[test]
    fn test_build_provider_not_configured() {
        // Default is OpenWeather, but only WeatherApi is configured
        let config = make_test_config(
            Some(ProviderName::OpenWeather),
            Some((ProviderName::WeatherApi, ProviderConfig::WeatherApi { api_key: "wa_key".to_string() }))
        );
        let result = ProviderBuilder::build_provider(config);
        assert!(matches!(result, Err(ProviderBuilderError::ProvidersIsNotConfigured)));
    }
}