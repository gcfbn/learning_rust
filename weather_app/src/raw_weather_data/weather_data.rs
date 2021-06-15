use super::inner;

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherData{
    pub coord: inner::Coord,
    pub weather: inner::Weather,
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
    pub cod: String,
}