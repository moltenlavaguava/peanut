use std::{io, path::PathBuf};

use tokio::sync::oneshot;

use crate::util::{service::ServiceLogic, sync::EventSender};

pub enum PlaylistMessage {}

/// Handles file paths.
pub struct PlaylistService {
    event_sender: EventSender,
}

impl PlaylistService {
    pub fn new(event_sender: EventSender) -> Self {
        Self { event_sender }
    }
}

#[async_trait::async_trait]
impl ServiceLogic<PlaylistMessage> for PlaylistService {
    fn name(&self) -> &'static str {
        "FileService"
    }
    async fn handle_message(&mut self, msg: PlaylistMessage) {}
}
