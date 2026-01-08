use std::{collections::HashMap, sync::Arc};

use crate::{
    service::{
        file::{self, structs::BinApps},
        gui::enums::{EventMessage, EventSender, TaskResponse},
        id::structs::Id,
        playlist::{download::initialize_playlist, structs::PlaylistMetadata},
        process::ProcessSender,
    },
    util::service::ServiceLogic,
};
use anyhow::anyhow;
use enums::PlaylistMessage;
use structs::Playlist;
use tokio::{
    fs,
    sync::{mpsc, oneshot},
};

mod download;
pub mod enums;
pub mod structs;
mod util;

pub type PlaylistSender = mpsc::Sender<PlaylistMessage>;

/// Handles playlist management.
pub struct PlaylistService {
    event_sender: EventSender,
    process_sender: ProcessSender,
    playlist_sender: PlaylistSender,
    // Store playlists in an arc to make 'managing' them easier
    playlists: HashMap<Id, Arc<Playlist>>,
    bin_files: Option<BinApps>,
}

pub struct PlaylistFlags {
    pub event_sender: EventSender,
    pub process_sender: ProcessSender,
    pub playlist_sender: PlaylistSender,
}

impl PlaylistService {
    pub fn new(flags: PlaylistFlags) -> Self {
        Self {
            event_sender: flags.event_sender,
            process_sender: flags.process_sender,
            playlists: HashMap::new(),
            bin_files: None,
            playlist_sender: flags.playlist_sender,
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
        let bin_files = file::util::get_bin_app_paths();
        self.bin_files = Some(bin_files);

        // load existing playlists
        let playlists = file::util::load_saved_playlists().await.unwrap();
        self.playlists = playlists;
        self.event_sender
            .send(EventMessage::InitialPlaylistsInitalized(
                self.playlists
                    .values()
                    .map(|playlist| PlaylistMetadata {
                        title: playlist.title.clone(),
                        id: playlist.id().clone(),
                    })
                    .collect(),
            ))
            .await
            .unwrap();

        Ok(())
    }
    async fn handle_message(&mut self, msg: enums::PlaylistMessage) {
        // message handling
        match msg {
            PlaylistMessage::InitializePlaylist { url, reply_stream } => {
                let bin_files_copy = self.bin_files.as_ref().unwrap().clone();
                let process_sender_copy = self.process_sender.clone();
                let playlist_sender_copy = self.playlist_sender.clone();
                tokio::spawn(async move {
                    // create channel to send info (progress updates) back through
                    let (t_init_status, r_init_status) = mpsc::channel(100);
                    reply_stream.send(r_init_status).unwrap();

                    if let Ok(playlist) = initialize_playlist(
                        url,
                        bin_files_copy,
                        process_sender_copy,
                        &t_init_status,
                    )
                    .await
                    {
                        // before playlist is sent, copy title + id to send to gui in case of success
                        let playlist_title = playlist.title.clone();
                        let playlist_id = playlist.id().clone();
                        let playlist_metadata = PlaylistMetadata {
                            title: playlist_title,
                            id: playlist_id,
                        };
                        // check to see if playlist is duplicate or not
                        let (tx, rx) = oneshot::channel();
                        playlist_sender_copy
                            .send(PlaylistMessage::PlaylistInitDone {
                                playlist,
                                result_sender: tx,
                            })
                            .await
                            .unwrap();
                        if let Err(_) = rx.await.unwrap() {
                            t_init_status
                                .send(TaskResponse::PlaylistInitStatus(
                                    enums::PlaylistInitStatus::Duplicate(playlist_metadata),
                                ))
                                .await
                                .unwrap();
                        } else {
                            t_init_status
                                .send(TaskResponse::PlaylistInitStatus(
                                    enums::PlaylistInitStatus::Complete(playlist_metadata),
                                ))
                                .await
                                .unwrap();
                        }
                    } else {
                        println!("playlist init failed");
                        t_init_status
                            .send(TaskResponse::PlaylistInitStatus(
                                enums::PlaylistInitStatus::Fail,
                            ))
                            .await
                            .unwrap();
                    }
                });
            }
            PlaylistMessage::PlaylistInitDone {
                playlist,
                result_sender,
            } => {
                // check if playlist is duplicate. otherwise, add to hashmap + save to file
                if self.playlists.contains_key(&playlist.id()) {
                    result_sender
                        .send(Err(anyhow!("Duplicate playlist")))
                        .unwrap();
                } else {
                    // get playlist json
                    let playlist_json = serde_json::to_string_pretty(&playlist).unwrap();
                    println!("playlist id in string: {}", playlist.id().to_string());
                    // write to file
                    let pth = file::util::playlist_file_path_from_id(&playlist.id());
                    println!("{pth:?}");
                    fs::write(pth.unwrap(), playlist_json).await.unwrap();

                    // insert playlist into cache
                    self.playlists
                        .insert(playlist.id().clone(), Arc::new(playlist));
                    result_sender.send(Ok(())).unwrap();
                }
            }
            PlaylistMessage::RequestPlaylist { id, result_sender } => {
                if let Some(playlist) = self.playlists.get(&id) {
                    result_sender.send(Some(playlist.as_ref().clone())).unwrap();
                } else {
                    result_sender.send(None).unwrap()
                }
            }
        }
    }
}
