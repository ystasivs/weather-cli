use crate::geocoder::Geocoder;
use crate::input::read_user_number;
mod providers;
mod geocoder;
mod input;

fn main() {
    let toponym = "Pavlivka30";
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
