use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    service::{
        audio::{enums::LoopPolicy, structs::AudioProgress},
        gui::enums::{DownloadState, EventMessage, Message, Page, PlayingState},
        id::structs::Id,
        playlist::{
            PlaylistSender,
            structs::{Album, OwnedPlaylist, PlaylistMetadata, Track, Tracklist},
        },
    },
    util::sync::ReceiverHandle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlaylistInitId(u64);

#[derive(Clone)]
pub struct Counter {
    n: u64,
}
impl Counter {
    fn new() -> Counter {
        Self { n: 0 }
    }
    fn next(&mut self) -> u64 {
        self.n += 1;
        self.n
    }
}

#[derive(Clone)]
pub struct IdCounter {
    counter: Counter,
}
impl IdCounter {
    pub fn new() -> Self {
        Self {
            counter: Counter::new(),
        }
    }
    pub fn next(&mut self) -> TaskId {
        TaskId(self.counter.next())
    }
}

#[derive(Clone)]
pub struct PlaylistInitIdCounter {
    counter: Counter,
}
impl PlaylistInitIdCounter {
    pub fn new() -> Self {
        Self {
            counter: Counter::new(),
        }
    }
    pub fn next(&mut self) -> PlaylistInitId {
        PlaylistInitId(self.counter.next())
    }
}

// organizational structs for app state
pub struct GuiCommunication {
    pub playlist_sender: PlaylistSender,
    pub active_tasks: HashMap<TaskId, ReceiverHandle<Message>>,
    pub event_bus: ReceiverHandle<EventMessage>,
}
pub struct GuiSettings {
    pub volume: f64,
}
pub struct GuiManagement {
    pub id_counter: IdCounter,
    pub playlist_init_id_counter: PlaylistInitIdCounter,
    pub current_page: Page,
}
#[derive(Default)]
pub struct HomePlaylistsWidgetData {
    pub search_text: String,
    pub scrolling_offset: f32,
}
#[derive(Default)]
pub struct HomeTracksWidgetData {
    pub scrolling_offset: f32,
}
#[derive(Default)]
pub struct HomeAlbumsWidgetData {
    pub scrolling_offset: f32,
}

pub struct GeneralCache {
    // Track caching
    pub downloaded_tracks: HashSet<Id>,
    pub downloading_tracks: HashSet<Id>,
    pub all_tracks: Vec<Track>,

    // Album caching
    pub all_albums: Vec<Album>,

    // Playlist caching
    pub recent_playlists: VecDeque<PlaylistMetadata>,
    pub all_playlist_metadata: Vec<PlaylistMetadata>,
}
pub struct PlaylistRenderData {
    pub current_track: Option<Track>,
    pub playing_track_progress: AudioProgress,
    pub playing_track_loop_policy: LoopPolicy,
    pub owned_playlist: OwnedPlaylist,
    pub current_tracklist: Tracklist,
    pub playing_state: PlayingState,
    pub download_state: DownloadState,
    pub scroll_offset: f32,
    pub track_search_text: String,
}
pub struct PlaylistInitData {
    pub platform_display_id: Option<String>,
    pub current_init_track_count: Option<u32>,
    pub total_track_count: Option<u32>,
    pub name: Option<String>,
}
