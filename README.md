# Weather CLI

A command-line tool for fetching weather information using multiple weather providers. Get current weather and forecasts by city name or coordinates with intelligent geocoding.

## Features

- 🌍 **Multiple Weather Providers**: Support for OpenWeather and WeatherAPI
- 📍 **Flexible Location Input**: Search by city name or exact coordinates
- 🗺️ **Smart Geocoding**: Automatically resolves city names to coordinates with disambiguation
- 📅 **Date-based Queries**: Get weather for specific dates (current or historical)
- ⚙️ **Configurable**: Easy provider selection and API key management
- 🌡️ **Metric Units**: Temperature in Celsius, wind speed in m/s

## Installation

### Prerequisites

- Rust 2024 edition or later
- API keys for your chosen weather provider(s):
    - [OpenWeather API](https://openweathermap.org/api)
    - [WeatherAPI](https://www.weatherapi.com/)

### Build from Source

```bash
git clone <your-repo-url>
cd weather-cli
cargo build --release
```

The binary will be available at `target/release/weather`.

## Configuration

Before using the CLI, you need to configure at least one weather provider with your API key.

### Configure a Provider

```bash
weather configure <PROVIDER>
```

Available providers:
- `open-weather` - OpenWeather API
- `weather-api` - WeatherAPI

Example:
```bash
weather configure weather-api
# You'll be prompted to enter your API key
```

### Select Default Provider

```bash
weather select <PROVIDER>
```

Example:
```bash
weather select weather-api
```

## Usage

### Basic Weather Query

Get weather for a city:
```bash
weather "New York"
```

Get weather with a country code for disambiguation:
```bash
weather "Paris" --country-code FR
```

### Query by Coordinates

```bash
weather --latitude 40.7128 --longitude -74.0060
```

### Query for Specific Date

```bash
# Future forecast
weather "London" 2024-12-25

# Historical data (if supported by provider)
weather "Tokyo" 2024-11-01
```

### Examples

```bash
# Current weather in Tokyo
weather Tokyo

# Weather in Paris, France for Christmas
weather Paris --country-code FR 2025-12-25

# Weather at specific coordinates
weather --latitude 51.5074 --longitude -0.1278

# Weather in London for today (explicit date)
weather London 2024-12-02
```

## How It Works

### Geocoding

When you provide a city name, the CLI uses the Open-Meteo Geocoding API to resolve it to coordinates. If multiple matches are found:

1. It first looks for cities with population ≥ 100,000
2. If no large city matches, it displays all results and prompts you to select one
3. Results include country, administrative regions, and population for easy identification

### Weather Providers

The CLI supports multiple weather providers through a unified interface:

- **OpenWeather**: Uses the One Call API 3.0 for comprehensive weather data
- **WeatherAPI**: Supports both forecast and historical weather data

Each provider is configured separately, and you can switch between them using the `select` command.

## Project Structure

```
weather-cli/
├── src/
│   ├── main.rs              # Entry point
│   ├── cli.rs               # Core CLI logic
│   ├── argparser.rs         # Command-line argument parsing
│   ├── configs.rs           # Configuration management
│   ├── geocoder.rs          # Location resolution
│   ├── input.rs             # User input handling
│   ├── provider_builder.rs # Provider instantiation
│   ├── providers.rs         # Provider module exports
│   └── providers/
│       ├── provider_trait.rs    # Weather provider trait
│       ├── open_weather.rs      # OpenWeather implementation
│       ├── weather_api.rs       # WeatherAPI implementation
│       ├── models.rs            # API response models
│       ├── providers_report.rs  # Unified weather report
│       └── error.rs             # Provider error types
├── Cargo.toml
└── README.md
```

## Dependencies

- **clap**: Command-line argument parsing with derive macros
- **reqwest**: HTTP client with blocking API and JSON support
- **serde**: Serialization/deserialization framework
- **serde_json**: JSON support for serde
- **chrono**: Date and time handling
- **thiserror**: Error type derivation
- **urlencoding**: URL encoding for API requests
- **toml**: Configuration file parsing

## Error Handling

The CLI provides clear error messages for common issues:

- Missing location information
- Invalid API keys
- Network connectivity problems
- Date out of range for the provider
- Ambiguous location names

## Development

### Running Tests

```bash
cargo test
```

### Building for Development

```bash
cargo build
./target/debug/weather "Berlin"
```

### Adding a New Weather Provider

1. Implement the `WeatherProvider` trait in a new file under `src/providers/`
2. Add the provider to `ProviderName` enum in `argparser.rs`
3. Update `ProviderBuilder` to instantiate your provider
4. Add API response models in `providers/models.rs`

## License

[Your License Here]

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Author

Yaroslav Stasiv

## Acknowledgments

- [Open-Meteo](https://open-meteo.com/) for the free geocoding API
- [OpenWeather](https://openweathermap.org/) for weather data
- [WeatherAPI](https://www.weatherapi.com/) for weather data