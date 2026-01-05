use std::process::ExitStatus;

use strum_macros::{Display, EnumString};
use tokio::sync::{mpsc, oneshot};
use url::Url;

use crate::service::{gui::enums::TaskResponse, playlist::structs::Playlist};

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
}

#[derive(Debug, EnumString, Display, PartialEq, Eq, Hash, Clone)]
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
    DownloadProgress(f64),
    DownloadTrackData(DownloadTrackJson),
    PlaylistInitDone(String),
    Exit(ExitStatus),
    Standard(String),
    Error(String),
}

#[derive(Debug, Clone)]
pub enum PlaylistInitStatus {
    Progress { current: u32, total: u32 },
    Complete { title: String },
    Fail,
    Duplicate { title: String },
}

#[derive(Debug)]
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
