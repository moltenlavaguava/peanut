use tokio::fs;

use crate::util::{service::ServiceLogic, sync::EventSender};

pub mod enums;

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
impl ServiceLogic<enums::FileMessage> for FileService {
    fn name(&self) -> &'static str {
        "FileService"
    }
    async fn handle_message(&mut self, msg: enums::FileMessage) {
        match msg {
            enums::FileMessage::ReadFile { reply, path_buf } => {
                let file_text = fs::read_to_string(path_buf).await;
                let _ = reply.send(file_text.map_err(|err| err.kind()));
            }
        }
    }
}
