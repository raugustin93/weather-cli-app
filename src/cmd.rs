use clap::{App, Arg};

#[derive(Debug)]
pub struct Cli;

impl Cli {
    pub fn new() -> App<'static, 'static> {
        // basic app information
        let app = App::new("Weather CLI App")
            .version("1.0.0")
            .author("Rudson Augustin")
            .about("Displays weather information for a specified location");

        // Define the name command line option

        let zip_option = Arg::with_name("zipcode")
            .long("zipcode")
            .short("z")
            .takes_value(true)
            .required(false);

        let latitude_option = Arg::with_name("latitude")
            .long("latitude")
            .short("t")
            .takes_value(true)
            .required(false);

        let longitude_option = Arg::with_name("longitude")
            .long("longitude")
            .short("n")
            .takes_value(true)
            .required(false);

        let app = app.arg(zip_option);
        let app = app.arg(latitude_option);
        let app = app.arg(longitude_option);
        app
    }
}
