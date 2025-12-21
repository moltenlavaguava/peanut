use std::{ffi::OsString, sync::LazyLock};

use regex::Regex;

use anyhow::{Result, anyhow};
use tokio::sync::mpsc;
use url::Url;

use crate::service::{
    file::structs::BinApps, playlist::{enums::DownloadMessage, structs::InitJsonOutput}, process::{
        ProcessSender,
        enums::{ChildMessage, ProcessMessage},
    }
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
        let download_msg = parse_init_output(msg);
        println!("recieved message: {download_msg:?}")
    }

    Err(anyhow!("unimplemented"))
} 

fn parse_init_output(msg: ChildMessage) -> DownloadMessage {
    // regex setup just for parsing init logic
    static RE_PROGRESS: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^\[download\] Downloading item (\d+) of (\d+)").unwrap()
    });

    match msg {
        ChildMessage::StdOut(line) => {
            // check to see if this line is likely json
            if line.starts_with('{') {
                // try to parse json
                match serde_json::from_str::<InitJsonOutput>(&line) {
                    Ok(output) => DownloadMessage::InitData(output),
                    Err(_) => DownloadMessage::Standard(line),
                }
            } else {
                // not json, just normal status message
                // check if this is an init progress message
                if let Some(captures) = RE_PROGRESS.captures(&line) { 
                    return DownloadMessage::InitProgress {
                        current: captures[1].parse().unwrap_or(0),
                        total: captures[2].parse().unwrap_or(0),
                    };
                }
                // line is not one that is recongized, so just return the line
                DownloadMessage::Standard(line)
            }
        }
        ChildMessage::StdErr(line) => {
            DownloadMessage::Error(line)
        }
        ChildMessage::Exit(status) => {
            DownloadMessage::Exit(status)
        }
    }
}