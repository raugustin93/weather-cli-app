extern crate clap;

// use clap::{Arg, Command};
use clap::{App, Arg};
use std::error::Error;
use tokio;

mod api;
mod cmd;

// mod args;

// use args::WeatherArgs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = cmd::Cli::new();
    // extract the matches
    let matches = app.get_matches();

    // Extract the actual match
    let zipcode = matches.value_of("zipcode");

    if zipcode.is_some() {
        api::make_get_request_with_zipcode(zipcode.unwrap()).await;
    }
    if zipcode.is_none() {
        println!("Needs a zipcode for demo version")
    }

    println!("Done");
    Ok(())
}
