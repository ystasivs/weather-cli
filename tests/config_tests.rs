use std::fs;
use std::path::Path;
use weather::argparser::ProviderName;
use weather::configs::{self, Config};

const CONFIG_FILE_NAME: &str = "/tmp/weather_cli_config.toml";

fn cleanup_config_file() {
    if Path::new(CONFIG_FILE_NAME).exists() {
        fs::remove_file(CONFIG_FILE_NAME).unwrap();
    }
}

#[test]
fn test_config_initial_load() {
    cleanup_config_file();
    let config = Config::load();
    assert!(config.default_provider.is_none());
    assert!(config.providers.is_empty());
}

#[test]
fn test_select_default_provider() {
    cleanup_config_file();
    configs::select_default_provider(&ProviderName::WeatherApi);

    // 2. Load the config and verify
    let config = Config::load();
    assert_eq!(config.default_provider, Some(ProviderName::WeatherApi));

    // 3. Select a different provider
    configs::select_default_provider(&ProviderName::OpenWeather);

    // 4. Load and verify the change
    let config = Config::load();
    assert_eq!(config.default_provider, Some(ProviderName::OpenWeather));

    cleanup_config_file();
}
