use std::time::Duration;

use super::enums::TrackSource;

pub struct Playlist {
    pub name: String,
    pub display_name: String,
    pub length: usize,
    pub tracks: Vec<Track>,
}

pub struct Track {
    pub name: String,
    pub display_name: String,
    pub length: Duration,
    pub artists: Vec<String>,
    pub album: Option<Album>,
    pub source: TrackSource,
}

pub struct Album {
    pub name: String,
    pub display_name: String,
    pub artists: Vec<String>,
}
