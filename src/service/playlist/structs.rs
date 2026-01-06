use serde::{Deserialize, Serialize};
use std::time::Duration;

use url::Url;

use crate::service::{
    id::{enums::Platform, structs::Id},
    playlist::enums::{Artist, MediaType},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Playlist {
    pub title: String,
    pub length: u64,
    pub id: Id,
    pub tracks: Vec<Track>,
}

impl Playlist {
    pub fn new(title: String, tracks: Vec<Track>, id: Id) -> Self {
        // calculate length from tracks
        let length = tracks.len() as u64;
        Self {
            title,
            length,
            tracks,
            id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Track {
    pub title: String,
    pub length: Duration,
    pub artist: Artist,
    pub album: Option<Album>,
    pub id: Id,
}

impl Track {
    pub fn from_playlist_track_json(ptj: PlaylistTrackJson) -> Self {
        Self {
            title: ptj.title,
            length: Duration::from_secs(ptj.duration),
            artist: Artist::Community(ptj.channel),
            album: None,
            id: Id::new(Platform::Youtube, MediaType::Track, ptj.id),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Album {
    pub name: String,
    pub artists: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistTrackJson {
    url: Url,
    title: String,
    duration: u64,
    channel: String,
    playlist_index: usize,
    pub playlist_id: String,
    id: String,
}

#[derive(Debug)]
pub struct DownloadTrackJson {}
