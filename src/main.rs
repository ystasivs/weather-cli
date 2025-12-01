use chrono::NaiveDate;
use clap::Parser;
use geocoder::Geocoder;
use input::read_user_number;
use providers::open_weather::OpenWeather;

mod providers;
mod geocoder;
mod input;
mod argparser;

fn main() {
    // let args = Argparser::parse();
    let toponym = "Pavlivka";
    let country = Some("UA");
    let date = NaiveDate::from_ymd_opt(2025, 12, 5).unwrap();

    let mut geo = Geocoder::resolve_address(toponym, country).unwrap();
    let top = if geo.results.len() == 1 {
        geo.results.remove(0)
    } else {
        println!("Found multiple matches for \"{}\":", toponym);
        println!("{geo}");
        println!("Enter a number 1-{}:", geo.results.len());
        let idx = read_user_number(1, geo.results.len(), 3).unwrap();
        geo.results.remove(idx - 1)
    };
    println!("TOTP: {}", top);
    // let provider = WeatherApi::new("88793bbbe01c4cdaa74132047252911".to_string());
    // let res = provider.get_weather(top.latitude, top.longitude, date).unwrap();
    // println!("{:#?}", res);
    let open_weather_api = "18f9d9d962c74a0e011d96cf4f825bdb";
    let open_provider = OpenWeather::new(open_weather_api.to_string());
    // open_provider.get_weather(top.latitude, top.longitude, date).unwrap()
}
