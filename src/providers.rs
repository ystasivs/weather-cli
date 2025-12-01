pub mod weather_api;
mod provider_trait;
mod error;
pub mod open_weather;
mod models;
mod providers_report;

pub use error::ProviderResult;
pub use providers_report::ProvidersReport;
pub use provider_trait::WeatherProvider;