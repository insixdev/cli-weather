/// client do the HTTP req functions
use curl::easy::Easy;
/// new url req for a city, of forecast information the basic of the applicatio
/// url_wheater is thinking for the request on the endpoint current of the api


pub fn url_req_city(url_b: &String, city: String, api_key: String, current: &str) -> String {
   let final_url = format!("{}{}", url_b, current);
   let url_wheater = format!("{}?key={}&q={}",final_url, api_key, city); // format devuelve un
   url_wheater
}

// easy struct es modificada se hace una transferencia por pedida 
// exelente para un cli, sin complicaciones de varias request por segundo 

/// New transfer of the main easy req with a given formated url 
/// what is the endpoint
/// # return 
/// Vec<u8> with the res_data
/// res_data can be convert with UTF_8 parser
pub fn new_transfer(easy: &mut Easy, url_wheater: &str) -> Vec<u8> {
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
   res_data

}


