use std::collections::HashSet;

use tokio::sync::mpsc;

use crate::{
    service::{
        id::structs::Id,
        playlist::{
            enums::PlaylistInitStatus,
            structs::{Playlist, PlaylistMetadata, TrackDownloadData},
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
    TaskFinished(u64),
    // A playlist init task started. Provides the id and the receiver handle relevant to the task.
    PlaylistInitTaskStarted(u64, ReceiverHandle<Message>),
    // A playlist was selected to be loaded. Provides the selected playlist's metadata.
    PlaylistSelect(PlaylistMetadata),
    // A playlist that was selected received playlist data to render.
    PlaylistSelectAccepted(Playlist),
    // The list of tracks that was downloaded before the program started. Given: the list of track ids
    DownloadedTrackListReceived(HashSet<Id>),
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
    PlaylistInitStatus(PlaylistInitStatus),
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
    PlayTrack { playlist_id: Id },
    PauseTrack { playlist_id: Id },
    NextTrack { playlist_id: Id },
    LoopTrack { playlist_id: Id },
}

// represents each possible major page the gui can be
#[derive(Debug, Clone)]
pub enum Page {
    Home,
    Player,
}

pub enum PlayingState {
    Playing,
    Stopped,
}

// for app-wide messages (usually more important)
#[derive(Debug, Clone)]
pub enum EventMessage {
    InitialPlaylistsInitalized(Vec<PlaylistMetadata>),
    // A single track has downloaded. Given: the id of the track.
    // This is an EventMessage because it is generally independent of a playlist.
    TrackDownloadFinished { id: Id },
}

pub type EventSender = mpsc::Sender<EventMessage>;
