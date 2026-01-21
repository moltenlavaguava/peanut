use anyhow::anyhow;
use atomic_float::AtomicF64;
use futures::FutureExt;
use musicbrainz_rs::MusicBrainzClient;
use parking_lot::Mutex;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::Duration,
};
use tokio::sync::{Notify, mpsc, oneshot, watch};
use tokio_util::sync::CancellationToken;
use url::Url;

use crate::service::{
    audio::{
        AudioSender,
        enums::{AlbumKind, AudioMessage},
        structs::AudioConfig,
    },
    file::{
        self,
        structs::{BinApps, DataSize},
    },
    gui::enums::Message,
    id::{enums::Platform, structs::Id},
    playlist::{
        PlaylistSender, download,
        enums::{Artist, DownloadEndType, ExtractorLineOut, MediaType, PlaylistMessage},
        util,
    },
    process::ProcessSender,
};

// --- PLAYLIST STRUCTS --- //

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlaylistMetadata {
    pub title: String,
    // for both playlists and tracks: source_id is the id for where this originated,
    // and dyn_id is the id for this 'true' playlist or track, and can change. generally, dyn_id is preferred.
    source_id: Id,
    dyn_id: Id,
}
impl PlaylistMetadata {
    pub fn new(title: String, source_id: Id, dyn_id: Id) -> Self {
        Self {
            title,
            source_id,
            dyn_id,
        }
    }
    pub fn id(&self) -> &Id {
        &self.dyn_id
    }
}

/// Standard `Playlist` struct. Does NOT own its tracks.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Playlist {
    pub metadata: PlaylistMetadata,
    pub track_ids: Vec<Id>,
}

impl Playlist {
    pub fn new(metadata: PlaylistMetadata, track_ids: Vec<Id>) -> Self {
        Self {
            metadata,
            track_ids,
        }
    }
    pub fn id(&self) -> &Id {
        self.metadata.id()
    }
    pub fn length(&self) -> usize {
        self.track_ids.len()
    }
}

/// `Playlist` that owns all of its tracks. Does NOT change when global
/// tracklist is modified.
#[derive(Debug, Clone)]
pub struct OwnedPlaylist {
    pub metadata: PlaylistMetadata,
    pub tracks: Vec<Track>,
}
impl OwnedPlaylist {
    pub fn new(metadata: PlaylistMetadata, tracks: Vec<Track>) -> OwnedPlaylist {
        Self { metadata, tracks }
    }
    pub fn with_cache(
        metadata: PlaylistMetadata,
        track_ids: Vec<Id>,
        track_cache: &HashMap<Id, Track>,
    ) -> Self {
        let tracks = util::clone_tracks_from_cache(track_ids, &track_cache);
        Self { metadata, tracks }
    }
    pub fn unpack_to_playlist(self) -> (Playlist, Vec<Track>) {
        let track_ids = self.tracks.iter().map(|t| t.id().clone()).collect();
        let playlist = Playlist::new(self.metadata, track_ids);
        (playlist, self.tracks)
    }
    pub fn length(&self) -> usize {
        self.tracks.len()
    }
    pub fn contains_track(&self, track_id: &Id) -> bool {
        self.tracks
            .iter()
            .find(|track| track.id() == track_id)
            .is_some()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Track {
    pub title: String,
    pub length: Duration,
    pub artist: Artist,
    pub album_kind: AlbumKind,
    pub source_id: Id,
    pub dyn_id: Id,
    pub download_url: Url,
}

impl Track {
    pub fn from_playlist_track_json(ptj: PlaylistTrackJson) -> Self {
        let id = Id::new(Platform::Youtube, MediaType::Track, ptj.id);
        Self {
            title: ptj.title,
            length: Duration::from_secs(ptj.duration),
            artist: Artist::Community(ptj.channel),
            album_kind: AlbumKind::Unknown,
            source_id: id.clone(),
            dyn_id: id,
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
    pub source_id: Id,
    pub dyn_id: Id,
    pub artists: Vec<String>,
    pub img_url: Url,
}
impl Album {
    pub fn id(&self) -> &Id {
        &self.dyn_id
    }
}

#[derive(Debug, Deserialize)]
// created on playlist initialization
pub struct PlaylistTrackJson {
    url: Url,
    title: String,
    duration: u64,
    channel: String,
    pub playlist_id: String,
    id: String,
}

#[derive(Debug, Deserialize)]
// created on track download
pub struct TrackDownloadJson {}

#[derive(Debug)]
pub struct DownloadTrackJson {}

// Stores a `Track`'s 'metadata.' mostly just used for gui buttons to only redraw the button when important information changes.
#[derive(Debug, Hash, Clone)]
pub struct TrackDisplayMetadata {
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
    pub fn from_owned_playlist(playlist: &OwnedPlaylist) -> Self {
        Self::from_length(playlist.length() as u64)
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
    pub fn sort(&mut self) {
        self.index_order.sort_unstable();
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
    pub fn from_owned_playlist_ref(playlist: &OwnedPlaylist) -> Self {
        Self::new(
            TrackOrder::from_owned_playlist(&playlist),
            playlist.tracks.clone(),
        )
        .expect("Track order from playlist should have same length as playlist itself")
    }
    pub fn from_tracks_vec(tracks: Vec<Track>) -> Self {
        Self::new(TrackOrder::from_length(tracks.len() as u64), tracks)
            .expect("Track order from playlist should have same length as playlist itself")
    }
    pub fn iter(&self) -> impl Iterator<Item = &Track> {
        self.order
            .order()
            .iter()
            .map(|index| &self.tracks[*index as usize])
    }
    pub fn replace_tracks(&mut self, new_tracks: Vec<Track>) {
        self.tracks = Arc::new(new_tracks);
    }
    pub fn randomize_order(&mut self) {
        self.order.randomize();
    }
    pub fn sort(&mut self) {
        self.order.sort();
    }
}

pub struct PlaylistDownloadManager {
    tracklist: TrackList,
    playlist_id: Id,
    cancel_token: CancellationToken,
    stop_flag: Arc<AtomicBool>,
    start_pos_flag: Arc<AtomicU64>,
    restart_flag: Arc<AtomicBool>,
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
            start_pos_flag: Arc::new(AtomicU64::new(0)),
            restart_flag: Arc::new(AtomicBool::new(false)),
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
        musicbrainz_client: MusicBrainzClient,
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
            while let Some((id, line)) = map_r.recv().await {
                match line {
                    ExtractorLineOut::DownloadProgress(data) => {
                        gui_reply_stream_clone
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
        let start_pos_flag = Arc::clone(&self.start_pos_flag);
        let restart_flag = Arc::clone(&self.restart_flag);

        let async_block = async move {
            while internal_r.has_changed().unwrap_or(false) || restart_flag.load(Ordering::Relaxed)
            {
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

                // restart stop + restart flags
                stop_flag_clone.store(false, Ordering::Relaxed);
                restart_flag.store(false, Ordering::Relaxed);

                // run the playlist downloading logic
                println!("running playlist downloading logic lol");

                // if there's a custom start pos then use that
                let mut tracklist_iter = tracklist.iter();
                let start_pos = start_pos_flag.load(Ordering::Relaxed);
                if start_pos > 0 {
                    println!("Downloading playlist with custom index {start_pos}");
                    tracklist_iter.nth(start_pos as usize - 1);
                }

                for track in tracklist_iter {
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
                    let maybe_new_track = download::download_track(
                        &track,
                        file::util::track_dir_path().unwrap(),
                        &musicbrainz_client,
                        track.id().to_string(),
                        bin_apps.clone(),
                        &process_sender,
                        &map_t,
                        &playlist_sender_clone,
                    )
                    .await
                    .unwrap();

                    // update track logic
                    if let Some(track) = maybe_new_track {
                        println!("Found new match for track: {track:?}");
                        let _ = playlist_sender_clone
                            .send(PlaylistMessage::UpdateTrack {
                                playlist_id: None,
                                track,
                                restart_audio: false,
                                restart_download: false,
                            })
                            .await;
                    }

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

        self.restart_flag.store(true, Ordering::Relaxed);
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
    pub fn skip_to_index(&mut self, pos: u64) {
        if self.dead() {
            return;
        }
        if pos >= self.tracklist.order.length() as u64 {
            println!("failed to restart with start pos; greater than tracklist length");
            return;
        }

        self.start_pos_flag.store(pos, Ordering::Relaxed);
        self.restart();
    }
    fn dead(&self) -> bool {
        if self.dead {
            println!("cannot run methods on dead playlist manager");
        }
        self.dead
    }
}

pub struct PlaylistAudioManager {
    tracklist: Option<watch::Receiver<Option<TrackList>>>,
    playlist_id: Id,
    cancel_token: CancellationToken,
    restart_flag: Arc<AtomicBool>,
    internal_t: Option<watch::Sender<Option<TrackList>>>,
    current_track_id: Arc<Mutex<Option<Id>>>,
    current_pos: Arc<AtomicU64>,
    dead: bool,
    running: bool,
    start_index: Option<Arc<watch::Sender<Option<u64>>>>,
    audio_sender: Option<AudioSender>,
    playlist_sender: Option<PlaylistSender>,
    previous_until_valid_flag: Arc<AtomicBool>,
    stop_waiting_on_track_notify: Arc<Notify>,
    playing_flag: Arc<AtomicBool>,
    start_audio_looped: Arc<AtomicBool>,
    volume: Arc<AtomicF64>,
}
impl PlaylistAudioManager {
    pub fn new(playlist_id: Id) -> Self {
        Self {
            tracklist: None,
            playlist_id,
            cancel_token: CancellationToken::new(),
            restart_flag: Arc::new(AtomicBool::new(false)),
            previous_until_valid_flag: Arc::new(AtomicBool::new(false)),
            internal_t: None,
            dead: false,
            running: false,
            current_track_id: Arc::new(Mutex::new(None)),
            current_pos: Arc::new(AtomicU64::new(0)),
            audio_sender: None,
            start_index: None,
            playlist_sender: None,
            stop_waiting_on_track_notify: Arc::new(Notify::new()),
            playing_flag: Arc::new(AtomicBool::new(false)),
            start_audio_looped: Arc::new(AtomicBool::new(false)),
            volume: Arc::new(AtomicF64::new(1.0)),
        }
    }
    pub fn run(
        &mut self,
        tracklist: TrackList,
        // gui reply stream: directly audio progress updates, audio starts, and audio ends
        gui_progress_sender: mpsc::Sender<Message>,
        // playlist sender: directly gets playlist playing finish
        playlist_sender: mpsc::Sender<PlaylistMessage>,
        audio_sender: mpsc::Sender<AudioMessage>,
        autoplay_first_track: bool,
    ) {
        if self.dead() {
            return;
        }
        if self.running {
            println!("cannot run same playlist manager twice");
        }
        self.running = true;

        self.audio_sender = Some(audio_sender);
        self.playlist_sender = Some(playlist_sender.clone());

        // create the start index watch channel
        let (start_index_t, mut start_index_r) = watch::channel(None);
        let arc_start_index = Arc::new(start_index_t);
        self.start_index = Some(Arc::clone(&arc_start_index));

        // create mini task to map extractor lines to gui messages
        let (map_t, mut map_r) = mpsc::channel(100);
        let gui_progress_sender_clone = gui_progress_sender.clone();
        tokio::spawn(async move {
            while let Some((id, progress)) = map_r.recv().await {
                let _ = gui_progress_sender_clone
                    .send(Message::TrackAudioProgress { id, progress })
                    .await;
            }
        });

        // setup internal communication to the task at hand
        let (internal_t, mut internal_r) = watch::channel(None);
        // send first value
        internal_t.send(Some(tracklist)).unwrap();
        self.internal_t = Some(internal_t);
        self.tracklist = Some(internal_r.clone());

        // all the cloning
        let playlist_id = self.playlist_id.clone();
        let playlist_sender_clone = playlist_sender.clone();
        let gui_reply_stream_clone = gui_progress_sender.clone();
        let cancel_token = self.cancel_token.clone();
        let current_track_id = Arc::clone(&self.current_track_id);
        let audio_sender = self.audio_sender.clone().unwrap();
        let restart_flag = Arc::clone(&self.restart_flag);
        let current_pos_arc = Arc::clone(&self.current_pos);
        let previous_until_valid_arc = Arc::clone(&self.previous_until_valid_flag);
        let stop_waiting_on_track_notify = Arc::clone(&self.stop_waiting_on_track_notify);
        let playing_flag = self.playing_flag.clone();
        let volume_arc = Arc::clone(&self.volume);

        // spawn async process
        tokio::spawn(async move {
            let mut first_pass = true;
            while internal_r.has_changed().unwrap_or(false) || restart_flag.load(Ordering::Relaxed)
            {
                println!("starting new loop in audio player");
                let tracklist = {
                    let t = internal_r.borrow_and_update().clone();
                    if let Some(t) = t {
                        t
                    } else {
                        println!("tracklist was None, breaking");
                        break;
                    }
                };

                // reset restart flag
                restart_flag.store(false, Ordering::Relaxed);

                // run the playlist downloading logic
                println!("running playlist playing logic lol");
                let mut current_pos: i64 = -1;
                let playlist_length = tracklist.order.length() as u64;

                // check if a specific position was requested
                if start_index_r.has_changed().unwrap_or(false) {
                    println!("start index has changed");
                    // get value and skip to that point in the tracklist
                    let start_pos = start_index_r.borrow_and_update();
                    if let Some(pos) = *start_pos
                        && pos > 0
                    {
                        println!("start index changed; index: {pos}");
                        current_pos = pos as i64 - 1;
                    }
                }
                while current_pos < playlist_length as i64 - 1 {
                    // advance the pos
                    current_pos += 1;
                    current_pos_arc.store(current_pos as u64, Ordering::Relaxed);
                    // get the current track
                    let playlist_loc = tracklist.order.index_order[current_pos as usize];
                    let track = &tracklist.tracks[playlist_loc as usize];

                    println!("[Track] On track {}", track.title);

                    // check to see if a stop was requested
                    if restart_flag.load(Ordering::Relaxed) {
                        println!("breaking playlist playing");
                        break;
                    }
                    // check to see if this current track was already downloaded
                    let (downloaded_t, downloaded_r) = oneshot::channel();
                    // send the request
                    if let Err(_) = playlist_sender_clone
                        .send(PlaylistMessage::CheckTrackDownloaded {
                            id: track.id().clone(),
                            result_sender: downloaded_t,
                        })
                        .await
                    {
                        break;
                    }

                    if let Ok(result) = downloaded_r.await {
                        // check the result
                        let search_previous = previous_until_valid_arc.load(Ordering::Relaxed);
                        if !result {
                            if search_previous && current_pos == 0 {
                                // couldn't find downloaded track before; giving up
                                println!("Failed to find previous track that was downloaded");
                                previous_until_valid_arc.store(false, Ordering::Relaxed);
                            } else if search_previous {
                                // go up the tree and hope something is found (skipping 2 b/c on track start
                                // pos increments by one)
                                current_pos -= 2;
                                continue;
                            } else {
                                // check if this playlist is being downloaded
                                let (tx, rx) = oneshot::channel();
                                let _ = playlist_sender
                                    .send(PlaylistMessage::IfPlaylistDownloadingWait {
                                        playlist_id: playlist_id.clone(),
                                        track_id_to_wait: track.id().clone(),
                                        result_sender: tx,
                                    })
                                    .await;
                                let result = rx.await;
                                match result {
                                    Err(_) => {
                                        println!(
                                            "an error occured while checking playlist downloading status"
                                        );
                                        continue;
                                    }
                                    Ok(result) => match result {
                                        None => {
                                            // track is not downloaded; skip it
                                            // (and downloader is not active)
                                            println!(
                                                "Track {} skipped: not downloaded",
                                                track.title
                                            );
                                            continue;
                                        }
                                        Some(rx) => {
                                            // send request to playlist for this track to download next
                                            let (rt, rr) = oneshot::channel();

                                            let _ = playlist_sender
                                                .send(PlaylistMessage::SelectDownloadIndex {
                                                    playlist_id: playlist_id.clone(),
                                                    index: current_pos as u64,
                                                    result_sender: rt,
                                                })
                                                .await;
                                            let req_r = rr.await;
                                            if let Err(_) = req_r {
                                                println!(
                                                    "An error occured while waiting for track download request"
                                                );
                                            }

                                            println!(
                                                "Waiting for track: {} to download",
                                                track.title
                                            );
                                            // reset the notify
                                            stop_waiting_on_track_notify.notified().now_or_never();

                                            let result = tokio::select! {
                                                recv_result = rx => {
                                                    // do gross error mapping
                                                    recv_result.map_err(anyhow::Error::from).and_then(|inner| inner.map_err(anyhow::Error::from))
                                                },
                                                _ = stop_waiting_on_track_notify.notified() => {
                                                    Err(anyhow!("Download wait skipped"))
                                                }
                                            };
                                            match result {
                                                Ok(_) => {
                                                    // everything was ok; continue as normal
                                                }
                                                Err(_) => {
                                                    // something went wrong (ie. cancelled)
                                                    // continue
                                                    continue;
                                                }
                                            }
                                        }
                                    },
                                }
                            }
                        } else if search_previous {
                            println!("Found previous track that was downloaded");
                            previous_until_valid_arc.store(false, Ordering::Relaxed);
                        }
                    }

                    // Track Audio Start message
                    let _ = gui_reply_stream_clone
                        .send(Message::TrackAudioStart {
                            id: track.id().clone(),
                        })
                        .await;

                    println!("Playing track {}..", track.title);
                    // audio playing logic
                    let start_paused = first_pass && !autoplay_first_track;
                    // immediately change first pass
                    first_pass = false;
                    let audio_config =
                        AudioConfig::new(start_paused, volume_arc.load(Ordering::Relaxed));
                    let (end_t, end_r) = oneshot::channel();

                    let audio_message = AudioMessage::PlayAudio {
                        id: track.id().clone(),
                        audio_config,
                        progress_sender: map_t.clone(),
                        on_end: end_t,
                        on_loop: playlist_sender.clone(),
                        maybe_playlist_id: Some(playlist_id.clone()),
                    };

                    playing_flag.store(true, Ordering::Relaxed);

                    // send the request
                    let _ = audio_sender.send(audio_message).await;

                    // update the current track id so it can be controlled from the outside
                    {
                        let mut guard = current_track_id.lock();
                        *guard = Some(track.id().clone());
                    }

                    // play audio unless cancelled (audio mgr shut down)
                    tokio::select! {
                        _ = cancel_token.cancelled() => {
                            println!("Playlist audio manager cancelled");
                            break;
                        }
                        _ = end_r => {
                            println!("Track finished in mgr.");
                            // reset current track
                            let mut guard = current_track_id.lock();
                            *guard = None;
                        }
                    }

                    // this track ended
                    let _ = gui_reply_stream_clone
                        .send(Message::TrackAudioEnd {
                            id: track.id().clone(),
                            maybe_playlist_id: Some(playlist_id.clone()),
                        })
                        .await;

                    playing_flag.store(false, Ordering::Relaxed);
                }
            }
            // end of while loop; management finished
            let _ = playlist_sender
                .send(PlaylistMessage::PlaylistAudioManagementDone { id: playlist_id })
                .await;
        });
    }
    // Only cancel exists here, as there's no point in waiting for the audio to stop playing
    pub fn cancel(&mut self) {
        if self.dead() {
            return;
        }
        self.dead = true;

        println!("Cancelling manager..");
        self.stop_waiting_on_track_notify.notify_one();
        self.cancel_token.cancel();
    }
    pub fn get_playlist_id(&self) -> &Id {
        &self.playlist_id
    }
    pub fn restart(&mut self) {
        if self.dead() {
            return;
        }
        self.restart_flag.store(true, Ordering::Relaxed);
        self.stop_waiting_on_track_notify.notify_one();
        self.stop_current_track();
    }
    pub fn stop_current_track(&mut self) {
        if self.dead() {
            return;
        }

        if let Some(sender) = &self.audio_sender {
            let track_id = {
                let guard = self.current_track_id.lock();
                guard.clone()
            };
            if let Some(track_id) = track_id {
                let sender_clone = sender.clone();
                tokio::spawn(async move {
                    let (tx, _) = oneshot::channel();
                    let _ = sender_clone
                        .clone()
                        .send(AudioMessage::StopAudio {
                            id: track_id,
                            result: tx,
                        })
                        .await;
                });
            }
        }
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
    pub fn pause_current_track(&mut self) {
        if self.dead() {
            return;
        }

        if let Some(sender) = &self.audio_sender {
            // get the current track id if it exists
            let track_id: Option<Id> = {
                let guard = self.current_track_id.lock();
                guard.clone()
            };
            if let Some(track_id) = track_id {
                let sender_clone = sender.clone();
                tokio::spawn(async move {
                    let (tx, _) = oneshot::channel();
                    let _ = sender_clone
                        .clone()
                        .send(AudioMessage::PauseAudio {
                            id: track_id,
                            result: tx,
                        })
                        .await;
                });
            }
        }
    }
    pub fn resume_current_track(&mut self) {
        if self.dead() {
            return;
        }

        if let Some(sender) = &self.audio_sender {
            // get the current track id if it exists
            let track_id: Option<Id> = {
                let guard = self.current_track_id.lock();
                guard.clone()
            };
            if let Some(track_id) = track_id {
                let sender_clone = sender.clone();
                tokio::spawn(async move {
                    let (tx, _) = oneshot::channel();
                    let _ = sender_clone
                        .clone()
                        .send(AudioMessage::ResumeAudio {
                            id: track_id,
                            result: tx,
                        })
                        .await;
                });
            }
        }
    }
    pub fn skip_to_index(&mut self, index: u64) {
        if self.dead() {
            return;
        }

        // change the start index and 'restart' the playlist
        // check to make sure its in the bounds though
        let current_tracklist = {
            if let Some(tracklist_watch) = &self.tracklist {
                let maybe_tracklist = tracklist_watch.borrow().clone();
                if let Some(tracklist) = maybe_tracklist {
                    tracklist
                } else {
                    return;
                }
            } else {
                return;
            }
        };
        let length = current_tracklist.order.length() as u64;
        if index < length
            && let Some(start_pos) = &self.start_index
        {
            let _ = start_pos.send(Some(index));
            self.restart();
        } else if index >= length {
            // just end the playlist
            self.cancel();
        }
    }
    pub fn skip_current_track(&mut self) {
        // just end this track
        self.stop_current_track();
    }
    pub fn previous_current_track(&mut self) {
        // get the current index and skip to the previous one
        let mut current_pos = self.current_pos.load(Ordering::Relaxed);
        if current_pos == 0 {
            current_pos = 1
        }
        println!("going to pos: {}", current_pos - 1);
        // set the flag to not stop until a downloaded track is found (or the start)
        self.previous_until_valid_flag
            .store(true, Ordering::Relaxed);
        self.skip_to_index(current_pos - 1);
    }
    pub fn get_current_track(&self) -> Option<Track> {
        // try to find the current track being played
        let current_pos = self.current_pos.load(Ordering::Relaxed);
        if let Some(tracklist_watch) = &self.tracklist {
            let guard = tracklist_watch.borrow();
            if let Some(tracklist) = guard.as_ref() {
                return tracklist
                    .tracks
                    .get(tracklist.order.index_order[current_pos as usize] as usize)
                    .cloned();
            }
        }

        None
    }
    pub fn loaded_track(&self) -> bool {
        self.playing_flag.load(Ordering::Relaxed)
    }
    pub fn set_audio_loop(&self, loop_audio: bool) {
        // set the flag for future tracks
        self.start_audio_looped.store(loop_audio, Ordering::Relaxed);
    }
    /// updates the manager's internal volume. Note: it does not update
    /// any current track's volume.
    pub fn update_volume(&mut self, volume: f64) {
        self.volume.store(volume, Ordering::Relaxed);
    }
    fn dead(&self) -> bool {
        if self.dead {
            println!("cannot run methods on dead playlist manager");
        }
        self.dead
    }
}
