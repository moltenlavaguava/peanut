use std::{
    ffi::{OsStr, OsString},
    path::PathBuf,
    sync::LazyLock,
};

use regex::Regex;

use anyhow::{Result, anyhow};
use tokio::sync::mpsc;
use url::Url;

use crate::service::{
    file::structs::BinApps,
    gui::enums::TaskResponse,
    id::{enums::Platform, structs::Id},
    playlist::{
        enums::{ExtractorContext, ExtractorLineOut, MediaType, PlaylistInitStatus},
        structs::{PlaylistTrackJson, Track, TrackDownloadJson},
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
    let mut deno_s = OsString::from("deno:");
    deno_s.push(bin_apps.deno.into_os_string());
    let args = vec![
        OsString::from("--ffmpeg"),
        bin_apps.ffmpeg.into_os_string(),
        OsString::from("--js-runtimes"),
        deno_s,
        OsString::from("--newline"),
        OsString::from("--flat-playlist"),
        OsString::from("--dump-json"),
        OsString::from("--no-quiet"),
        OsString::from(url.as_str()),
    ];

    println!(
        "Command:\n{} {}",
        cmd.display(),
        args.iter()
            .map(|os| os.as_ref())
            .collect::<Vec<&OsStr>>()
            .join(OsStr::new(" "))
            .display()
    );
    
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
    let mut playlist_id: Option<String> = None;

    // receive messages from process
    while let Some(msg) = rx.recv().await {
        let download_msg = parse_output(msg, ExtractorContext::Initialize);
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
                // if the playlist id isn't already set, use this track data to get it
                if let None = playlist_id {
                    playlist_id = Some(json_track_data.playlist_id.clone())
                }
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

    // make the id for the playlist. unwrap here should be fine due to error checking above
    let id = Id::new(Platform::Youtube, MediaType::Playlist, playlist_id.unwrap());
    Ok(Playlist::new(playlist_name.unwrap(), tracks, id))
}

pub async fn download_track(
    url: &Url,
    download_directory: PathBuf,
    file_name: String,
    bin_apps: BinApps,
    process_sender: &ProcessSender,
    // status_sender: &mpsc::Sender<TaskResponse>,
) -> Result<Track> {
    let cmd = bin_apps.yt_dlp.into_os_string();
    let mut deno_s = OsString::from("deno:");
    deno_s.push(bin_apps.deno.into_os_string());
    let args = vec![
        OsString::from("--ffmpeg"),
        bin_apps.ffmpeg.into_os_string(),
        OsString::from("--js-runtimes"),
        deno_s,
        OsString::from("--newline"),
        OsString::from("--dump-json"),
        OsString::from("--no-quiet"),
        OsString::from("-P"),
        OsString::from(download_directory),
        OsString::from("-o"),
        OsString::from(format!("{}.%(ext)s", file_name)),
        OsString::from("-f"),
        OsString::from("bestaudio"),
        OsString::from("--audio-format"),
        OsString::from("opus"),
        OsString::from("--no-simulate"),
        OsString::from(url.as_str()),
    ];

    println!(
        "Command:\n{} {}",
        cmd.display(),
        args.iter()
            .map(|os| os.as_ref())
            .collect::<Vec<&OsStr>>()
            .join(OsStr::new(" "))
            .display()
    );

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

    while let Some(msg) = rx.recv().await {
        println!("Received msg from download: {msg:?}");
        let line = parse_output(msg, ExtractorContext::Download);
    }

    Err(anyhow!("unimplemented"))
}

fn parse_output(msg: ChildMessage, context: ExtractorContext) -> ExtractorLineOut {
    // regex setup just for parsing init logic
    static RE_PROGRESS: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^\[download\] Downloading item (\d+) of (\d+)").unwrap());
    static RE_FINISH: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"\[download\] Finished downloading playlist: (.*)").unwrap());
    static RE_DOWNLOAD_PROGRESS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[download\]\s+(\d+(?:\.\d+)?)%\s+of\s+(\d+(?:\.\d+)?)(KiB|MiB|GiB)\s+at\s+(\d+(?:\.\d+)?)(KiB|MiB|GiB)\/s\s+ETA\s+(\d+):(\d+)").unwrap());

    match msg {
        ChildMessage::StdOut(line) => {
            // check to see if this line is likely json
            if line.starts_with('{') {
                // try to parse json. depends on if this is an init or download though
                match context {
                    ExtractorContext::Initialize => {
                        match serde_json::from_str::<PlaylistTrackJson>(&line) {
                            Ok(output) => ExtractorLineOut::InitTrackData(output),
                            Err(_) => ExtractorLineOut::Standard(line),
                        }
                    }
                    ExtractorContext::Download => {
                        match serde_json::from_str::<TrackDownloadJson>(&line) {
                            Ok(output) => ExtractorLineOut::DownloadTrackData(output),
                            Err(_) => ExtractorLineOut::Standard(line),
                        }
                    }
                }
            } else {
                // not json, just normal status message
                match context {
                    ExtractorContext::Initialize => {
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
                        return ExtractorLineOut::Standard(line)
                    }
                    ExtractorContext::Download => {
                        if let Some(captures) = RE_DOWNLOAD_PROGRESS.captures(&line) {
                            return ExtractorLineOut::DownloadProgress { progress: captures[1].parse().unwrap_or(0.0), download_size: captures[2].parse().unwrap_or(0.0), download_speed: captures[4].parse().unwrap_or(0.0), eta: (captures[6].parse().unwrap_or(1000), captures[7].parse.unwrap_or(1000)) }
                        }
                }
            }
        }
        ChildMessage::StdErr(line) => ExtractorLineOut::Error(line),
        ChildMessage::Exit(status) => ExtractorLineOut::Exit(status),
    }
}
