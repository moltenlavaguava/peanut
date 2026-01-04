use serde::Deserialize;
use std::time::Duration;

use url::Url;

use crate::service::playlist::enums::Artist;

#[derive(Debug)]
pub struct Playlist {
    pub title: String,
    pub length: u64,
    pub tracks: Vec<Track>,
}

impl Playlist {
    pub fn new(title: String, tracks: Vec<Track>) -> Self {
        // calculate length from tracks
        let length = tracks.len() as u64;
        Self {
            title,
            length,
            tracks,
        }
    }
}

#[derive(Debug)]
pub struct Track {
    pub title: String,
    pub length: Duration,
    pub artist: Artist,
    pub album: Option<Album>,
}

impl Track {
    pub fn from_playlist_track_json(ptj: PlaylistTrackJson) -> Self {
        Self {
            title: ptj.title,
            length: Duration::from_secs(ptj.duration),
            artist: Artist::Community(ptj.channel),
            album: None,
        }
    }
}

#[derive(Debug)]
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
    playlist_count: usize,
    playlist_index: usize,
    id: String,
}

#[derive(Debug)]
pub struct DownloadTrackJson {}
