use std::sync::MutexGuard;

use actix_web::{get,post, web, Responder};
use super::super::domain::Playlist;
use super::dtos::{Info, CreatePlaylist};
use crate::state::State;



#[get("/playlist")]
async fn playlist(data: web::Data<State>) -> impl Responder {

    let playlists: MutexGuard<Vec<Playlist>> = data.playlist.lock().expect("bad state");
    web::Json(playlists.clone())
}

#[get("/playlist/{id}")]
async fn get_playlist(info: web::Path<Info>, data: web::Data<State>) -> impl Responder {

    let playlists: MutexGuard<Vec<Playlist>> = data.playlist.lock().expect("bad state");


    let p1 = playlists[info.id].clone();

    web::Json(p1)

}

#[post("/playlist")]
async fn create_playlist(dto: web::Json<CreatePlaylist>, data: web::Data<State>) -> impl Responder {

    let mut playlists: MutexGuard<Vec<Playlist>> = data.playlist.lock().expect("bad state");
    let p1: Playlist = Playlist {
        name: dto.name.clone(),
        songs: vec![]
    };

    playlists.push(p1.clone());
    web::Json(p1)
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_playlist);
    cfg.service(playlist);
    cfg.service(create_playlist);
}