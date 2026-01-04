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
        structs::{PlaylistTrackJson, Track},
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
    status_sender: &mpsc::Sender<TaskResponse>,
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

    // cache received track data
    let mut tracks = vec![];
    let mut playlist_name: Option<String> = None;

    // receive messages from process
    while let Some(msg) = rx.recv().await {
        let download_msg = parse_init_output(msg);
        println!("recieved message: {download_msg:?}");

        // if this is a progress message, then notify the gui
        match download_msg {
            ExtractorLineOut::InitProgress { current, total } => {
                let _ = status_sender
                    // wrap in task response for gui
                    .send(TaskResponse::PlaylistInitStatus(
                        PlaylistInitStatus::Progress { current, total },
                    ))
                    .await;
            }
            ExtractorLineOut::InitTrackData(json_track_data) => {
                // add track to list to be added to playlist
                tracks.push(Track::from_playlist_track_json(json_track_data))
            }
            ExtractorLineOut::PlaylistInitDone(name) => playlist_name = Some(name),
            ExtractorLineOut::Exit(status) => match status.code() {
                Some(code) => {
                    if code != 0 {
                        return Err(anyhow!("yt-dlp returned nonzero exit code"));
                    }
                }
                None => return Err(anyhow!("yt-dlp did not return an exit code")),
            },
            _ => {}
        }
    }

    // error checking for playlist
    if tracks.len() == 0 {
        return Err(anyhow!("track length is 0"));
    }
    if let None = playlist_name {
        return Err(anyhow!("no playlist name found"));
    }

    Ok(Playlist::new(playlist_name.unwrap(), tracks))
}

fn parse_init_output(msg: ChildMessage) -> ExtractorLineOut {
    // regex setup just for parsing init logic
    static RE_PROGRESS: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^\[download\] Downloading item (\d+) of (\d+)").unwrap());
    static RE_FINISH: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"\[download\] Finished downloading playlist: (.*)").unwrap());

    match msg {
        ChildMessage::StdOut(line) => {
            // check to see if this line is likely json
            if line.starts_with('{') {
                // try to parse json
                match serde_json::from_str::<PlaylistTrackJson>(&line) {
                    Ok(output) => ExtractorLineOut::InitTrackData(output),
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
                } else if let Some(captures) = RE_FINISH.captures(&line) {
                    return ExtractorLineOut::PlaylistInitDone(
                        captures[1]
                            .parse()
                            .unwrap_or(String::from("Unknown playlist")),
                    );
                }
                // line is not one that is recongized, so just return the line
                ExtractorLineOut::Standard(line)
            }
        }
        ChildMessage::StdErr(line) => ExtractorLineOut::Error(line),
        ChildMessage::Exit(status) => ExtractorLineOut::Exit(status),
    }
}
