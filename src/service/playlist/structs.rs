use anyhow::anyhow;
use futures::future::BoxFuture;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use tokio::{task::JoinHandle, time::sleep};
use tokio_util::sync::CancellationToken;
use url::Url;

use crate::service::{
    file::{
        self,
        structs::{BinApps, DataSize},
    },
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
pub struct TrackDownloadJson {}

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

#[derive(Debug, Clone)]
pub struct TrackDownloadData {
    pub progress: f32,
    pub download_size: DataSize,
    pub download_speed: DataSize,
    pub eta: Duration,
}

#[derive(Debug, Clone)]
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
    pub fn length(&self) -> usize {
        self.index_order.len()
    }
    pub fn randomize(&mut self) {
        // get mut reference to the internal vec
        let slice = &mut self.index_order;
        // get some rng
        let mut rng = rand::rng();
        slice.shuffle(&mut rng);
    }
    pub fn order(&self) -> &Vec<u64> {
        &self.index_order
    }
}

#[derive(Debug, Clone)]
pub struct TrackList {
    order: TrackOrder,
    tracks: Arc<Vec<Track>>,
}
impl TrackList {
    pub fn new(order: TrackOrder, tracks: Vec<Track>) -> anyhow::Result<Self> {
        // verify the length is the same between the track order and the tracks provided
        if order.length() != tracks.len() {
            return Err(anyhow!("Length of TrackOrder and tracks vec are not equal"));
        }
        Ok(Self {
            order,
            tracks: Arc::new(tracks),
        })
    }
    pub fn from_playlist_ref(playlist: &Playlist) -> Self {
        Self::new(
            TrackOrder::from_playlist(&playlist),
            playlist.tracks.clone(),
        )
        .expect("Track order from playlist should have same length as playlist itself")
    }
    pub fn iter(&self) -> impl Iterator<Item = &Track> {
        self.order
            .order()
            .iter()
            .map(|index| &self.tracks[*index as usize])
    }
}

pub struct PlaylistDownloadManager {
    tracklist: TrackList,
    join_handle: Option<JoinHandle<()>>,
}
impl PlaylistDownloadManager {
    pub fn new(tracklist: TrackList) -> Self {
        Self {
            tracklist,
            join_handle: None,
        }
    }
    pub fn run<F1, F2>(
        &mut self,
        mut on_track_download: F1,
        on_finish: F2,
        process_sender: ProcessSender,
        bin_apps: BinApps,
    ) where
        F1: FnMut(Id) -> BoxFuture<'static, ()> + Send + 'static,
        F2: FnOnce(bool) -> BoxFuture<'static, ()> + Send + 'static,
    {
        let tracklist = self.tracklist.clone();
        let join_handle = tokio::spawn(async move {
            // run the playlist downloading logic
            println!("running playlist downloading logic lol");
            for track in tracklist.iter() {
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

            println!("finished downloading");
            on_finish(false).await;
        });

        self.join_handle = Some(join_handle);
    }
    pub fn cancel(&mut self) {
        println!("Cancelling manager..");
        if let Some(handle) = &self.join_handle {
            handle.abort();
        }
    }
}
