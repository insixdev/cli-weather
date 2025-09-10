use curl::easy::Easy;
use dotenv::dotenv;
use std::env;
mod api;
mod cli;
use crate::{api::{models::WeatherData, request::*}, cli::commands::command_forecast};

// tipo string para constantes &str, &'static str, es la otra opcion para q este nunca se elimine y
// esta directamente enlazado con el binarios raiz del programa

const API_URL: &str= "http://api.weatherapi.com/v1/";
// variable de entorno
fn main() {
   dotenv().ok();
   
   let api_key = env::var("WEATHERAPI_KEY").expect("API key no encontrada");
   let url_base = API_URL.to_string();
   // url_wheater es de tipo referencia str
   // string tempa
   let url_wheater = &url_req_city(&url_base, "Montevideo".to_string(), api_key, "current.json");

   let mut easy = Easy::new();

   let res_data = new_transfer(&mut easy, url_wheater);

   let res_string = String::from_utf8(res_data).unwrap();
   let weather: WeatherData= serde_json::from_str(&res_string).unwrap();
   print!("{:?}", &weather);
   command_forecast(weather);
}

