use std::process::ExitStatus;

use strum_macros::{Display, EnumString};
use tokio::sync::{mpsc, oneshot};
use url::Url;

use crate::service::gui::enums::TaskResponse;

use super::structs::{DownloadJsonOutput, InitJsonOutput};

pub enum PlaylistMessage {
    InitializePlaylist {
        url: Url,
        reply_stream: oneshot::Sender<mpsc::Receiver<TaskResponse>>,
    },
}

#[derive(Debug, EnumString, Display)]
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
    InitData(InitJsonOutput),
    DownloadProgress(f64),
    DownloadData(DownloadJsonOutput),
    Exit(ExitStatus),
    Standard(String),
    Error(String),
}

#[derive(Debug, Clone)]
pub enum PlaylistInitStatus {
    Progress { current: u32, total: u32 },
    Complete,
    Fail,
}
