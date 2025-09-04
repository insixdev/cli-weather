use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Current {
    temp_c: f64,
    condition: Condition,
}

#[derive(Debug, Deserialize)]
pub struct Condition {
    text: String,
    icon: String,
}

pub fn url_req_city(url_b: &String, city: String, api_key: String, current: &str) -> String {
    let final_url = format!("{}{}", url_b, current);
    let url_wheater = format!("{}?key={}&q={}",final_url, api_key, city); // format devuelve un
    url_wheater
}


