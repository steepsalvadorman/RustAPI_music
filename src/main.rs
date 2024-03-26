use std::sync::{Arc, Mutex};
use std::vec;

use actix_web::{web, App, HttpServer};
use api_play::infra;
use api_play::config::read_config;
use api_play::modules::music;
use api_play::modules::music::domain::Playlist;
use api_play::state::State;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = read_config();
    let _stack: Vec<Playlist> = vec![];
    let playlists = Arc::new(Mutex::new(Vec::new()));

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(State{
            playlist: playlists.clone(),
        
        }))
        .service(web::scope("/api")
        .configure(infra::endpoints::config)
        .configure(music::infra::endpoints::config)
        
    )
    })
        .bind((config.host, config.port))?
        .run()
        .await
}