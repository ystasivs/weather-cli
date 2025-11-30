use clap::Parser;
use geocoder::Geocoder;
use input::read_user_number;
use argparser::Argparser;
mod providers;
mod geocoder;
mod input;
mod argparser;

fn main() {
    let args = Argparser::parse();
    let toponym = "Pavlivka";
    let country = Some("UA");
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
}
