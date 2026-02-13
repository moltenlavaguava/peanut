use std::collections::{HashMap, HashSet};

use iced::{Theme, widget::scrollable::Viewport};
use tokio::sync::mpsc;

use crate::{
    service::{
        audio::structs::AudioProgress,
        gui::structs::{PlaylistInitId, TaskId},
        id::structs::Id,
        playlist::{
            enums::PlaylistInitStatus,
            structs::{
                Album, OwnedPlaylist, PlaylistMetadata, Track, TrackDownloadData, Tracklist,
            },
        },
    },
    util::sync::ReceiverHandle,
};

#[derive(Debug, Clone)]
pub enum Message {
    // Playlist url text box edited. Provides text of box.
    PlaylistTextEdit(String),
    // Playlist url button submitted.
    PlaylistURLSubmit,
    // General event received. Provides message.
    EventRecieved(EventMessage),
    // Event bus closed.
    EventBusClosed,
    // Task finished. Provides id.
    TaskFinished(TaskId),
    TrackSearchTextEdit {
        playlist_id: Id,
        search_text: String,
    },
    // A playlist init task started. Provides the id and the receiver handle relevant to the task.
    PlaylistInitTaskStarted(TaskId, PlaylistInitId, ReceiverHandle<Message>),
    // A playlist was selected to be loaded. Provides the selected playlist's metadata.
    PlaylistSelect(PlaylistMetadata),
    // A playlist that was selected received `OwnedPlaylist` data to render.
    PlaylistSelectAccepted(OwnedPlaylist),
    // The list of tracks that was downloaded before the program started. Given: the list of track ids
    DownloadedTracklistReceived(HashSet<Id>),
    // A playlist download request succeeded and caused a playlist to start downloading. Provided: the playlist Id and its ReceiverHandle for information.
    DownloadPlaylistStarted {
        id: Id,
        receiver_handle: ReceiverHandle<Message>,
    },
    // A playlist download request was received and started, but it hasn't actually stopped yet. Provided: the playlist id.
    PlaylistDownloadCancelStarted {
        id: Id,
    },
    // fired when new track info is received for a playlist init
    PlaylistInitStatus {
        status: PlaylistInitStatus,
        id: PlaylistInitId,
    },
    // A single track download started. Given: the id of the track.
    TrackDownloadStarted {
        id: Id,
    },
    // The status for a single track download updated. Provided: the id of the track and the download data.
    TrackDownloadStatus {
        id: Id,
        data: TrackDownloadData,
    },
    // A playlist download ended. Provided: the playlist Id.
    DownloadPlaylistEnded {
        id: Id,
    },
    // A playlist tracklist updated. Provided: the playlist id and the list.
    PlaylistOrderUpdated {
        id: Id,
        tracklist: Tracklist,
    },
    // A track's audio progressed. Provided: the track id and the progress.
    TrackAudioProgress {
        id: Id,
        maybe_playlist_id: Option<Id>,
        progress: AudioProgress,
    },
    // A track started playing its audio.
    TrackAudioStart {
        id: Id,
        maybe_playlist_id: Option<Id>,
        start_paused: bool,
    },
    // A track finished playing its audio.
    TrackAudioEnd {
        id: Id,
        maybe_playlist_id: Option<Id>,
    },
    TrackAudioResumeResult {
        playlist_id: Id,
    },
    TrackAudioPauseResult {
        playlist_id: Id,
    },
    TrackAudioSkipResult {
        playlist_id: Id,
    },
    TrackAudioPreviousResult {
        playlist_id: Id,
    },
    // A generic message for when a task starts. Provided: the handle for the task.
    TaskStarted {
        handle: ReceiverHandle<Message>,
    },
    PlayPlaylistEnded {
        playlist_id: Id,
    },
    PlayTrackResult {
        playlist_id: Option<Id>,
    },
    SetPlaylistLoopPolicyResult {
        playlist_id: Id,
    },
    TrackLooped {
        maybe_playlist_id: Option<Id>,
        track_id: Id,
    },
    TracklistScrolled {
        playlist_id: Id,
        scrollable_viewport: Viewport,
    },
    HomeTracksScrolled {
        scrollable_viewport: Viewport,
    },
    HomeAlbumsScrolled {
        scrollable_viewport: Viewport,
    },
    HomePlaylistsScrolled {
        scrollable_viewport: Viewport,
    },
    ThemeUpdated {
        theme: Theme,
    },
    SetGlobalVolumeResult,
    // An action was performed. Usually occurs when the user triggers something.
    Action(Action),
    // A special message for when nothing should happen
    None,
}

#[derive(Debug, Clone)]
pub enum Action {
    // In the player menu, the home button was activated.
    Home,
    DownloadPlaylist { playlist_id: Id },
    StopPlaylistDownload { playlist_id: Id },
    OrganizePlaylist { playlist_id: Id },
    ShufflePlaylist { playlist_id: Id },
    PreviousTrack { playlist_id: Id },
    ResumeTrack { playlist_id: Id },
    PauseTrack { playlist_id: Id },
    NextTrack { playlist_id: Id },
    LoopTrack { playlist_id: Id },
    PlayTrack { playlist_id: Id, track_index: u64 },
    SeekAudio { playlist_id: Id, progress: f32 },
    StopSeekingAudio { playlist_id: Id },
    SetVolume { volume: f64 },
}

// represents each possible major page the gui can be
#[derive(Debug, Clone)]
pub enum Page {
    Home,
    Player { playlist_id: Id },
}

// represents possible downloading states
#[derive(Debug, Clone)]
pub enum DownloadState {
    Idle,
    Downloding,
    StopPending,
}

#[derive(Debug, Clone)]
pub enum PlayingState {
    Playing,
    Paused,
    Seeking,
    Unloaded,
    None,
}

// for app-wide messages (usually more important)
#[derive(Debug, Clone)]
pub enum EventMessage {
    InitialPlaylistsInitalized(Vec<PlaylistMetadata>),
    DownloadedAlbumsReceived(HashMap<Id, Album>),
    // A single track has downloaded. Given: the id of the track.
    // This is an EventMessage because it is generally independent of a playlist.
    TrackDownloadFinished {
        id: Id,
        success: bool,
    },
    AlbumDataDownloaded {
        album: Album,
    },
    TrackUpdated {
        track: Track,
    },
    TrackCacheUpdated {
        tracks_added: Option<HashMap<Id, Track>>,
        tracks_removed: Option<Vec<Id>>,
    },
}

pub type EventSender = mpsc::Sender<EventMessage>;
