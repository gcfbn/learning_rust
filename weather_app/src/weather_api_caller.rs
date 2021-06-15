use crate::query::Query;
use crate::raw_weather_data::WeatherData;
use restson::{RestClient, Error};


pub fn call_weather_api(query: &Query) -> Result<WeatherData, restson::Error> {
    let mut client = RestClient::new("https://api.openweathermap.org/data/2.5/weather")?;
    client.get(query)
}