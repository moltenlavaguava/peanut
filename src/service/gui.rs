use std::collections::HashMap;

use iced::Subscription;
use iced::{Task, widget::Column};
use tokio::sync::{mpsc, oneshot};
use tokio_util::sync::CancellationToken;
use url::Url;

use crate::service::gui::structs::IdCounter;
use crate::service::playlist::PlaylistSender;
use crate::service::playlist::enums::{PlaylistInitStatus, PlaylistMessage};
use crate::util::sync::ReceiverHandle;
use enums::{EventMessage, Message, Page, TaskResponse};
use util::{home, player};

pub mod enums;
mod structs;
mod util;

struct App {
    // Communication
    _shutdown_token: CancellationToken,
    playlist_sender: PlaylistSender,
    tasks: HashMap<u64, ReceiverHandle<TaskResponse>>,
    event_bus: ReceiverHandle<EventMessage>,

    // Internal state
    playlist_url: String,
    id_counter: IdCounter,
    current_track_index: u32,
    total_tracks: u32,
    page: Page,
    loaded_playlist_names: Vec<String>,
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
                loaded_playlist_names: vec![],
            },
            Task::none(),
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
            Message::FileLoadResult(result) => match result {
                Ok(contents) => {
                    println!("contents: {contents}");
                    Task::none()
                }
                Err(err) => {
                    println!("error: {err}");
                    Task::none()
                }
            },
            Message::EventRecieved(msg) => {
                println!("Recieved event message: {msg:?}");
                match msg {
                    EventMessage::InitialPlaylistsInitalized(playlist_data) => {
                        for (playlist_title, _playlist_id) in playlist_data {
                            self.loaded_playlist_names.push(playlist_title);
                        }
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
            Message::TaskDataReceived(_id, response) => {
                // manage state with the response
                match response {
                    TaskResponse::PlaylistInitStatus(status) => match status {
                        PlaylistInitStatus::Progress { current, total } => {
                            self.current_track_index = current;
                            self.total_tracks = total;
                        }
                        PlaylistInitStatus::Complete { title } => {
                            println!("received msg that playlist with title {title} finished init");
                            self.loaded_playlist_names.push(title);
                        }
                        PlaylistInitStatus::Fail => {
                            println!("received msg that playlist init failed");
                        }
                        PlaylistInitStatus::Duplicate { title } => {
                            println!("received msg that playlist {title} was a duplicate")
                        }
                    },
                }
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
        let tasks = Subscription::batch(self.tasks.values().map(|handle| {
            handle.watch(
                |id, msg| Message::TaskDataReceived(id, msg),
                |id| Message::TaskFinished(id),
            )
        }));

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
