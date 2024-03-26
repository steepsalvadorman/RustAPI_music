use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Info {
    pub id: usize
}

#[derive(Debug, Deserialize)]
pub struct CreatePlaylist {
    pub name: String
}