use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Song {
    name: String,
    author: String,
    duration_ms: u16
}


#[derive(Debug, Serialize, Clone)]
pub struct Playlist {
    pub name: String,
    pub songs: Vec<Song>
}

