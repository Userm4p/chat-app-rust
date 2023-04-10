use actix_web::{get, web, Responder };
use serde::{Deserialize};

use super::super::domain::Playlist;

#[derive(Debug, Deserialize)]
struct info {
    id: i32,
}

#[get("/playlist")]
async fn index() -> impl Responder {
    let mut playlists : Vec<Playlist> = vec![];
    let pls: Playlist = Playlist {
        id: 1,
        name: "Playlist 1".to_string(),
        songs: vec![],
    };

    playlists.push(pls);


    web::Json(playlists)
}


#[get("/playlist/{id}")]
async fn get_playlist(info:web::Path<info>) -> impl Responder {
    let playlists : Vec<Playlist> = vec![Playlist {
        id: 1,
        name: "Playlist 1".to_string(),
        songs: vec![],
    }];

    let pl = playlists[info.id as usize].clone();

    web::Json(pl)
}



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(get_playlist);
}
