use std::collections::HashMap;

use iced::Subscription;
use iced::widget::{Space, container, progress_bar, row};
use iced::{
    Length, Task,
    widget::{Column, button, column, text, text_input},
};
use tokio::sync::{mpsc, oneshot};
use tokio_util::sync::CancellationToken;
use url::Url;

use crate::service::file::FileSender;
use crate::service::gui::structs::IdCounter;
use crate::service::playlist::PlaylistSender;
use crate::service::playlist::enums::{PlaylistInitStatus, PlaylistMessage};
use crate::util::sync::ReceiverHandle;
use enums::{EventMessage, Message, TaskResponse};

pub mod enums;
mod structs;

struct App {
    // Communication
    _shutdown_token: CancellationToken,
    file_sender: FileSender,
    playlist_sender: PlaylistSender,
    tasks: HashMap<u64, ReceiverHandle<TaskResponse>>,
    event_bus: ReceiverHandle<EventMessage>,

    // Internal state
    playlist_url: String,
    id_counter: IdCounter,
    current_track_index: u32,
    total_tracks: u32,
}

#[derive(Clone)]
struct GuiFlags {
    shutdown_token: CancellationToken,
    event_receiver: ReceiverHandle<EventMessage>,
    file_sender: FileSender,
    playlist_sender: PlaylistSender,
    id_counter: IdCounter,
}

impl App {
    fn new(flags: GuiFlags) -> (Self, Task<Message>) {
        (
            Self {
                _shutdown_token: flags.shutdown_token,
                file_sender: flags.file_sender,
                playlist_sender: flags.playlist_sender,
                tasks: HashMap::new(),
                event_bus: flags.event_receiver,
                playlist_url: String::new(),
                id_counter: flags.id_counter,
                current_track_index: 0,
                total_tracks: 0,
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
                        _ => {
                            self.current_track_index = 0;
                            self.total_tracks = 0;
                        }
                    },
                }
                Task::none()
            }
            Message::None => Task::none(),
        }
    }

    fn view(&self) -> Column<'_, Message> {
        let title_text = text("peanut v0.00069?");

        let header = row![title_text, Space::new().width(Length::Fill),];

        let load_file = button("init playlist").on_press(Message::PlaylistURLSubmit);
        let playlist_url = text_input("file path", &self.playlist_url)
            .width(Length::Fill)
            .on_input(Message::PlaylistTextEdit)
            .on_paste(Message::PlaylistTextEdit)
            .on_submit(Message::PlaylistURLSubmit);

        let prog_bar = progress_bar(
            0.0..=self.total_tracks as f32,
            self.current_track_index as f32,
        );
        let text_prog = text(format!(
            "{}/{} tracks init'd",
            self.current_track_index, self.total_tracks
        ));

        let prog_row = row![prog_bar, text_prog];

        let content = container(column![row![playlist_url, load_file], prog_row])
            .width(Length::Fill)
            .height(Length::Fill);

        let footer_text = text("unused text at the bottom :p");

        let footer = row![footer_text];

        column![header, content, footer]
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
        file_sender: FileSender,
        playlist_sender: PlaylistSender,
        event_bus_rx: mpsc::Receiver<EventMessage>,
    ) -> iced::Result {
        let mut id_counter = IdCounter::new();
        let event_recv_id = id_counter.next();

        let flags = GuiFlags {
            shutdown_token,
            file_sender,
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
