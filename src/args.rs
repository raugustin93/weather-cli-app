use clap::Parser;

#[derive(Debug, Parser)]
// #[clap(city, state, country, zipcode)]
pub struct WeatherArgs {
    /// The City your looking for
    pub city: Option<String>,
    /// The State your looking for (USA)
    pub state: Option<String>,
    /// Please use ISO 3166 country codes
    pub country: Option<String>,
    /// The Zipcode your looking for
    pub zipcode: Option<String>,
}
