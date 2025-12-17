use std::{io, path::PathBuf};

use tokio::sync::oneshot;

pub enum FileMessage {
    ReadFile {
        reply: oneshot::Sender<Result<String, io::ErrorKind>>,
        path_buf: PathBuf,
    },
}
