use std::collections::HashMap;
use std::fs;

use crate::service::playlist::enums::MediaType;
use crate::service::{file, id::structs::Id};

use super::structs::Playlist;

use super::PlaylistService;

pub async fn load_saved_playlists() -> anyhow::Result<HashMap<Id, Playlist>> {
    // get the playlist data dir
    let playlist_data_dir = file::util::data_dir_path()?;
    // go through the directory and search for valid playlist files and add any successful files to a vec
    let mut playlists = HashMap::new();
    let paths = fs::read_dir(playlist_data_dir)?;
    for path in paths {
        if let Ok(path) = path {
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
    }
    Ok(playlists)
}
