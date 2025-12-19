use std::ffi::OsString;

use anyhow::{Result, anyhow};
use tokio::sync::mpsc;
use url::Url;

use crate::service::{
    file::structs::BinApps,
    process::{
        ProcessSender,
        enums::{ChildMessage, ProcessMessage},
    },
};

use super::structs::Playlist;

pub async fn initialize_playlist(
    url: Url,
    bin_apps: BinApps,
    process_sender: ProcessSender,
) -> Result<Playlist> {
    // construct command
    println!("init playlist?");
    let cmd = bin_apps.yt_dlp.into_os_string();
    let args = vec![
        OsString::from("--ffmpeg"),
        bin_apps.ffmpeg.into_os_string(),
        OsString::from("--newline"),
        OsString::from("--flat-playlist"),
        OsString::from("--dump-json"),
        OsString::from("--no-quiet"),
        OsString::from(url.as_str()),
    ];
    // get channel pair for status messages
    let (tx, mut rx) = mpsc::channel(100);

    // send through channel for execution
    let _send_result = process_sender
        .send(ProcessMessage::SpawnProcess {
            cmd,
            args,
            output_stream: tx,
        })
        .await;

    // receive messages from process
    while let Some(msg) = rx.recv().await {
        let (kind, line) = match msg {
            ChildMessage::StdOut(line) => ("StdOut", line),
            ChildMessage::StdErr(line) => ("StdErr", line),
            ChildMessage::Exit(code) => (
                "Exit",
                format!("Code: {}", code.code().unwrap().to_string()),
            ),
        };
        println!("Recieved message ({kind}): {line}")
    }

    Err(anyhow!("unimplemented"))
}
