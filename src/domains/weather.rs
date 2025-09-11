use serde::Deserialize;

#[derive(Debug)]
enum Temp {
   METRIC = 0,
   IMPERIAL = 1,
   STANDART = 2,
}
pub struct RequestGet{
   lang: String,
   city: String,
   humidity: u8,
   temp: String,
   pressure: Option<u32>,
   wind_speed: f32,
   datetime: Option<String>,
   days: Option<i32> // ponele
}

#[derive(Debug, Deserialize)]
pub struct WeatherData{
    pub location: Location,
    pub current: Current,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub localtime: String,
}

#[derive(Debug, Deserialize)]
pub struct Current {
    pub temp: f64,
    pub condition: Condition,
    pub is_day: u8,
    pub cloud: u8,
    // icon no lo incluimos
}

#[derive(Debug, Deserialize)]
pub struct Condition {
    pub text: String,
    // icon no lo incluimos
}

