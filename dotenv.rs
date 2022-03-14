.env 
PORT=1234

// Read Environment Variables

use std::env;

let port = match env::var("PORT") {
    OK(port) => port,
    Err(_) => "555".to_owned()
}

//dotenv - Load Environment Variables
use std::env;
use dotenv::dotenv;

dotenv().ok();

let port = match env::var("PORT") {
    OK(port) => port,
    Err(_) => "555".to_owned()
}
