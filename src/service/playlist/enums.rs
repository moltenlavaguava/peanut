use std::process::ExitStatus;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use tokio::sync::{mpsc, oneshot};
use url::Url;

use crate::service::{
    gui::enums::TaskResponse,
    id::structs::Id,
    playlist::structs::{Playlist, PlaylistMetadata, TrackDownloadJson},
};

use super::structs::{DownloadTrackJson, PlaylistTrackJson};

pub enum PlaylistMessage {
    InitializePlaylist {
        url: Url,
        reply_stream: oneshot::Sender<mpsc::Receiver<TaskResponse>>,
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
        reply_stream: oneshot::Sender<mpsc::Receiver<TaskResponse>>,
    },
    CancelDownloadPlaylist {
        id: Id,
        result_sender: oneshot::Sender<anyhow::Result<()>>,
    },
    PlaylistDownloadDone {
        success: bool,
        id: Id,
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
    // eta: min::sec
    DownloadProgress { progress: f32, download_size: f32, download_speed: f32, eta: (u64, u64) },
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