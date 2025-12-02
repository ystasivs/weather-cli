use crate::configs::Config;
use crate::geocoder::{Geocoder, GeocoderError};
use crate::input::read_user_number;
use crate::provider_builder::{ProviderBuilder, ProviderBuilderError};
use crate::providers::error::ProviderError;

use chrono::NaiveDate;
use thiserror::Error;

const POPULATION_THRESHOLD: u32 = 100_000;

#[derive(Error, Debug)]
pub enum WeatherCliError {
    #[error("Geocoder failed with an error: {0}")]
    Geocoder(#[from] GeocoderError),
    #[error("Provider builder failed with an error: {0}")]
    ProviderBuilder(#[from] ProviderBuilderError),
    #[error("Provider failed with an error: {0}")]
    ProviderError(#[from] ProviderError),
}

pub fn run(
    latitude: Option<f64>,
    longitude: Option<f64>,
    toponym: Option<String>,
    country_code: Option<String>,
    date: NaiveDate,
) -> Result<(), WeatherCliError> {
    let config = Config::load();
    let provider = ProviderBuilder::build_provider(config)?;
    let (lat, lon) = if toponym.is_some() {
        let toponym = toponym.unwrap();
        let mut geo = Geocoder::resolve_address(toponym.clone(), country_code)?;
        let top = if geo.results.len() == 1 {
            geo.results.remove(0)
        } else if let Some(res) = geo
            .results
            .iter()
            .find(|r| r.population.unwrap_or_default() >= POPULATION_THRESHOLD)
        {
            res.clone()
        } else {
            println!("Found multiple matches for \"{}\":", toponym);
            println!("{geo}");
            println!("Enter a number 1-{}:", geo.results.len());
            let idx = read_user_number(1, geo.results.len(), 3).unwrap();
            geo.results.remove(idx - 1)
        };
        (top.latitude, top.longitude)
    } else {
        (latitude.unwrap(), longitude.unwrap())
    };
    let report = provider.get_weather(lat, lon, date)?;
    println!("{}", report);
    Ok(())
}
