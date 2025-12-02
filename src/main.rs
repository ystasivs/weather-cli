use clap::Parser;

mod providers;
mod geocoder;
mod input;
mod argparser;
mod configs;
mod provider_builder;
mod cli;

use argparser::{Argparser, Commands};
use cli::run;
fn main() {
    let args = Argparser::parse();
    if args.command.is_none() {
        let toponym_present = args.toponym.is_some();
        let lat_long_present = args.latitude.is_some() && args.longitude.is_some();

        if !toponym_present && !lat_long_present {
            eprintln!("Error: Location is required for this command. Provide <TOPONYM> or --latitude and --longitude.");
            std::process::exit(1);
        }
    } else {
        match args.command {
            None => {
                if let Err(e) = run(args.latitude, args.longitude, args.toponym, args.country_code, args.date){
                    eprintln!("{}", e);
                }
            },
            Some(Commands::Configure { provider_name }) => {
                configs::set_config_for_provider(&provider_name);
                println!("updated config for {}", provider_name);
            },
            Some(Commands::Select { provider_name }) => {
                configs::select_default_provider(&provider_name);
                println!("selected {}", provider_name);
            }
        }
    }
}
