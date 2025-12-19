use std::{ffi::OsString, process::ExitStatus};

use tokio::sync::mpsc;

pub enum ProcessMessage {
    SpawnProcess {
        cmd: OsString,
        args: Vec<OsString>,
        output_stream: mpsc::Sender<ChildMessage>,
    },
}

pub enum ChildMessage {
    StdOut(String),
    StdErr(String),
    Exit(ExitStatus),
}
