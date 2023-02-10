use dotenv::dotenv;
use reqwest::Client;
use serde;
use std::env;

#[derive(Debug, serde::Deserialize)]
struct Coordinates {
    lon: f32,
    lat: f32,
}

#[derive(Debug, serde::Deserialize)]
struct CurrentWeather {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, serde::Deserialize)]
struct WeatherData {
    coord: Coordinates,
    weather: Vec<CurrentWeather>,
}

async fn get_weather_data(url: &str) -> Result<WeatherData, reqwest::Error> {
    let client = Client::new();
    let response = client.get(url).send().await?.json().await?;
    Ok(response)
}

pub async fn make_get_request_with_zipcode(zipcode: &str) {
    dotenv().ok();
    let value = env::var("api_key").expect("The API Key must be set");
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?zip={},us&APPID={}&units=imperial",
        zipcode, value
    );
    let results = get_weather_data(&url).await;
    match results {
        Ok(weather_data) => {
            println!("Main: {}", weather_data.weather[0].main);
            println!("Description: {}", weather_data.weather[0].description);
        }
        Err(error) => println!("The error is: {}", error),
    }
}
