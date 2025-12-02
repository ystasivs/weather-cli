use weather::configs::{self, Config, ProviderConfig};
use weather::argparser::ProviderName;
use std::fs;
use std::path::Path;

// NOTE: This constant is hardcoded in src/configs.rs
// For proper integration testing, ideally this should be set to a temporary file,
// but since the original constant is private, we must use the same path for tests.
const CONFIG_FILE_NAME: &str = "/tmp/weather_cli_config.toml";

// Helper function to ensure a clean state before and after each test
fn cleanup_config_file() {
    if Path::new(CONFIG_FILE_NAME).exists() {
        fs::remove_file(CONFIG_FILE_NAME).unwrap();
    }
}

// Mock input functions for testing set_config_for_provider
// NOTE: This requires modifying src/configs.rs and src/input.rs slightly to
// allow mock input (e.g., using a mockable function/trait for read_user_string).
// Since we cannot modify the internal implementation here, we will *simulate*
// the process by checking the final state of the file.

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

    // 1. Select a provider
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

// NOTE: To properly test `set_config_for_provider`, the `read_user_string` function
// from `src/input.rs` must be mocked or replaced.
// The test below can only verify the file structure.

/*
#[test]
fn test_set_config_for_provider() {
    // This test is highly dependent on mocking the `read_user_string` function,
    // which is not trivial for a simple test environment.
    // Assuming `read_user_string` is mocked to return "MOCK_KEY" for API key:

    // 1. Configure OpenWeather (also sets it as default since it's the first)
    // Mock user input: "MOCK_OW_KEY"
    configs::set_config_for_provider(&ProviderName::OpenWeather);

    let config = Config::load();
    assert_eq!(config.default_provider, Some(ProviderName::OpenWeather));
    if let Some(ProviderConfig::OpenWeather { api_key }) = config.providers.get(&ProviderName::OpenWeather) {
        assert_eq!(api_key, "MOCK_OW_KEY");
    } else {
        panic!("OpenWeather config not found or wrong type");
    }

    // 2. Configure WeatherApi (should NOT change the default provider)
    // Mock user input: "MOCK_WA_KEY"
    configs::set_config_for_provider(&ProviderName::WeatherApi);

    let config = Config::load();
    assert_eq!(config.default_provider, Some(ProviderName::OpenWeather)); // Still OpenWeather
    if let Some(ProviderConfig::WeatherApi { api_key }) = config.providers.get(&ProviderName::WeatherApi) {
        assert_eq!(api_key, "MOCK_WA_KEY");
    } else {
        panic!("WeatherApi config not found or wrong type");
    }

    cleanup_config_file();
}
*/