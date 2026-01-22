use std::{collections::HashSet, process::ExitStatus};

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use tokio::sync::{mpsc, oneshot};
use url::Url;

use crate::service::{
    audio::enums::LoopPolicy,
    gui::enums::Message,
    id::structs::Id,
    playlist::structs::{
        Album, OwnedPlaylist, PlaylistMetadata, Track, TrackDownloadData, TrackDownloadJson,
        TrackList,
    },
};

use super::structs::PlaylistTrackJson;

pub enum PlaylistMessage {
    InitializePlaylist {
        url: Url,
        reply_stream: oneshot::Sender<mpsc::Receiver<Message>>,
    },
    PlaylistInitDone {
        owned_playlist: OwnedPlaylist,
        result_sender: oneshot::Sender<anyhow::Result<()>>,
    },
    // Returns a new (organized) tracklist from the given playlist id.
    RequestOwnedPlaylist {
        id: Id,
        result_sender: oneshot::Sender<Option<OwnedPlaylist>>,
    },
    DownloadPlaylist {
        id: Id,
        tracklist: TrackList,
        reply_stream: oneshot::Sender<mpsc::Receiver<Message>>,
    },
    CancelDownloadPlaylist {
        id: Id,
        result_sender: oneshot::Sender<anyhow::Result<()>>,
    },
    PlaylistDownloadDone {
        success: bool,
        id: Id,
    },
    GetDownloadedTracks {
        result_sender: oneshot::Sender<HashSet<Id>>,
    },
    TrackDownloadDone {
        id: Id,
        success: bool,
    },
    CheckTrackDownloaded {
        id: Id,
        result_sender: oneshot::Sender<bool>,
    },
    ShufflePlaylist {
        playlist_id: Id,
        tracklist: Option<TrackList>,
        result_sender: oneshot::Sender<TrackList>,
    },
    OrganizePlaylist {
        playlist_id: Id,
        tracklist: Option<TrackList>,
        result_sender: oneshot::Sender<TrackList>,
    },
    PlaylistAudioManagementDone {
        id: Id,
    },
    PlayPlaylist {
        id: Id,
        tracklist: Option<TrackList>,
        // Sends individual track progress updates and when the playlist finishes.
        data_sender: mpsc::Sender<Message>,
    },
    PauseCurrentTrack {
        playlist_id: Id,
        result_sender: oneshot::Sender<anyhow::Result<()>>,
    },
    ResumeCurrentTrack {
        playlist_id: Id,
        seek_location: Option<f32>,
        result_sender: oneshot::Sender<anyhow::Result<()>>,
    },
    SkipCurrentTrack {
        playlist_id: Id,
        result_sender: oneshot::Sender<anyhow::Result<()>>,
    },
    PreviousCurrentTrack {
        playlist_id: Id,
        result_sender: oneshot::Sender<anyhow::Result<()>>,
    },
    // Used by playlist audio managers. used to check if the playlist is currently downloading,
    // and if it is, then waits for its respective track to download.
    IfPlaylistDownloadingWait {
        playlist_id: Id,
        track_id_to_wait: Id,
        result_sender: oneshot::Sender<Option<oneshot::Receiver<anyhow::Result<()>>>>,
    },
    // Selects the given index in the given playlist manager to download next.
    // (and continuing down)
    SelectDownloadIndex {
        playlist_id: Id,
        index: u64,
        result_sender: oneshot::Sender<anyhow::Result<()>>,
    },
    // Invoked by the gui. Selects the track for playing in the audio mgr and downloading in the download mgr, if appropariate.
    SelectPlaylistIndex {
        playlist_id: Id,
        track_index: u64,
        result_sender: oneshot::Sender<anyhow::Result<()>>,
    },
    SeekTrackAudioInPlaylist {
        playlist_id: Id,
        percentage: f64,
        result_sender: oneshot::Sender<anyhow::Result<()>>,
    },
    SetPlaylistLoopPolicy {
        playlist_id: Id,
        policy: LoopPolicy,
        result_sender: oneshot::Sender<anyhow::Result<()>>,
    },
    TrackLooped {
        maybe_playlist_id: Option<Id>,
        track_id: Id,
    },
    UpdateGlobalVolume {
        volume: f64,
        result_sender: oneshot::Sender<anyhow::Result<()>>,
    },
    UpdateTrack {
        // Provide playlist id if the modification is for that playlist only.
        // Otherwise don't provide one
        playlist_id: Option<Id>,
        track: Track,
        restart_audio: bool,
        restart_download: bool,
    },
    AlbumDataRetreived {
        album: Album,
    },
    AlbumDownloaded {
        album: Album,
    },
}

#[derive(Debug, EnumString, Display, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub enum MediaType {
    #[strum(serialize = "pl")]
    Playlist,

    #[strum(serialize = "tr")]
    Track,

    #[strum(serialize = "al")]
    Album,
}

#[derive(Debug)]
pub enum ExtractorLineOut {
    InitProgress { current: u32, total: u32 },
    InitTrackData(PlaylistTrackJson),
    DownloadTrackData(TrackDownloadJson),
    DownloadProgress(TrackDownloadData),
    PlaylistInitDone(String),
    Exit(ExitStatus),
    Standard(String),
    Error(String),
}

#[derive(Debug, Clone)]
pub enum PlaylistInitStatus {
    Progress { current: u32, total: u32 },
    Complete(PlaylistMetadata),
    Fail,
    Duplicate(PlaylistMetadata),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Artist {
    Community(String),
    Official(Vec<String>),
}
impl Artist {
    pub fn artist(self) -> String {
        match self {
            Self::Official(artist_list) => {
                // concat list separated by commas
                artist_list.join(", ")
            }
            Self::Community(artist) => artist,
        }
    }
}

pub enum ExtractorContext {
    Initialize,
    Download,
}

pub enum DownloadEndType {
    Cancelled,
    Finished,
    Stopped,
}

pub enum PlaylistDownloadEventType {
    TrackDownloadStart { id: Id },
    TrackDownloadEnd { id: Id },
    ExtractorLineOut { id: Id, line: ExtractorLineOut },
    PlaylistDownloadEnd { playlist_id: Id, cancelled: bool },
}
