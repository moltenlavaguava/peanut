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
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;

use crate::service::file::enums::FileMessage;
use crate::util::sync::{EventMessage, ReceiverHandle, TaskResponse};
use enums::Message;

pub mod enums;

struct App {
    // Communication
    _shutdown_token: CancellationToken,
    file_comm: mpsc::Sender<FileMessage>,
    tasks: HashMap<u64, ReceiverHandle<TaskResponse>>,
    event_bus: ReceiverHandle<EventMessage>,

    // Internal state
    file_path: String,
}

#[derive(Clone)]
struct GuiFlags {
    shutdown_token: CancellationToken,
    event_receiver: ReceiverHandle<EventMessage>,
    file_tx: mpsc::Sender<FileMessage>,
}

impl App {
    fn new(flags: GuiFlags) -> (Self, Task<Message>) {
        (
            Self {
                _shutdown_token: flags.shutdown_token,
                file_comm: flags.file_tx,
                tasks: HashMap::new(),
                event_bus: flags.event_receiver,
                file_path: String::new(),
            },
            Task::none(),
        )
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FileTextEdit(txt) => {
                self.file_path = txt;
                Task::none()
            }
            Message::FileTextSubmit => {
                println!("file path: {}", self.file_path);
                let path_buf = PathBuf::from(&self.file_path);
                Task::perform(
                    request_file_contents(self.file_comm.clone(), path_buf),
                    Message::FileLoadResult,
                )
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

        let load_file = button("load file").on_press(Message::FileTextSubmit);
        let start_task = button("start task").on_press(Message::StartTestTask);
        let file_path = text_input("file path", &self.file_path)
            .width(Length::Fill)
            .on_input(Message::FileTextEdit)
            .on_paste(Message::FileTextEdit)
            .on_submit(Message::FileTextSubmit);

        let content = container(row![file_path, load_file, start_task])
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
        file_comm: mpsc::Sender<FileMessage>,
        event_bus_rx: mpsc::Receiver<EventMessage>,
    ) -> iced::Result {
        let flags = GuiFlags {
            shutdown_token,
            file_tx: file_comm,
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
async fn request_file_contents(
    mailbox: mpsc::Sender<FileMessage>,
    path: PathBuf,
) -> Result<String, ErrorKind> {
    let (tx, rx) = oneshot::channel();
    let msg = FileMessage::ReadFile {
        reply: tx,
        path_buf: path,
    };

    // send the data and wait
    if let Err(_) = mailbox.send(msg).await {
        return Err(ErrorKind::BrokenPipe);
    }
    let result = rx.await;
    match result {
        Ok(inner) => inner,
        Err(_) => Err(ErrorKind::Other),
    }
}
