use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
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
    pub temp_c: f64,
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

