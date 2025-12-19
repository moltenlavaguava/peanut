use crate::{
    service::{
        file::{FileSender, enums::FileMessage, structs::BinApps},
        playlist::download::initialize_playlist,
        process::ProcessSender,
    },
    util::{service::ServiceLogic, sync::EventSender},
};
use enums::PlaylistMessage;
use structs::Playlist;
use tokio::sync::{mpsc, oneshot};

mod download;
pub mod enums;
mod structs;

pub type PlaylistSender = mpsc::Sender<PlaylistMessage>;

/// Handles playlist management.
pub struct PlaylistService {
    event_sender: EventSender,
    file_sender: FileSender,
    process_sender: ProcessSender,
    playlists: Vec<Playlist>,
    bin_files: Option<BinApps>,
}

pub struct PlaylistFlags {
    pub event_sender: EventSender,
    pub file_sender: FileSender,
    pub process_sender: ProcessSender,
}

impl PlaylistService {
    pub fn new(flags: PlaylistFlags) -> Self {
        Self {
            event_sender: flags.event_sender,
            file_sender: flags.file_sender,
            process_sender: flags.process_sender,
            playlists: Vec::new(),
            bin_files: None,
        }
    }
}

#[async_trait::async_trait]
impl ServiceLogic<enums::PlaylistMessage> for PlaylistService {
    fn name(&self) -> &'static str {
        "PlaylistService"
    }
    async fn on_start(&mut self) -> anyhow::Result<()> {
        // startup logic
        // get the yt_dlp and ffmpeg file locations
        let (bin_tx, bin_rx) = oneshot::channel();
        let _ = self
            .file_sender
            .send(FileMessage::GetBinApps { reply: bin_tx })
            .await;
        let bin_files = bin_rx.await.unwrap();
        self.bin_files = Some(bin_files);

        Ok(())
    }
    async fn handle_message(&mut self, msg: enums::PlaylistMessage) {
        // message handling
        match msg {
            PlaylistMessage::InitializePlaylist { url } => {
                let bin_files_copy = self.bin_files.as_ref().unwrap().clone();
                let process_sender_copy = self.process_sender.clone();
                tokio::spawn(async move {
                    if let Ok(playlist) =
                        initialize_playlist(url, bin_files_copy, process_sender_copy).await
                    {
                        println!("playlist init succeeded");
                    } else {
                        println!("playlist init failed");
                    }
                });
            }
        }
    }
}
