use chrono::NaiveDate;
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fmt;

/// CLI for geocoding/weather
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Argparser {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Toponym (city, place name)
    pub toponym: Option<String>,

    /// Date (default today)
    #[arg(value_parser = parse_date,
         default_value_t = chrono::Local::now().naive_local().date())]
    pub date: NaiveDate,

    /// Optional country code
    #[arg(short, long)]
    pub country_code: Option<String>,

    /// Latitude (required if toponym not provided)
    #[arg(long)]
    pub latitude: Option<f64>,

    /// Longitude (required if toponym not provided)
    #[arg(long)]
    pub longitude: Option<f64>,
}

/// Subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Configure provider
    Configure {
        #[arg(value_enum)]
        provider_name: ProviderName,
    },
    /// Select provider (open-weather, weather-api)
    Select {
        #[arg(value_enum)]
        provider_name: ProviderName,
    },
}

/// Supported provider names
#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProviderName {
    OpenWeather,
    WeatherApi,
}

impl fmt::Display for ProviderName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ProviderName::WeatherApi => "WeatherApi",
            ProviderName::OpenWeather => "OpenWeather",
        };
        write!(f, "{}", name)
    }
}
/// Custom parser for date in YYYY-MM-DD format
fn parse_date(s: &str) -> Result<NaiveDate, chrono::format::ParseError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_valid() {
        let date = parse_date("2025-10-26").unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2025, 10, 26).unwrap());
    }

    #[test]
    fn test_parse_date_invalid_format() {
        let result = parse_date("26-10-2025");
        assert!(result.is_err());
    }

    #[test]
    fn test_provider_name_display() {
        assert_eq!(format!("{}", ProviderName::WeatherApi), "WeatherApi");
        assert_eq!(format!("{}", ProviderName::OpenWeather), "OpenWeather");
    }
}
