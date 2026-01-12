use std::{collections::HashSet, process::ExitStatus};

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use tokio::sync::{mpsc, oneshot};
use url::Url;

use crate::service::{
    gui::enums::Message,
    id::structs::Id,
    playlist::structs::{Playlist, PlaylistMetadata, TrackDownloadData, TrackDownloadJson},
};

use super::structs::PlaylistTrackJson;

pub enum PlaylistMessage {
    InitializePlaylist {
        url: Url,
        reply_stream: oneshot::Sender<mpsc::Receiver<Message>>,
    },
    PlaylistInitDone {
        playlist: Playlist,
        result_sender: oneshot::Sender<anyhow::Result<()>>,
    },
    RequestPlaylist {
        id: Id,
        result_sender: oneshot::Sender<Option<Playlist>>,
    },
    DownloadPlaylist {
        id: Id,
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
    },
    CheckTrackDownloaded {
        id: Id,
        result_sender: oneshot::Sender<bool>,
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
