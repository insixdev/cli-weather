use curl::easy::Easy;

use dotenv::dotenv;
use std::env;
mod api;
// tipo string para constantes &str, &'static str, es la otra opcion para q este nunca se elimine y
// esta directamente enlazado con el binarios raiz del programa

const API_URL: &str= "http://api.weatherapi.com/v1/";

// variable de entorno

fn main() {
    dotenv().ok();
    let api_key = env::var("WEATHERAPI_KEY").expect("API key no encontrada");

    let url_base = API_URL.to_string();
    // url_wheater es de tipo referencia str
    // string temp
    let url_wheater = &url_req(&url_base, "Montevideo".to_string(), api_key, "current.json");

    let mut easy = Easy::new();
    easy.url(url_wheater).unwrap(); // unwrap es por si la peticion alurl falla, si falla hace panic
    let mut res_data = Vec::new(); // guardamos los bytes de la res
    
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data|{  // data funcion anonima 
            res_data.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
       transfer.perform().unwrap(); // execute
    }
    let res_string = String::from_utf8(res_data).unwrap();
    print!("res json {}", res_string);

}
