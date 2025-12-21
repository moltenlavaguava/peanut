use std::{ffi::OsString, sync::LazyLock};

use regex::Regex;

use anyhow::{Result, anyhow};
use tokio::sync::{mpsc, oneshot};
use url::Url;

use crate::service::{
    file::structs::BinApps,
    gui::enums::TaskResponse,
    playlist::{
        enums::{ExtractorLineOut, PlaylistInitStatus},
        structs::InitJsonOutput,
    },
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
    reply_stream: oneshot::Sender<mpsc::Receiver<TaskResponse>>,
) -> Result<Playlist> {
    // create channel to send info (progress updates) back through
    let (t_init_status, r_init_status) = mpsc::channel(100);
    reply_stream.send(r_init_status);

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
        let download_msg = parse_init_output(msg);
        println!("recieved message: {download_msg:?}");
        if let ExtractorLineOut::InitProgress { current, total } = download_msg {
            let _ = t_init_status
                // wrap in task response for gui
                .send(TaskResponse::PlaylistInitStatus(
                    PlaylistInitStatus::Progress { current, total },
                ))
                .await;
        }
    }

    Err(anyhow!("unimplemented"))
}

fn parse_init_output(msg: ChildMessage) -> ExtractorLineOut {
    // regex setup just for parsing init logic
    static RE_PROGRESS: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^\[download\] Downloading item (\d+) of (\d+)").unwrap());

    match msg {
        ChildMessage::StdOut(line) => {
            // check to see if this line is likely json
            if line.starts_with('{') {
                // try to parse json
                match serde_json::from_str::<InitJsonOutput>(&line) {
                    Ok(output) => ExtractorLineOut::InitData(output),
                    Err(_) => ExtractorLineOut::Standard(line),
                }
            } else {
                // not json, just normal status message
                // check if this is an init progress message
                if let Some(captures) = RE_PROGRESS.captures(&line) {
                    return ExtractorLineOut::InitProgress {
                        current: captures[1].parse().unwrap_or(0),
                        total: captures[2].parse().unwrap_or(0),
                    };
                }
                // line is not one that is recongized, so just return the line
                ExtractorLineOut::Standard(line)
            }
        }
        ChildMessage::StdErr(line) => ExtractorLineOut::Error(line),
        ChildMessage::Exit(status) => ExtractorLineOut::Exit(status),
    }
}
