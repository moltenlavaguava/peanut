use std::collections::{HashMap, HashSet};
use std::env;
use std::io::Write;
use std::path::PathBuf;

use crate::service::id::structs::Id;
use crate::service::playlist::enums::MediaType;
use crate::service::playlist::structs::{Album, Playlist, Track};

use super::structs::BinApps;
use anyhow::anyhow;
use image::ImageFormat;
use reqwest::Client;
use tokio::fs::{self};

const OUTPUT_DIR: &str = "output";
const TRACK_DIR: &str = "track";
const DATA_DIR: &str = "data";
const ALBUM_DIR: &str = "album";
const TRACK_DATA_FILENAME: &str = "tracks";
const ALBUM_DATA_FILENAME: &str = "albums";

const TRACK_EXTENSION: &str = "m4a";
const DATA_EXTENSION: &str = "json";
const ALBUM_EXTENSION: &str = "jpeg";

/// Returns the current project root. If in debug mode, returns the root directory for the project. Otherwise, returns the directory the executable is in.
pub fn get_project_root() -> std::io::Result<PathBuf> {
    #[cfg(not(debug_assertions))]
    {
        let exe_path = env::current_exe()?;
        return Ok(exe_path.with_extension(""));
    }
    #[cfg(debug_assertions)]
    {
        Ok(PathBuf::from(env!("CARGO_MANIFEST_DIR")))
    }
}

pub fn get_bin_app_paths() -> BinApps {
    // get root path where binaries should be located
    let root_path = get_project_root().expect("Critical: could not find root directory path");
    // manipulate paths
    let bin_path = root_path.join("bin");
    let yt_dlp_path = bin_path.join("yt-dlp_x86.exe");
    let ffmpeg_path = bin_path.join("ffmpeg").join("ffmpeg.exe");
    let deno_path = bin_path.join("deno.exe");
    if !yt_dlp_path.is_file() || !ffmpeg_path.is_file() || !deno_path.is_file() {
        println!(
            "Paths: '{}' '{}' '{}'",
            yt_dlp_path.display(),
            ffmpeg_path.display(),
            deno_path.display()
        );
        panic!(
            "Critical: yt_dlp, ffmpeg, or deno exes are not found or invalid.\nEnsure that there exists a bin folder in the main directory and that it contains yt-dlp_x86.exe, deno.exe, and a folder named ffmpeg containing ffmpeg.exe\n(Output dir: {}",
            root_path.display()
        )
    }
    // construct result
    BinApps {
        yt_dlp: yt_dlp_path,
        ffmpeg: ffmpeg_path,
        deno: deno_path,
    }
}

pub fn output_dir_path() -> anyhow::Result<PathBuf> {
    Ok(get_project_root()?.join(OUTPUT_DIR))
}

pub fn track_dir_path() -> anyhow::Result<PathBuf> {
    Ok(output_dir_path()?.join(TRACK_DIR))
}

pub fn data_dir_path() -> anyhow::Result<PathBuf> {
    Ok(output_dir_path()?.join(DATA_DIR))
}

pub fn album_dir_path() -> anyhow::Result<PathBuf> {
    Ok(output_dir_path()?.join(ALBUM_DIR))
}

pub fn track_file_path_from_id(id: &Id) -> anyhow::Result<PathBuf> {
    let MediaType::Track = id.media_type else {
        return Err(anyhow!("Id provided was not a track id"));
    };
    let mut track_path = track_dir_path()?.join(id.to_string());
    track_path.set_extension(TRACK_EXTENSION);
    Ok(track_path)
}

pub fn playlist_file_path_from_id(id: &Id) -> anyhow::Result<PathBuf> {
    let MediaType::Playlist = id.media_type else {
        return Err(anyhow!("Id provided was not a playlist id"));
    };
    let mut playlist_path = data_dir_path()?.join(id.to_string());
    playlist_path.set_extension(DATA_EXTENSION);
    Ok(playlist_path)
}

pub fn track_file_exists(id: &Id) -> bool {
    matches!(track_file_path_from_id(&id), Ok(_))
}

pub async fn get_downloaded_tracks() -> anyhow::Result<HashSet<Id>> {
    // get the track data dir
    let track_dir = track_dir_path()?;
    // go through the directory and search for valid playlist files and add any successful files to a vec
    let mut track_ids = HashSet::new();
    let mut paths = fs::read_dir(track_dir).await?;
    while let Some(path) = paths.next_entry().await.ok().flatten() {
        if path.path().is_file() {
            // check to see if the name of the file is a valid track id (removing filename)
            let path = path.path().with_extension("");
            let file_name = path.file_stem();
            if let Some(osstr) = file_name {
                let s = osstr.to_string_lossy().into_owned();
                // try to parse to id
                let id = Id::from_string(s);
                if let Ok(id) = id {
                    // valid id, check if it is a track id
                    if let MediaType::Track = id.media_type {
                        track_ids.insert(id);
                    }
                }
            }
        }
    }
    Ok(track_ids)
}

pub async fn load_saved_playlists() -> anyhow::Result<HashMap<Id, Playlist>> {
    // get the data dir
    let data_dir = data_dir_path()?;
    // go through the directory and search for valid playlist files and add any successful files to a vec
    let mut playlists = HashMap::new();
    let mut paths = fs::read_dir(data_dir).await?;
    while let Some(path) = paths.next_entry().await.ok().flatten() {
        if path.path().is_file() {
            // check to see if the name of the file is a valid playlist id
            let file_name = path.file_name().into_string();
            if let Ok(s) = file_name {
                // try to parse to id
                let id = Id::from_string(s);
                if let Ok(id) = id {
                    // valid id, check if it is a playlist id
                    if let MediaType::Playlist = id.media_type {
                        // valid file, let's have a look at its contents
                        let maybe_contents = tokio::fs::read_to_string(path.path()).await;
                        if let Ok(contents) = maybe_contents {
                            let maybe_playlist: Result<Playlist, serde_json::Error> =
                                serde_json::from_str(&contents);
                            if let Ok(playlist) = maybe_playlist {
                                playlists.insert(playlist.id().clone(), playlist);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(playlists)
}

pub async fn get_saved_tracks_file_path() -> anyhow::Result<PathBuf> {
    let mut tracks_file_path = data_dir_path()?;
    tracks_file_path.push(TRACK_DATA_FILENAME);
    tracks_file_path.set_extension(DATA_EXTENSION);
    Ok(tracks_file_path)
}

pub async fn get_album_data_file_path() -> anyhow::Result<PathBuf> {
    let mut albums_file_path = data_dir_path()?;
    albums_file_path.push(ALBUM_DATA_FILENAME);
    albums_file_path.set_extension(DATA_EXTENSION);
    Ok(albums_file_path)
}

// Gets all albums. Note that albums that are in this list should be downloaded.
pub async fn get_albums() -> anyhow::Result<HashMap<Id, Album>> {
    let path = get_album_data_file_path().await?;
    let file_text = fs::read_to_string(path).await?;
    let albums_vec: Vec<Album> = serde_json::from_str(&file_text)?;
    let mut map = HashMap::new();
    for album in albums_vec {
        map.insert(album.id().clone(), album);
    }
    Ok(map)
}

pub fn album_filename_from_id(album_id: &Id) -> anyhow::Result<PathBuf> {
    let mut album_dir_path = album_dir_path()?;
    album_dir_path.push(album_id.to_string());
    album_dir_path.set_extension(ALBUM_EXTENSION);
    Ok(album_dir_path)
}

pub async fn load_saved_tracks() -> anyhow::Result<HashMap<Id, Track>> {
    // get the tracks file path
    let tracks_file_path = get_saved_tracks_file_path().await?;

    let file_contents = fs::read_to_string(tracks_file_path).await?;
    let tracks: Vec<Track> = serde_json::from_str(&file_contents)?;
    let mut hashmap = HashMap::new();
    for track in tracks {
        hashmap.insert(track.id().clone(), track);
    }
    Ok(hashmap)
}

pub fn track_output_extension() -> &'static str {
    TRACK_EXTENSION
}

pub async fn download_album(album: &Album, client: &Client) -> anyhow::Result<()> {
    // download raw image bytes
    let response = client
        .get(album.img_url.clone())
        .send()
        .await?
        .error_for_status()?;
    let bytes = response.bytes().await?.to_vec();
    let album_filename = album_filename_from_id(&album.id())?;

    // spawn heavy stuff in separate thread
    tokio::task::spawn_blocking(move || {
        // check if img is jpeg (it probably is)
        let format = image::guess_format(&bytes)?;
        if format == ImageFormat::Jpeg {
            // save the file as is
            let mut file = std::fs::File::create(album_filename.clone())?;
            file.write_all(&bytes)?;
        } else {
            let img = image::load_from_memory(&bytes)?;
            let mut file = std::fs::File::create(album_filename)?;
            img.write_to(&mut file, ImageFormat::Jpeg)?;
        }
        Ok::<(), anyhow::Error>(())
    })
    .await??;

    Ok(())
}
