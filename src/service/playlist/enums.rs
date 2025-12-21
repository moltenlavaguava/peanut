use std::process::ExitStatus;

use url::Url;

use super::structs::{InitJsonOutput, DownloadJsonOutput};

pub enum PlaylistMessage {
    InitializePlaylist { url: Url },
}

pub enum TrackSource {
    Youtube {},
}

#[derive(Debug)]
pub enum DownloadMessage {
    InitProgress {
        current: u32,
        total: u32,
    },
    InitData(InitJsonOutput),
    DownloadProgress(f64),
    DownloadData(DownloadJsonOutput),
    Exit(ExitStatus),
    Standard(String),
    Error(String),
}