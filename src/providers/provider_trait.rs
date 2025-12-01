use chrono::NaiveDate;

use super::ProviderResult;
use super::ProvidersReport;

pub trait WeatherProvider{
    fn get_weather(&self, latitude: f64, longitude: f64, date: NaiveDate) -> ProviderResult<ProvidersReport>;
}