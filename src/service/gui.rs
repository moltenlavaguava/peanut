use std::collections::HashMap;
use std::io::ErrorKind;
use std::path::PathBuf;

use iced::Subscription;
use iced::widget::{Space, container, row};
use iced::{
    Length, Task,
    widget::{Column, button, column, text, text_input},
};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use url::Url;

use crate::service::file::FileSender;
use crate::service::playlist::PlaylistSender;
use crate::service::playlist::enums::PlaylistMessage;
use crate::util::sync::{EventMessage, ReceiverHandle, TaskResponse};
use enums::Message;

pub mod enums;

struct App {
    // Communication
    _shutdown_token: CancellationToken,
    file_sender: FileSender,
    playlist_sender: PlaylistSender,
    tasks: HashMap<u64, ReceiverHandle<TaskResponse>>,
    event_bus: ReceiverHandle<EventMessage>,

    // Internal state
    playlist_url: String,
}

#[derive(Clone)]
struct GuiFlags {
    shutdown_token: CancellationToken,
    event_receiver: ReceiverHandle<EventMessage>,
    file_sender: FileSender,
    playlist_sender: PlaylistSender,
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
                    Task::future(submit_playlist_url(url, self.playlist_sender.clone())).discard()
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
            Message::StartTestTask => Task::none(),
        }
    }

    fn view(&self) -> Column<'_, Message> {
        let title_text = text("peanut v0.00069?");

        let header = row![title_text, Space::new().width(Length::Fill),];

        let load_file = button("init playlist").on_press(Message::PlaylistURLSubmit);
        let start_task = button("start task").on_press(Message::StartTestTask);
        let playlist_url = text_input("file path", &self.playlist_url)
            .width(Length::Fill)
            .on_input(Message::PlaylistTextEdit)
            .on_paste(Message::PlaylistTextEdit)
            .on_submit(Message::PlaylistURLSubmit);

        let content = container(row![playlist_url, load_file, start_task])
            .width(Length::Fill)
            .height(Length::Fill);

        let footer_text = text("unused text at the bottom :p");

        let footer = row![footer_text];

        column![header, content, footer]
    }
    fn subscription(&self) -> Subscription<Message> {
        self.event_bus.watch(
            |_id, msg| Message::EventRecieved(msg),
            |_id| Message::EventBusClosed,
        )
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
        let flags = GuiFlags {
            shutdown_token,
            file_sender,
            playlist_sender,
            event_receiver: ReceiverHandle::new(0, event_bus_rx),
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
async fn submit_playlist_url(url: Url, sender: PlaylistSender) {
    println!("submitting playlist url..");
    let _ = sender
        .send(PlaylistMessage::InitializePlaylist { url })
        .await;
}
