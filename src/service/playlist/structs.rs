use serde::{Deserialize, Serialize};
use std::time::Duration;

use url::Url;

use crate::service::{
    id::{enums::Platform, structs::Id},
    playlist::enums::{Artist, MediaType},
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Playlist {
    pub title: String,
    pub length: u64,

    // for both playlists and tracks: source_id is the id for where this originated,
    // and dyn_id is the id for this 'true' playlist or track, and can change. generally, dyn_id is preferred.
    source_id: Id,
    dyn_id: Id,
    pub tracks: Vec<Track>,
}

impl Playlist {
    pub fn new(title: String, tracks: Vec<Track>, source_id: Id) -> Self {
        // calculate length from tracks
        let length = tracks.len() as u64;
        Self {
            title,
            length,
            tracks,
            source_id: source_id.clone(),
            dyn_id: source_id,
        }
    }
    pub fn id(&self) -> &Id {
        &self.dyn_id
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Track {
    pub title: String,
    pub length: Duration,
    pub artist: Artist,
    pub album: Option<Album>,
    pub source_id: Id,
    pub dyn_id: Id,
}

impl Track {
    pub fn from_playlist_track_json(ptj: PlaylistTrackJson) -> Self {
        let id = Id::new(Platform::Youtube, MediaType::Track, ptj.id);
        Self {
            title: ptj.title,
            length: Duration::from_secs(ptj.duration),
            artist: Artist::Community(ptj.channel),
            album: None,
            source_id: id.clone(),
            dyn_id: id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Clone)]
pub struct PlaylistMetadata {
    pub title: String,
    pub id: Id,
}
