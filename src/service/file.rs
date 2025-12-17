use std::{io, path::PathBuf};

use tokio::{fs, sync::oneshot};

use crate::util::{service::ServiceLogic, sync::EventSender};

pub enum FileMessage {
    ReadFile {
        reply: oneshot::Sender<Result<String, io::ErrorKind>>,
        path_buf: PathBuf,
    },
}

/// Handles file paths.
pub struct FileService {
    event_sender: EventSender,
}

impl FileService {
    pub fn new(event_sender: EventSender) -> Self {
        Self { event_sender }
    }
}

#[async_trait::async_trait]
impl ServiceLogic<FileMessage> for FileService {
    fn name(&self) -> &'static str {
        "FileService"
    }
    async fn handle_message(&mut self, msg: FileMessage) {
        match msg {
            FileMessage::ReadFile { reply, path_buf } => {
                let file_text = fs::read_to_string(path_buf).await;
                let _ = reply.send(file_text.map_err(|err| err.kind()));
            }
        }
    }
}
