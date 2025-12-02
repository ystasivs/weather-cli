use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs;
use crate::input::read_user_string;
use crate::argparser::ProviderName;

const CONFIG_FILE_NAME: &str = "/tmp/weather_cli_config.toml";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub default_provider: Option<ProviderName>,
    pub providers: HashMap<ProviderName, ProviderConfig>,
}

impl Config {
    pub fn load() -> Self {
        fs::read_to_string(CONFIG_FILE_NAME)
            .map(|contents| toml::from_str::<Self>(&contents).unwrap_or_default())
            .unwrap_or_default()
    }
    fn dump(&self) {
        fs::write(CONFIG_FILE_NAME, toml::to_string(self).unwrap_or_default()).unwrap();
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ProviderConfig {
    #[serde(rename = "weatherapi")]
    WeatherApi {
        api_key: String,
    },
    #[serde(rename = "openweather")]
    OpenWeather {
        api_key: String,
    },
}

pub fn set_config_for_provider(provider_name: &ProviderName) {
    let mut config = Config::load();
    match provider_name {
        ProviderName::OpenWeather => {
            println!("Please provide api_key for OpenWeather");
            let api_key = read_user_string();
            config.providers
                .entry(provider_name.clone())
                .and_modify(|cfg| {
                    if let ProviderConfig::OpenWeather { api_key: existing_key } = cfg {
                        *existing_key = api_key.clone();
                    }
                })
                .or_insert(ProviderConfig::OpenWeather { api_key });
        },
        ProviderName::WeatherApi => {
            println!("Please provide WeatherApi");
            let api_key = read_user_string();
            config.providers
                .entry(provider_name.clone())
                .and_modify(|cfg| {
                    if let ProviderConfig::WeatherApi { api_key: existing_key } = cfg {
                        *existing_key = api_key.clone();
                    }
                })
                .or_insert(ProviderConfig::WeatherApi { api_key });
        }
    }
    if config.default_provider.is_none() {
        config.default_provider = Some(provider_name.clone());
    }
    config.dump();
}

pub fn select_default_provider(provider_name: &ProviderName) {
    let mut config = Config::load();
    config.default_provider = Some(provider_name.clone());
    config.dump();
}
