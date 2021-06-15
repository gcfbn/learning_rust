pub mod inner;

use serde::{Serialize, Deserialize};
use restson::RestPath;

use crate::query::Query;

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherData{
    pub coord: inner::Coord,
    pub weather: Vec<inner::Weather>, // this API returns "weather" as JSON Array
    pub base: String,
    pub main: inner::Main,
    pub visibility: i32,
    pub wind: inner::Wind,
    pub clouds: inner::Clouds,
    pub dt: u64,
    pub sys: inner::Sys,
    pub timezone: i32,
    pub id: i64,
    pub name: String,
    pub cod: i32,
}

impl RestPath<&str> for WeatherData {
    fn get_path(city: &str) -> Result<String, restson::Error> {
        Ok(format!("?q={}&appid=a52958f9ad25d7d64c67d97957bc6119", city))
    }
}

impl RestPath<&Query> for WeatherData {
    fn get_path(query: &Query) -> Result<String, restson::Error> {
        Ok(format!("?q={}&units={}&lang={}", query.city, query.units.to_string(), query.language.to_string()))
    }
}

