use anyhow::anyhow;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tokio::sync::{mpsc, oneshot, watch};
use tokio_util::sync::CancellationToken;
use url::Url;

use crate::service::{
    file::{
        self,
        structs::{BinApps, DataSize},
    },
    gui::enums::Message,
    id::{enums::Platform, structs::Id},
    playlist::{
        download,
        enums::{Artist, DownloadEndType, ExtractorLineOut, MediaType, PlaylistMessage},
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

// Stores a TrackList but with a playlist metadata as well. Used when cloning a playlist may be expensive but the id is still needed.
#[derive(Debug, Clone)]
pub struct PTrackList {
    pub list: TrackList,
    pub metadata: PlaylistMetadata,
}

// Stores a `Track`'s 'metadata.' mostly just used for gui buttons to only redraw the button when important information changes.
#[derive(Debug, Hash, Clone)]
pub struct TrackMetadata {
    pub downloaded: bool,
    pub downloading: bool,
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
    pub fn randomize_order(&mut self) {
        self.order.randomize();
    }
}

pub struct PlaylistDownloadManager {
    tracklist: TrackList,
    playlist_id: Id,
    cancel_token: CancellationToken,
    stop_flag: Arc<AtomicBool>,
    restart_flag: bool,
    internal_t: Option<watch::Sender<Option<TrackList>>>,
    dead: bool,
    running: bool,
}
impl PlaylistDownloadManager {
    pub fn new(tracklist: TrackList, playlist_id: Id) -> Self {
        Self {
            tracklist,
            playlist_id,
            cancel_token: CancellationToken::new(),
            stop_flag: Arc::new(AtomicBool::new(false)),
            restart_flag: false,
            internal_t: None,
            dead: false,
            running: false,
        }
    }
    pub fn run(
        &mut self,
        // gui reply stream: directly gets track download start + progress
        gui_reply_stream: mpsc::Sender<Message>,
        // playlist sender: directly gets track download finish + playlist download finish
        playlist_sender: mpsc::Sender<PlaylistMessage>,
        process_sender: ProcessSender,
        bin_apps: BinApps,
    ) {
        if self.dead() {
            return;
        }
        if self.running {
            println!("cannot run same playlist manager twice");
        }
        self.running = true;

        // create mini task to map extractor lines to gui messages
        let (map_t, mut map_r) = mpsc::channel(100);
        let gui_reply_stream_clone = gui_reply_stream.clone();
        tokio::spawn(async move {
            let reply_stream = gui_reply_stream_clone.clone();
            while let Some((id, line)) = map_r.recv().await {
                match line {
                    ExtractorLineOut::DownloadProgress(data) => {
                        reply_stream
                            .send(Message::TrackDownloadStatus { id, data })
                            .await
                            .unwrap();
                    }
                    _ => {}
                }
            }
        });

        // setup internal communication to the task at hand
        let (internal_t, mut internal_r) = watch::channel(None);
        // send first value
        internal_t.send(Some(self.tracklist.clone())).unwrap();

        self.internal_t = Some(internal_t);
        let playlist_id = self.playlist_id.clone();
        let stop_flag = self.stop_flag.clone();
        let stop_flag_clone = stop_flag.clone();
        let playlist_sender_clone = playlist_sender.clone();
        let gui_reply_stream_clone = gui_reply_stream.clone();
        let process_sender = process_sender.clone();
        let map_t = map_t.clone();
        let bin_apps = bin_apps.clone();
        let async_block = async move {
            while internal_r.has_changed().unwrap_or(false) {
                println!("starting new loop in download");
                // pull the tracklist from the watch channel
                let tracklist = {
                    let t = internal_r.borrow_and_update().clone();
                    if let Some(t) = t {
                        t
                    } else {
                        println!("tracklist was None, breaking");
                        break;
                    }
                };

                // restart stop flag
                stop_flag_clone.store(false, Ordering::Relaxed);

                // run the playlist downloading logic
                println!("running playlist downloading logic lol");
                for track in tracklist.iter() {
                    // check to see if a stop was requested
                    if stop_flag_clone.load(Ordering::Relaxed) {
                        println!("breaking playlist download");
                        break;
                    }
                    // check to see if this current track was already downloaded
                    let (downloaded_t, downloaded_r) = oneshot::channel();
                    // send the request
                    playlist_sender_clone
                        .send(PlaylistMessage::CheckTrackDownloaded {
                            id: track.id().clone(),
                            result_sender: downloaded_t,
                        })
                        .await
                        .unwrap();
                    if let Ok(result) = downloaded_r.await {
                        // check the result
                        if result {
                            // track was downloaded; skip it
                            continue;
                        }
                    }

                    // Track Download Start message
                    gui_reply_stream_clone
                        .send(Message::TrackDownloadStarted {
                            id: track.id().clone(),
                        })
                        .await
                        .unwrap();

                    println!("Downloading track {}..", track.title);
                    download::download_track(
                        &track.download_url,
                        track.id().clone(),
                        file::util::track_dir_path().unwrap(),
                        track.id().to_string(),
                        bin_apps.clone(),
                        &process_sender,
                        &map_t,
                    )
                    .await
                    .unwrap();

                    // Track Download End message
                    playlist_sender_clone
                        .send(PlaylistMessage::TrackDownloadDone {
                            id: track.id().clone(),
                        })
                        .await
                        .unwrap();
                }
            }
        };

        let cancel_token_clone = self.cancel_token.clone();
        let _ = tokio::spawn(async move {
            let stop_kind = tokio::select! {
                _ = cancel_token_clone.cancelled() => {DownloadEndType::Cancelled},
                _ = async_block => {if stop_flag.load(Ordering::Relaxed) {DownloadEndType::Stopped} else {DownloadEndType::Finished}},
            };

            println!("finished downloading");

            // Playlist Download End Message
            playlist_sender
                .send(PlaylistMessage::PlaylistDownloadDone {
                    success: if let DownloadEndType::Finished = stop_kind {
                        true
                    } else {
                        false
                    },
                    id: playlist_id,
                })
                .await
                .unwrap();
        });
    }
    pub fn stop(&mut self) {
        if self.dead() {
            return;
        }
        // send signal that no more tracks are coming
        if let Some(internal_t) = &self.internal_t {
            internal_t.send(None).unwrap();
        }
        // stop the current track download
        self.stop_flag.store(true, Ordering::Relaxed);
        self.dead = true;
    }
    pub fn cancel(&mut self) {
        if self.dead() {
            return;
        }
        self.dead = true;

        println!("Cancelling manager..");
        self.cancel_token.cancel();
    }
    pub fn get_playlist_id(&self) -> &Id {
        &self.playlist_id
    }
    pub fn restart(&mut self) {
        if self.dead() {
            return;
        }

        self.restart_flag = true;
        // stop the current track download to prepare for the next
        self.stop_flag.store(true, Ordering::Relaxed);
    }
    pub fn restart_with_tracklist(&mut self, tracklist: TrackList) {
        if self.dead() {
            return;
        }

        if let Some(internal_t) = &self.internal_t {
            internal_t.send(Some(tracklist)).unwrap();
        }
        self.restart();
    }
    fn dead(&self) -> bool {
        if self.dead {
            println!("cannot run methods on dead playlist manager");
        }
        self.dead
    }
}
