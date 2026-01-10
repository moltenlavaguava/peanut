use anyhow::anyhow;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use url::Url;

use crate::service::{
    file::{self, structs::BinApps},
    id::{enums::Platform, structs::Id},
    playlist::{
        download,
        enums::{Artist, MediaType},
    },
    process::ProcessSender,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Playlist {
    pub title: String,

    // for both playlists and tracks: source_id is the id for where this originated,
    // and dyn_id is the id for this 'true' playlist or track, and can change. generally, dyn_id is preferred.
    source_id: Id,
    dyn_id: Id,
    pub tracks: Vec<Track>,
}

impl Playlist {
    pub fn new(title: String, tracks: Vec<Track>, source_id: Id) -> Self {
        Self {
            title,
            tracks,
            source_id: source_id.clone(),
            dyn_id: source_id,
        }
    }
    pub fn id(&self) -> &Id {
        &self.dyn_id
    }
    pub fn length(&self) -> u64 {
        self.tracks.len() as u64
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
    pub index: u64,
    pub download_url: Url,
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
            index: ptj.playlist_index,
            download_url: ptj.url,
        }
    }
    pub fn id(&self) -> &Id {
        &self.dyn_id
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Album {
    pub name: String,
    pub artists: Vec<String>,
}

#[derive(Debug, Deserialize)]
// created on playlist initialization
pub struct PlaylistTrackJson {
    url: Url,
    title: String,
    duration: u64,
    channel: String,
    playlist_index: u64,
    pub playlist_id: String,
    id: String,
}

#[derive(Debug, Deserialize)]
// created on track download 
pub struct TrackDownloadJson {
    
}

#[derive(Debug)]
pub struct DownloadTrackJson {}

#[derive(Debug, Clone)]
pub struct PlaylistMetadata {
    pub title: String,
    pub id: Id,
}

// Stores a `Track`'s 'metadata.' mostly just used for gui buttons to only redraw the button when important information changes.
#[derive(Debug, Hash, Clone)]
pub struct TrackMetadata {
    pub downloaded: bool,
    // needed to prevent unnecessary copying
    pub title: Arc<str>,
}

#[derive(Debug)]
pub struct TrackOrder {
    index_order: Vec<u64>,
}
impl TrackOrder {
    pub fn from_playlist(playlist: &Playlist) -> Self {
        Self::from_length(playlist.length())
    }
    pub fn from_length(length: u64) -> Self {
        TrackOrder {
            index_order: (0..=length - 1).collect(),
        }
    }
    pub fn randomize(&mut self) {
        // get mut reference to the internal vec
        let slice = &mut self.index_order;
        // get some rng
        let mut rng = rand::rng();
        slice.shuffle(&mut rng);
    }
    pub fn iter_playlist<'a>(
        &self,
        playlist: &'a Playlist,
    ) -> anyhow::Result<impl Iterator<Item = &'a Track>> {
        if self.index_order.len() as u64 != playlist.length() {
            return Err(anyhow!(
                "Own index order and playlist length are different ({} vs {})",
                self.index_order.len(),
                playlist.length()
            ));
        }
        let iter = self
            .index_order
            .iter()
            .map(|index| &playlist.tracks[*index as usize]);
        Ok(iter)
    }
}

pub struct PlaylistDownloadManager {
    playlist: Arc<Playlist>,
    track_order: TrackOrder,
    cancel_token: CancellationToken,
}
impl PlaylistDownloadManager {
    pub fn new(
        playlist: Arc<Playlist>,
        track_order: TrackOrder,
        cancel_token: CancellationToken,
    ) -> Self {
        Self {
            playlist,
            track_order,
            cancel_token,
        }
    }
    pub async fn run<F1: AsyncFnMut(Id), F2: AsyncFnOnce(bool)>(
        self,
        mut on_track_download: F1,
        on_finish: F2,
        process_sender: ProcessSender,
        bin_apps: BinApps,
    ) {
        // run the playlist downloading logic
        println!("running playlist downloading logic lol");
        for track in self.track_order.iter_playlist(&self.playlist).unwrap() {
            println!("Downloading track {}..", track.title);
            download::download_track(
                &track.download_url,
                file::util::track_dir_path().unwrap(),
                track.id().to_string(),
                bin_apps.clone(),
                &process_sender,
                // status_sender,
            )
            .await
            .unwrap();
            on_track_download(track.id().clone()).await;
        }

        on_finish(false).await;
    }
}
