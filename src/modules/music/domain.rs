
use serde::{Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: i32,
    pub path: String,
    pub cover: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Playlist {
    pub id: i32,
    pub name: String,
    pub songs: Vec<Song>,
}