use std::fmt;
use thiserror::Error;
use reqwest::blocking::get;
use serde::Deserialize;
pub struct Geocoder;

impl Geocoder {
    // const POPULATION_THRESHOLD: u32 = 100_000;

    pub fn resolve_address(toponym: String, country_code: Option<String>) -> Result<GeocoderResult, GeocoderError> {
        let country_code = country_code
            .map(|c| format!("&countryCode={c}"))
            .unwrap_or_default();
        let url = format!("https://geocoding-api.open-meteo.com/v1/search?name={toponym}&count=20&language=en&format=json{country_code}");
        let response = get(&url)?;
        let status = response.status();
        match status{
            reqwest::StatusCode::OK => {
                let results: GeocoderResult = response.json().map_err(|e| GeocoderError::ParseError(e.to_string()))?;
                if results.results.is_empty() {
                    return Err(GeocoderError::ParseError("no results".into()));
                }
                Ok(results)
            },
            _ => {
                let body = response.text().unwrap_or_else(|_| "<failed to read body>".into());
                Err(GeocoderError::GeocoderError(body, status.as_u16()))
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum GeocoderError{
    #[error("failed to send request to geocoder, e={0}")]
    FailedRequest(#[from] reqwest::Error),
    #[error("geocoder returned: {0}. with status code {1}")]
    GeocoderError(String, u16),
    #[error("failed to parse response body")]
    ParseError(String)
}

#[derive(Deserialize, Debug)]
pub struct GeocoderResult {
    pub results: Vec<GeocoderToponym>,
}

impl fmt::Display for GeocoderResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Header with row number
        writeln!(
            f,
            "{:>4} | {:<20} | {:>9} | {:>9} | {:>10} | {:<20} | {:<15} | {:<15} | {:<15} | {:<15}",
            "No.",
            "Name",
            "Latitude",
            "Longitude",
            "Population",
            "Country",
            "Admin1",
            "Admin2",
            "Admin3",
            "Admin4",
        )?;

        // Separator line
        writeln!(
            f,
            "{}",
            "-".repeat(
                4 + 1 + 20 + 1 + 9 + 1 + 9 + 1 + 10 + 1 + 20 + 1 + 15 + 1 + 15 + 1 + 15 + 1 + 15 + 10
            )
        )?;

        // Rows
        for (i, toponym) in self.results.iter().enumerate() {
            writeln!(
                f,
                "{:>4} | {}",
                i + 1, // row number starting from 1
                toponym
            )?;
        }

        Ok(())
    }
}


#[derive(Deserialize, Debug, Clone)]
pub struct GeocoderToponym {
    name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub population: Option<u32>,
    country: String,
    admin1: Option<String>,
    admin2: Option<String>,
    admin3: Option<String>,
    admin4: Option<String>
}

impl fmt::Display for GeocoderToponym {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<20} | {:>9.5} | {:>9.5} | {:>10} | {:<20} | {:<15} | {:<15} | {:<15} | {:<15}",
            self.name,                    // left-aligned, width 20
            self.latitude,                // right-aligned, width 9, 5 decimals
            self.longitude,               // same as above
            self.population.map(|p| p.to_string()).unwrap_or_default(),
            self.country,                 // left-aligned, width 20
            self.admin1.clone().unwrap_or_default(),
            self.admin2.clone().unwrap_or_default(),
            self.admin3.clone().unwrap_or_default(),
            self.admin4.clone().unwrap_or_default(),
        )
    }
}