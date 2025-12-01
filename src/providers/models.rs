use serde::Deserialize;
pub mod weather_api {
    use super::*;
    #[derive(Deserialize, Debug)]
    pub struct WeatherApiReport {
        pub(crate) forecast: WeatherApiForecast
    }

    #[derive(Deserialize, Debug)]
    pub(crate) struct WeatherApiForecast {
        pub(crate) forecastday: Vec<WeatherApiForecastDay>
    }

    #[derive(Deserialize, Debug)]
    pub(crate) struct WeatherApiForecastDay {
        pub(crate) date: String,
        pub(crate) day: WeatherApiDay
    }

    #[derive(Deserialize, Debug)]
    pub(crate) struct WeatherApiDay {
        pub(crate) maxtemp_c: f32,
        pub(crate) mintemp_c: f32,
        pub(crate) avgtemp_c: f32,
        pub(crate) maxwind_kph: f32,
        pub(crate) avghumidity: f32,
        pub(crate) daily_chance_of_rain: f32,
        pub(crate) condition: WeatherApiCondition
    }

    #[derive(Deserialize, Debug)]
    pub(crate) struct WeatherApiCondition {
        pub(crate) text: String,
    }

    #[derive(Deserialize, Debug)]
    pub(crate) struct WeatherApiForecastError {
        pub(crate) message: String,
        code: i32
    }
}

pub mod openweather_api {
    use super::*;

    #[derive(Deserialize, Debug)]
    pub(crate) struct OpenWeatherReport{
        pub(crate) daily: Vec<OpenWeatherDaily>
    }

    #[derive(Deserialize, Debug)]
    pub(crate) struct OpenWeatherDaily {
        pub(crate) dt: i64,
        pub(crate) summary: String,
        pub(crate) temp: OpeWeatherTemperature,
        pub(crate) humidity: f32,
        pub(crate) wind_speed: f32,
        pub(crate) wind_deg: i32,
        pub(crate) pop: f32
    }

    #[derive(Deserialize, Debug)]
    pub(crate) struct OpeWeatherTemperature {
        pub(crate) day: f32,
        pub(crate) min: f32,
        pub(crate) max: f32,
    }
}