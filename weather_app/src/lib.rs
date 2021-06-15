extern crate serde_derive;

use crate::query::Query;
use crate::raw_weather_data::WeatherData;

pub mod raw_weather_data;
pub mod query;
pub mod weather_api_caller;

pub fn run(query: &Query) -> Result<WeatherData, restson::Error> {
    weather_api_caller::call_weather_api(query)
}