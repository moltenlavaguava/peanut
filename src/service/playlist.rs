use std::collections::{HashMap, HashSet};

use crate::{
    service::{
        file::{self, structs::BinApps},
        gui::enums::{EventMessage, EventSender, Message},
        id::structs::Id,
        playlist::{
            download::initialize_playlist,
            structs::{PlaylistDownloadManager, PlaylistMetadata, TrackList},
        },
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
    playlists: HashMap<Id, Playlist>,
    bin_files: Option<BinApps>,
    // cache downloaded tracks to prevent re-downloading
    downloaded_tracks: HashSet<Id>,
    // Contains gui listener as well to send notifications back
    download_managers: HashMap<Id, (PlaylistDownloadManager, mpsc::Sender<Message>)>,
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
            download_managers: HashMap::new(),
            downloaded_tracks: HashSet::new(),
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

        // cache downloaded tracks
        let downloaded_tracks = file::util::get_downloaded_tracks().await.unwrap();
        self.downloaded_tracks = downloaded_tracks;

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
                                .send(Message::PlaylistInitStatus(
                                    enums::PlaylistInitStatus::Duplicate(playlist_metadata),
                                ))
                                .await
                                .unwrap();
                        } else {
                            t_init_status
                                .send(Message::PlaylistInitStatus(
                                    enums::PlaylistInitStatus::Complete(playlist_metadata),
                                ))
                                .await
                                .unwrap();
                        }
                    } else {
                        println!("playlist init failed");
                        t_init_status
                            .send(Message::PlaylistInitStatus(enums::PlaylistInitStatus::Fail))
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
                    self.playlists.insert(playlist.id().clone(), playlist);
                    result_sender.send(Ok(())).unwrap();
                }
            }
            PlaylistMessage::RequestTracklist { id, result_sender } => {
                if let Some(playlist) = self.playlists.get(&id) {
                    let tracklist = TrackList::from_playlist_ref(&playlist);
                    result_sender.send(Some(tracklist)).unwrap();
                } else {
                    result_sender.send(None).unwrap()
                }
            }
            PlaylistMessage::DownloadPlaylist {
                id,
                reply_stream,
                tracklist,
            } => {
                // first, check to see if there's already a current downloading playlist.
                // if there is, then do nothing.
                if !self.download_managers.is_empty() {
                    println!("A playlist is already downloading; doing nothing");
                    return;
                }
                // setup reply channel
                let (reply_t, reply_r) = mpsc::channel(100);
                reply_stream.send(reply_r).unwrap();
                // setup and start a new download manager
                // locate the playlist to download
                let playlist = self.playlists.get(&id);
                if let None = playlist {
                    println!("Playlist was somehow none when downloading; returning");
                    return;
                }
                let playlist = playlist.unwrap();
                let playlist_sender = self.playlist_sender.clone();
                let process_sender = self.process_sender.clone();
                let bin_apps = self.bin_files.clone().unwrap();

                let mut manager = PlaylistDownloadManager::new(tracklist, playlist.id().clone());
                manager.run(reply_t.clone(), playlist_sender, process_sender, bin_apps);

                self.download_managers.insert(id, (manager, reply_t));
            }
            PlaylistMessage::CancelDownloadPlaylist { id, result_sender } => {
                println!("Cancelling playlist?");
                if let Some((mgr, _gui_reply_t)) = self.download_managers.get_mut(&id) {
                    // send the cancel signal
                    mgr.stop();
                    result_sender.send(Ok(())).unwrap();
                } else {
                    result_sender
                        .send(Err(anyhow!("Playlist was not previously downloading")))
                        .unwrap();
                }
            }
            PlaylistMessage::PlaylistDownloadDone { success: _, id } => {
                println!("playlist download done");
                if let Some((mgr, gui_reply_stream)) = self.download_managers.remove(&id) {
                    gui_reply_stream
                        .send(Message::DownloadPlaylistEnded {
                            id: mgr.get_playlist_id().clone(),
                        })
                        .await
                        .unwrap();
                }
            }
            PlaylistMessage::GetDownloadedTracks { result_sender } => {
                result_sender.send(self.downloaded_tracks.clone()).unwrap();
            }
            PlaylistMessage::TrackDownloadDone { id } => {
                // update local downloaded cache
                self.downloaded_tracks.insert(id.clone());

                // then update the gui
                self.event_sender
                    .send(EventMessage::TrackDownloadFinished { id })
                    .await
                    .unwrap();
            }
            PlaylistMessage::CheckTrackDownloaded { id, result_sender } => {
                let downloaded = self.downloaded_tracks.contains(&id);
                result_sender.send(downloaded).unwrap();
            }
            PlaylistMessage::ShufflePlaylist {
                playlist_id,
                result_sender,
                tracklist,
            } => {
                println!("Shuffling playlist on plalyist end");
                // either take the current tracklist given or create one from the playlist
                let playlist = self.playlists.get(&playlist_id);
                let playlist = match playlist {
                    None => {
                        println!(
                            "failed to shuffle playlist; playlist id did not return a playlist"
                        );
                        return;
                    }
                    Some(playlist) => playlist,
                };
                let mut tracklist = match tracklist {
                    Some(tracklist) => tracklist,
                    None => TrackList::from_playlist_ref(&playlist),
                };
                tracklist.randomize_order();
                result_sender.send(tracklist.clone()).unwrap();

                // take the active mgr if it exists and do some goofy shuffling
                if let Some((mgr, _)) = self.download_managers.get_mut(&playlist_id) {
                    println!("Sending all the requests");
                    // restart mgr with new tracklist
                    mgr.restart_with_tracklist(tracklist);
                }
            }
            PlaylistMessage::OrganizePlaylist {
                playlist_id,
                tracklist,
                result_sender,
            } => {
                println!("sorting playlist on plalyist end");
                // either take the current tracklist given or create one from the playlist
                let playlist = self.playlists.get(&playlist_id);
                let playlist = match playlist {
                    None => {
                        println!(
                            "failed to shuffle playlist; playlist id did not return a playlist"
                        );
                        return;
                    }
                    Some(playlist) => playlist,
                };
                let mut tracklist = match tracklist {
                    Some(tracklist) => tracklist,
                    None => TrackList::from_playlist_ref(&playlist),
                };
                tracklist.sort();
                result_sender.send(tracklist.clone()).unwrap();

                // take the active mgr if it exists and do some goofy shuffling
                if let Some((mgr, _)) = self.download_managers.get_mut(&playlist_id) {
                    println!("Sending all the requests");
                    // restart mgr with new tracklist
                    mgr.restart_with_tracklist(tracklist);
                }
            }
        }
    }
}
