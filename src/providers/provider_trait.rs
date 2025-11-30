use super::ProviderResult;

trait WeatherProvider{
    type Output;
    fn get_weather(location: &str, date: &str) -> ProviderResult<Self::Output>;
}