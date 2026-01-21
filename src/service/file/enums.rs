use std::{io, path::PathBuf};

use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use tokio::sync::oneshot;

use super::structs::BinApps;

pub enum FileMessage {
    ReadFile {
        reply: oneshot::Sender<Result<String, io::ErrorKind>>,
        path_buf: PathBuf,
    },
    GetBinApps {
        reply: oneshot::Sender<BinApps>,
    },
}

#[derive(Debug, Deserialize, Serialize, EnumString)]
pub enum SizeUnit {
    #[strum(serialize = "KiB")]
    Kibibyte,
    #[strum(serialize = "MiB")]
    Mebibyte,
    #[strum(serialize = "KB")]
    Kilobyte,
    #[strum(serialize = "MB")]
    Megabyte,
}

#[derive(Debug, Clone, Hash)]
pub enum TrackDownloadState {
    NotDownloaded,
    Downloading,
    Downloaded,
}
