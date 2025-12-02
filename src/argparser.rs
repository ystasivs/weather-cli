use clap::{Parser, Subcommand, ValueEnum};
use chrono::NaiveDate;
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
    pub latitude: Option<f64>,

    /// Longitude (required if toponym not provided)
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
    WeatherApi
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