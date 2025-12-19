use tokio::{fs, sync::mpsc};

use crate::util::{service::ServiceLogic, sync::EventSender};
use enums::FileMessage;
use structs::BinApps;

pub mod enums;
pub mod structs;
mod util;

// easier types
pub type FileSender = mpsc::Sender<FileMessage>;

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
            FileMessage::GetBinApps { reply } => {
                let _ = reply.send(util::get_bin_app_paths());
            }
        }
    }
}
