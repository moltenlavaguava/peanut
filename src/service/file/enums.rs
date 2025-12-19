use std::{io, path::PathBuf};

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
