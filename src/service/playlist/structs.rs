use std::time::Duration;
use serde::Deserialize;

use url::Url;

pub struct Playlist {
    pub name: String,
    pub length: usize,
    pub tracks: Vec<Track>,
}

pub struct Track {
    pub name: String,
    pub length: Duration,
    pub artists: Vec<String>,
    pub album: Option<Album>,
}

pub struct Album {
    pub name: String,
    pub artists: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct InitJsonOutput {
    url: Url,
    title: String,
    duration: usize,
    channel: String,
    playlist_count: usize,
    playlist_index: usize,
    id: String,
}

#[derive(Debug)]
pub struct DownloadJsonOutput {

}