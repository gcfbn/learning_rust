use crate::query::Query;
use crate::raw_weather_data::WeatherData;
use restson::RestClient;


pub fn weather_info(query: &Query) -> Result<WeatherData, restson::Error> {
    let mut client = RestClient::new("https://api.openweathermap.org/data/2.5/weather")?;
    client.get(query)
}