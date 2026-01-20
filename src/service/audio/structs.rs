use std::{
    sync::{Arc, atomic::AtomicU64},
    time::Duration,
};

use atomic_float::AtomicF64;
use kira::sound::static_sound::StaticSoundHandle;
use parking_lot::Mutex;
use tokio::sync::oneshot;

use crate::service::{
    audio::enums::{AlbumKind, ExtractorConfidence, LoopPolicy},
    id::structs::Id,
    playlist::PlaylistSender,
};

#[derive(Debug)]
pub struct AudioConfig {
    start_paused: bool,
    volume: f64,
}
impl AudioConfig {
    pub fn new(start_paused: bool, volume: f64) -> Self {
        Self {
            start_paused,
            volume,
        }
    }
    pub fn start_paused(&self) -> bool {
        self.start_paused
    }
    pub fn volume(&self) -> f64 {
        self.volume
    }
}

// Small wrapper for audio handles; contains other information relevant to the handle.
pub struct AudioHandleWrapper {
    pub handle: Arc<Mutex<StaticSoundHandle>>,
    pub on_end: oneshot::Sender<anyhow::Result<()>>,
    pub on_loop: PlaylistSender,
    pub last_known_pos: Arc<AtomicF64>,
    pub loop_policy: LoopPolicy,
    pub audio_duration: Duration,
    pub seek_count: Arc<AtomicU64>,
    pub maybe_playlist_id: Option<Id>,
}

#[derive(Debug, Clone)]
pub struct AudioProgress {
    current: Duration,
    total: Duration,
}
impl AudioProgress {
    pub fn new(current: Duration, total: Duration) -> Self {
        Self { current, total }
    }
    // gives progress as a decimal (eg. 0.26)
    pub fn progress(&self) -> f32 {
        self.current().as_millis() as f32 / self.total().as_millis() as f32
    }
    pub fn current(&self) -> &Duration {
        &self.current
    }
    pub fn total(&self) -> &Duration {
        &self.total
    }
    pub fn update_progress(&mut self, progress: f32) {
        self.current = Duration::from_secs_f32(self.total().as_secs_f32() * progress);
    }
}

#[derive(Debug)]
pub struct YoutubeTitleMetadata {
    pub track_title: String,
    pub main_artist_string: String,
    pub extract_confidence: ExtractorConfidence,
}

// Information about track retreived from online databases
#[derive(Debug)]
pub struct UpdatedTrackData {
    pub title: String,
    pub album_kind: AlbumKind,
    pub artists: Vec<String>,
}
