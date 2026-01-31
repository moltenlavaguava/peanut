use std::collections::{HashSet, VecDeque};
use std::time::Duration;

use tokio::sync::{mpsc, oneshot};

use crate::service::audio::enums::{AlbumKind, LoopPolicy};
use crate::service::gui::App;
use crate::service::id::structs::Id;
use crate::service::playlist::PlaylistSender;
use crate::service::playlist::enums::{Artist, PlaylistMessage};
use crate::service::playlist::structs::{Album, OwnedPlaylist, PlaylistMetadata, Track, TrackList};
use crate::util::sync::ReceiverHandle;

use super::enums::Message;

// requesting methods
pub async fn request_owned_playlist(
    id: Id,
    playlist_sender: PlaylistSender,
) -> anyhow::Result<Option<OwnedPlaylist>> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::RequestOwnedPlaylist {
            id,
            result_sender: tx,
        })
        .await?;
    rx.await.map_err(|err| anyhow::Error::from(err))
}

pub async fn request_downloaded_tracks(
    playlist_sender: PlaylistSender,
) -> anyhow::Result<HashSet<Id>> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::GetDownloadedTracks { result_sender: tx })
        .await?;

    let tracks = rx.await?;
    Ok(tracks)
}

pub async fn download_playlist(
    id: Id,
    playlist_sender: PlaylistSender,
    task_id: u64,
    tracklist: TrackList,
) -> anyhow::Result<Message> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::DownloadPlaylist {
            id: id.clone(),
            tracklist: tracklist,
            reply_stream: tx,
        })
        .await?;
    let receiver = rx.await?;
    let receiver_handle = ReceiverHandle::new(task_id, receiver);
    Ok(Message::DownloadPlaylistStarted {
        id,
        receiver_handle,
    })
}

pub async fn stop_playlist_download(id: Id, playlist_sender: PlaylistSender) -> anyhow::Result<()> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::CancelDownloadPlaylist {
            id,
            result_sender: tx,
        })
        .await?;
    let _ = rx.await??;
    Ok(())
}

pub async fn shuffle_playlist(
    id: Id,
    playlist_sender: PlaylistSender,
    tracklist: Option<TrackList>,
) -> anyhow::Result<TrackList> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::ShufflePlaylist {
            playlist_id: id,
            result_sender: tx,
            tracklist,
        })
        .await?;

    let tracklist = rx.await?;
    Ok(tracklist)
}

pub async fn organize_playlist(
    id: Id,
    playlist_sender: PlaylistSender,
    tracklist: Option<TrackList>,
) -> anyhow::Result<TrackList> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::OrganizePlaylist {
            playlist_id: id,
            result_sender: tx,
            tracklist,
        })
        .await?;

    let tracklist = rx.await?;
    Ok(tracklist)
}

pub async fn play_playlist(
    playlist_id: Id,
    task_id: u64,
    playlist_sender: PlaylistSender,
    tracklist: Option<TrackList>,
) -> anyhow::Result<ReceiverHandle<Message>> {
    // create a receiver handle for progress updates
    let (tx, rx) = mpsc::channel(100);
    let handle = ReceiverHandle::new(task_id, rx);

    playlist_sender
        .send(PlaylistMessage::PlayPlaylist {
            id: playlist_id,
            tracklist,
            data_sender: tx,
        })
        .await?;
    Ok(handle)
}

pub async fn pause_current_playlist_track(
    playlist_id: Id,
    playlist_sender: PlaylistSender,
) -> anyhow::Result<()> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::PauseCurrentTrack {
            playlist_id,
            result_sender: tx,
        })
        .await
        .unwrap();
    let _ = rx.await?;
    Ok(())
}

pub async fn resume_current_playlist_track(
    playlist_id: Id,
    playlist_sender: PlaylistSender,
    seek_location: Option<f32>,
) -> anyhow::Result<()> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::ResumeCurrentTrack {
            playlist_id,
            result_sender: tx,
            seek_location,
        })
        .await
        .unwrap();
    let _ = rx.await?;
    Ok(())
}

pub async fn skip_current_playlist_track(
    playlist_id: Id,
    playlist_sender: PlaylistSender,
) -> anyhow::Result<()> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::SkipCurrentTrack {
            playlist_id,
            result_sender: tx,
        })
        .await
        .unwrap();
    let _ = rx.await?;
    Ok(())
}

pub async fn previous_current_playlist_track(
    playlist_id: Id,
    playlist_sender: PlaylistSender,
) -> anyhow::Result<()> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::PreviousCurrentTrack {
            playlist_id,
            result_sender: tx,
        })
        .await
        .unwrap();
    let _ = rx.await?;
    Ok(())
}

pub async fn play_track_in_playlist(
    playlist_id: Id,
    playlist_sender: PlaylistSender,
    track_index: u64,
) -> anyhow::Result<()> {
    let (tx, rx) = oneshot::channel();
    let _ = playlist_sender
        .send(PlaylistMessage::SelectPlaylistIndex {
            playlist_id,
            track_index,
            result_sender: tx,
        })
        .await;

    let _ = rx.await?;
    Ok(())
}

pub async fn set_playlist_loop_policy(
    playlist_id: Id,
    policy: LoopPolicy,
    playlist_sender: PlaylistSender,
) -> anyhow::Result<()> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::SetPlaylistLoopPolicy {
            playlist_id,
            policy,
            result_sender: tx,
        })
        .await
        .unwrap();
    let _ = rx.await?;
    Ok(())
}

pub async fn update_volume_in_playlist_service(
    volume: f64,
    playlist_sender: PlaylistSender,
) -> anyhow::Result<()> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::UpdateGlobalVolume {
            volume,
            result_sender: tx,
        })
        .await
        .unwrap();
    let _ = rx.await?;
    Ok(())
}

pub fn format_duration(duration: &Duration) -> String {
    let total_seconds = duration.as_secs();
    let mins = total_seconds / 60;
    let secs = total_seconds - mins * 60;
    format!("{}:{:02}", mins, secs)
}
pub fn get_u64_digit_count(mut num: u64) -> u64 {
    if num == 0 {
        return 0;
    }
    let mut count = 0;
    while num != 0 {
        num /= 10;
        count += 1;
    }
    count
}

pub fn sort_playlist_metadata(metadata_vec: &mut Vec<PlaylistMetadata>) {
    metadata_vec.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
}
pub fn update_recent_playlists(
    recent_playlists: &mut VecDeque<PlaylistMetadata>,
    new_metadata: PlaylistMetadata,
) {
    if let Some(i) = recent_playlists.iter().position(|x| x == &new_metadata) {
        recent_playlists.remove(i);
    }
    recent_playlists.push_front(new_metadata);
    if recent_playlists.len() > super::RECENT_PLAYLIST_SIZE {
        recent_playlists.pop_back();
    }
}
pub fn generate_playlist_list(app: &App) -> impl Iterator<Item = &PlaylistMetadata> {
    app.recent_playlists.iter().chain(
        app.loaded_playlist_metadata
            .iter()
            .take(super::RECENT_PLAYLIST_SIZE - app.recent_playlists.len())
            .filter(|m| !app.recent_playlists.contains(m)),
    )
}
pub fn sort_albums(albums: &mut Vec<Album>) {
    albums.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
}
pub fn track_contains_search_term(track: &Track, term: &str) -> bool {
    if !term.trim().is_empty() {
        let term = term.to_lowercase();
        // test 1: term is in title
        if track.title.to_lowercase().contains(&term) {
            return true;
        }
        // test 2: term is in artist name
        match &track.artist {
            Artist::Community(uploader) => {
                if uploader.to_lowercase().contains(&term) {
                    return true;
                }
            }
            // if the search term is found in any of the artists names
            Artist::Official(artists) => {
                if artists.join("").to_lowercase().contains(&term) {
                    return true;
                }
            }
        }
        // test 3: term is in album name
        match &track.album_kind {
            AlbumKind::Album(album) => {
                if album.name.to_lowercase().contains(&term) {
                    return true;
                }
            }
            _ => {}
        }
    } else {
        return true;
    }
    false
}
