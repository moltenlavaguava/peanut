use std::io;

use crate::util::sync::EventMessage;

#[derive(Debug, Clone)]
pub enum Message {
    FileTextEdit(String),
    FileTextSubmit,
    FileLoadResult(Result<String, io::ErrorKind>),
    EventRecieved(EventMessage),
    EventBusClosed,
    TaskFinished(u64),
    StartTestTask,
}
