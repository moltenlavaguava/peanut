use tokio::sync::mpsc;

use crate::{
    service::{gui::enums::EventSender, process::util::stream_process},
    util::service::ServiceLogic,
};
use enums::ProcessMessage;

pub mod enums;
pub mod structs;
mod util;

// easier types
pub type ProcessSender = mpsc::Sender<ProcessMessage>;

/// Handles file paths.
pub struct ProcessService {
    event_sender: EventSender,
}

impl ProcessService {
    pub fn new(event_sender: EventSender) -> Self {
        Self { event_sender }
    }
}

#[async_trait::async_trait]
impl ServiceLogic<ProcessMessage> for ProcessService {
    fn name(&self) -> &'static str {
        "ProcessService"
    }
    async fn handle_message(&mut self, msg: ProcessMessage) {
        match msg {
            ProcessMessage::SpawnProcess {
                cmd,
                args,
                output_stream,
            } => {
                tokio::spawn(async move {
                    stream_process(cmd, args, output_stream).await;
                });
            }
        }
    }
}
