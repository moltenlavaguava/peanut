use std::io;

use tokio::sync::mpsc;

use crate::{service::playlist::enums::PlaylistInitStatus, util::sync::ReceiverHandle};

#[derive(Debug, Clone)]
pub enum Message {
    PlaylistTextEdit(String),
    PlaylistURLSubmit,
    FileLoadResult(Result<String, io::ErrorKind>),
    EventRecieved(EventMessage),
    EventBusClosed,
    TaskFinished(u64),
    PlaylistInitTaskStarted(u64, ReceiverHandle<TaskResponse>),
    TaskDataReceived(u64, TaskResponse),
    None,
}

// represents each possible major page the gui can be
#[derive(Debug, Clone)]
pub enum Page {
    Home,
    Player,
}

// wrapper around all possible task messages
#[derive(Debug, Clone)]
pub enum TaskResponse {
    // fired when new track info is received for a playlist init
    PlaylistInitStatus(PlaylistInitStatus),
}

// for app-wide messages (usually more important)
#[derive(Debug, Clone)]
pub enum EventMessage {}

pub type EventSender = mpsc::Sender<EventMessage>;
