use std::env;
use dotenv::dotenv;



pub struct Config {
    pub port: u16,
    pub host: String,
}



pub fn read_config() -> Config {

    dotenv().ok();

    let port = env::var("PORT")
        .expect("PORT must be set")
        .parse::<u16>()
        .expect("PORT must be a number");

    let host = env::var("HOST")
        .expect("HOST must be set");

    Config {
        port,
        host,
    }
}