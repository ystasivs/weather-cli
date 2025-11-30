use clap::{Parser, Subcommand, ValueEnum};
use chrono::NaiveDate;

/// CLI for geocoding/weather
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Argparser {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Toponym (city, place name)
    #[arg(required_unless_present_all(&["latitude", "longitude"]))]
    pub toponym: Option<String>,

    /// Optional country code
    #[arg(short, long)]
    pub country_code: Option<String>,

    /// Latitude (required if toponym not provided)
    #[arg(long, required_unless_present("toponym"))]
    pub latitude: Option<f64>,

    /// Longitude (required if toponym not provided)
    #[arg(long, required_unless_present("toponym"))]
    pub longitude: Option<f64>,

    /// Date (default today)
    #[arg(long, value_parser = parse_date, default_value_t = chrono::Local::today().naive_local())]
    pub date: NaiveDate,
}

/// Subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Configure provider
    Configure {
        #[arg(value_enum)]
        provider_name: ProviderName,
    },
    /// Select provider
    Select {
        #[arg(value_enum)]
        provider_name: ProviderName,
    },
}

/// Supported provider names
#[derive(Debug, Clone, ValueEnum)]
pub enum ProviderName {
    OpenWeather,
    AccuWeather,
    AerisWeather,
    WeatherApi,
}

/// Custom parser for date in YYYY-MM-DD format
fn parse_date(s: &str) -> Result<NaiveDate, chrono::format::ParseError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
}