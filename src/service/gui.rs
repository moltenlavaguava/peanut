use std::collections::{HashMap, HashSet};

use iced::Subscription;
use iced::{Task, widget::Column};
use tokio::sync::{mpsc, oneshot};
use tokio_util::sync::CancellationToken;
use url::Url;

use crate::service::gui::enums::{Action, PlayingState};
use crate::service::gui::structs::IdCounter;
use crate::service::id::structs::Id;
use crate::service::playlist::PlaylistSender;
use crate::service::playlist::enums::{PlaylistInitStatus, PlaylistMessage};
use crate::service::playlist::structs::{PTrackList, PlaylistMetadata};
use crate::util::sync::ReceiverHandle;
use enums::{EventMessage, Message, Page};
use util::{home, player};

pub mod enums;
mod structs;
mod util;

struct App {
    // Communication
    _shutdown_token: CancellationToken,
    playlist_sender: PlaylistSender,
    tasks: HashMap<u64, ReceiverHandle<Message>>,
    event_bus: ReceiverHandle<EventMessage>,

    // Internal state
    playlist_url: String,
    id_counter: IdCounter,
    current_track_index: u32,
    total_tracks: u32,
    page: Page,
    loaded_playlist_metadata: Vec<PlaylistMetadata>,
    downloaded_tracks: HashSet<Id>,
    downloading_tracks: HashSet<Id>,
    current_ptracklist: Option<PTrackList>,
    track_playing_state: PlayingState,
    download_stopping_playlists: HashSet<Id>,
    downloading_playlists: HashSet<Id>,
}

#[derive(Clone)]
struct GuiFlags {
    shutdown_token: CancellationToken,
    event_receiver: ReceiverHandle<EventMessage>,
    playlist_sender: PlaylistSender,
    id_counter: IdCounter,
}

impl App {
    fn new(flags: GuiFlags) -> (Self, Task<Message>) {
        let playlist_sender_clone = flags.playlist_sender.clone();
        (
            Self {
                _shutdown_token: flags.shutdown_token,
                playlist_sender: flags.playlist_sender,
                tasks: HashMap::new(),
                event_bus: flags.event_receiver,
                playlist_url: String::new(),
                id_counter: flags.id_counter,
                current_track_index: 0,
                total_tracks: 0,
                page: Page::Home,
                loaded_playlist_metadata: Vec::new(),
                current_ptracklist: None,
                downloaded_tracks: HashSet::new(),
                track_playing_state: PlayingState::Stopped,
                downloading_playlists: HashSet::new(),
                download_stopping_playlists: HashSet::new(),
                downloading_tracks: HashSet::new(),
            },
            Task::perform(
                util::request_downloaded_tracks(playlist_sender_clone),
                |maybe_tracks| {
                    if let Ok(tracks) = maybe_tracks {
                        Message::DownloadedTrackListReceived(tracks)
                    } else {
                        Message::DownloadedTrackListReceived(HashSet::new())
                    }
                },
            ),
        )
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PlaylistTextEdit(txt) => {
                self.playlist_url = txt;
                Task::none()
            }
            Message::PlaylistURLSubmit => {
                println!("playlist url: {}", self.playlist_url);
                // try to create a url
                if let Ok(url) = Url::parse(&self.playlist_url) {
                    Task::perform(
                        submit_playlist_url(
                            self.id_counter.next(),
                            url,
                            self.playlist_sender.clone(),
                        ),
                        |msg| msg,
                    )
                } else {
                    Task::none()
                }
            }
            Message::EventRecieved(msg) => {
                println!("Recieved event message: {msg:?}");
                match msg {
                    EventMessage::InitialPlaylistsInitalized(playlist_data) => {
                        self.loaded_playlist_metadata = playlist_data;
                    }
                    EventMessage::TrackDownloadFinished { id } => {
                        // a given track finished downloading.
                        println!("Track download finished");
                        // add downloaded track to list and remove it from the downloading tracks list
                        self.downloading_tracks.remove(&id);
                        self.downloaded_tracks.insert(id);
                    }
                };
                Task::none()
            }
            Message::EventBusClosed => {
                println!("Event bus closed");
                Task::none()
            }
            Message::TaskFinished(id) => {
                self.tasks.remove(&id);
                Task::none()
            }
            Message::PlaylistInitTaskStarted(task_id, handle) => {
                self.tasks.insert(task_id, handle);
                self.current_track_index = 0;
                self.total_tracks = 0;
                Task::none()
            }
            Message::PlaylistInitStatus(status) => {
                match status {
                    PlaylistInitStatus::Progress { current, total } => {
                        self.current_track_index = current;
                        self.total_tracks = total;
                    }
                    PlaylistInitStatus::Complete(metadata) => {
                        self.loaded_playlist_metadata.push(metadata);
                    }
                    PlaylistInitStatus::Fail => {
                        println!("received msg that playlist init failed");
                    }
                    PlaylistInitStatus::Duplicate(metadata) => {
                        println!(
                            "received msg that playlist {} was a duplicate",
                            metadata.title
                        );
                    }
                }
                Task::none()
            }
            Message::PlaylistSelect(playlist_metadata) => {
                println!("selected metadata: {playlist_metadata:?}");
                // request playlist
                let playlist_sender_clone = self.playlist_sender.clone();
                Task::perform(
                    util::request_playlist(playlist_metadata.id.clone(), playlist_sender_clone),
                    |output| {
                        Message::PlaylistSelectAccepted(PTrackList {
                            metadata: playlist_metadata,
                            list: output.unwrap().unwrap(),
                        })
                    },
                )
            }
            Message::PlaylistSelectAccepted(ptracklist) => {
                // change the page + set the current playlist
                self.current_ptracklist = Some(ptracklist);
                self.page = Page::Player;
                Task::none()
            }
            Message::DownloadedTrackListReceived(tracks) => {
                // add to the list of downloaded tracks
                self.downloaded_tracks.extend(tracks);
                Task::none()
            }
            Message::Action(action) => {
                match action {
                    Action::Home => {
                        // reset everything player-wise
                        self.current_ptracklist = None;
                        self.page = Page::Home;
                        Task::none()
                    }
                    Action::DownloadPlaylist { playlist_id } => {
                        // send request to playlist service to download
                        let playlist_sender_clone = self.playlist_sender.clone();
                        let next_id = self.id_counter.next();
                        Task::perform(
                            util::download_playlist(
                                playlist_id,
                                playlist_sender_clone.clone(),
                                next_id,
                            ),
                            |maybe_msg| {
                                if let Ok(msg) = maybe_msg {
                                    msg
                                } else {
                                    println!(
                                        "Message from download playlist was an error; probably channel dropped"
                                    );
                                    Message::None
                                }
                            },
                        )
                    }
                    Action::StopPlaylistDownload { playlist_id } => {
                        let playlist_sender_clone = self.playlist_sender.clone();
                        Task::perform(
                            util::stop_playlist_download(
                                playlist_id.clone(),
                                playlist_sender_clone,
                            ),
                            |result| {
                                if let Err(e) = result {
                                    println!(
                                        "An error occured while stopping the playlist download: {}",
                                        e
                                    )
                                }
                                Message::PlaylistDownloadCancelStarted { id: playlist_id }
                            },
                        )
                    }
                    _ => Task::none(),
                }
            }
            Message::DownloadPlaylistStarted {
                id,
                receiver_handle,
            } => {
                // start the (listening) task
                self.tasks.insert(receiver_handle.id(), receiver_handle);
                // mark this playlist as downloading
                self.downloading_playlists.insert(id);
                Task::none()
            }
            Message::DownloadPlaylistEnded { id } => {
                self.downloading_playlists.remove(&id);
                self.download_stopping_playlists.remove(&id);
                println!(
                    "Download ended. Downloading playlists: {:?}; Stopping playlists: {:?}",
                    self.downloading_playlists, self.download_stopping_playlists
                );
                Task::none()
            }
            Message::PlaylistDownloadCancelStarted { id } => {
                self.downloading_playlists.remove(&id);
                self.download_stopping_playlists.insert(id);
                println!(
                    "Cancel started. Downloading playlists: {:?}; Stopping playlists: {:?}",
                    self.downloading_playlists, self.download_stopping_playlists
                );
                Task::none()
            }
            Message::TrackDownloadStarted { id } => {
                // a given track started downloading.
                println!("track download started");
                self.downloading_tracks.insert(id);
                Task::none()
            }
            Message::TrackDownloadStatus { id: _, data: _ } => {
                // A given track's download status updated.
                println!("Track download progress");
                Task::none()
            }
            Message::None => Task::none(),
        }
    }

    fn view(&self) -> Column<'_, Message> {
        match self.page {
            Page::Home => home(&self),
            Page::Player => player(&self),
        }
    }
    fn subscription(&self) -> Subscription<Message> {
        let bus = self.event_bus.watch(
            |_id, msg| Message::EventRecieved(msg),
            |_id| Message::EventBusClosed,
        );
        let tasks = Subscription::batch(
            self.tasks
                .values()
                .map(|handle| handle.watch(|_id, msg| msg, |id| Message::TaskFinished(id))),
        );

        Subscription::batch(vec![bus, tasks])
    }
}

/// Handles GUI processing.
pub struct GuiService {
    // shutdown_token: Option<CancellationToken>,
}

impl GuiService {
    /// Creates the GuiService.
    pub fn new() -> Self {
        GuiService {
            // shutdown_token: Some(shutdown_token),
        }
    }
    pub fn start_loop(
        &self,
        shutdown_token: CancellationToken,
        playlist_sender: PlaylistSender,
        event_bus_rx: mpsc::Receiver<EventMessage>,
    ) -> iced::Result {
        let mut id_counter = IdCounter::new();
        let event_recv_id = id_counter.next();

        let flags = GuiFlags {
            shutdown_token,
            playlist_sender,
            event_receiver: ReceiverHandle::new(event_recv_id, event_bus_rx),
            id_counter,
        };

        let application =
            iced::application(move || App::new(flags.clone()), App::update, App::view)
                .subscription(App::subscription)
                .title("peanut")
                .exit_on_close_request(true);

        application.run()
    }
}

// helper methods
async fn submit_playlist_url(task_id: u64, url: Url, sender: PlaylistSender) -> Message {
    println!("submitting playlist url..");
    // create oneshot channel to get progress update from
    let (tx, rx) = oneshot::channel();

    let _ = sender
        .send(PlaylistMessage::InitializePlaylist {
            url,
            reply_stream: tx,
        })
        .await;

    match rx.await {
        Ok(raw_recv) => {
            println!("Sending playlist init task started msg");
            let handle = ReceiverHandle::new(task_id, raw_recv);
            Message::PlaylistInitTaskStarted(task_id, handle)
        }
        Err(_) => {
            println!("something went wrong when submitting playlist url?");
            Message::None
        }
    }
}
