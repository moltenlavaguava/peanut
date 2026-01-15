// page factory functions

use std::collections::HashSet;
use std::sync::Arc;

use iced::Length;
use iced::widget::{
    Column, Space, button, column, container, lazy, row, scrollable, text, text_input,
};
use tokio::sync::oneshot;

use crate::service::gui::enums::{Action, PlayingState};
use crate::service::id::structs::Id;
use crate::service::playlist::PlaylistSender;
use crate::service::playlist::enums::PlaylistMessage;
use crate::service::playlist::structs::{TrackList, TrackMetadata};
use crate::util::sync::ReceiverHandle;

use super::App;

use super::enums::Message;

pub fn home(app: &App) -> Column<'_, Message> {
    let title_text = text("peanut v0.00069?");

    let header = row![title_text, Space::new().width(Length::Fill),];

    let load_file = button("init playlist").on_press(Message::PlaylistURLSubmit);
    let playlist_url = text_input("file path", &app.playlist_url)
        .width(Length::Fill)
        .on_input(Message::PlaylistTextEdit)
        .on_paste(Message::PlaylistTextEdit)
        .on_submit(Message::PlaylistURLSubmit);

    let content = container(column(app.loaded_playlist_metadata.iter().map(
        |metadata| {
            button(text(&metadata.title))
                .on_press(Message::PlaylistSelect(metadata.clone()))
                .into()
        },
    )))
    .width(Length::Fill)
    .height(Length::Fill);

    let footer_text = text("unused text at the bottom :p");

    let footer = row![footer_text];

    column![header, row![playlist_url, load_file], content, footer]
}

pub fn player(app: &App) -> Column<'_, Message> {
    if let None = app.current_ptracklist {
        return column![text(
            "somehow there's no playlist to load on this screen lol"
        )];
    }
    let current_ptracklist = app.current_ptracklist.as_ref().unwrap();

    let title = text(&current_ptracklist.metadata.title);
    let home_button = button("home").on_press(Message::Action(Action::Home));
    let header = row![home_button, title];

    let tracklist = current_ptracklist.list.iter().map(|track| {
        // create a metadata object for each track to know when important information changes between renders
        let track_downloaded = app.downloaded_tracks.contains(&track.id());
        let track_downloading = app.downloading_tracks.contains(&track.id());
        let track_metadata = TrackMetadata {
            downloaded: track_downloaded,
            downloading: track_downloading,
            title: Arc::from(track.title.as_str()),
        };
        // create a lazy button
        lazy(track_metadata, |metadata| {
            button(text(format!(
                "{}{}",
                metadata.title.to_string(),
                if metadata.downloading {
                    " ⬇️"
                } else if metadata.downloaded {
                    " ✅"
                } else {
                    ""
                }
            )))
        })
        .into()
    });
    let album_next = row![scrollable(column(tracklist))].height(Length::Fill);

    let current_playlist_id = app.current_ptracklist.as_ref().unwrap().metadata.id.clone();

    let download_button = if app.downloading_playlists.contains(&current_playlist_id) {
        button("stop download").on_press(Message::Action(Action::StopPlaylistDownload {
            playlist_id: current_playlist_id.clone(),
        }))
    } else if app
        .download_stopping_playlists
        .contains(&current_playlist_id)
    {
        button("stopping..")
    } else {
        button("SODAA 🗣🔥").on_press(Message::Action(Action::DownloadPlaylist {
            playlist_id: current_playlist_id.clone(),
        }))
    };

    let controls = row![
        // all the buttons lol
        download_button,
        button("orgnze").on_press(Message::Action(Action::OrganizePlaylist {
            playlist_id: current_playlist_id.clone()
        })),
        button("prev").on_press(Message::Action(Action::PreviousTrack {
            playlist_id: current_playlist_id.clone()
        })),
        if let PlayingState::Playing = app.track_playing_state {
            button("pause").on_press(Message::Action(Action::PlayTrack {
                playlist_id: current_playlist_id.clone(),
            }))
        } else {
            button("play").on_press(Message::Action(Action::PauseTrack {
                playlist_id: current_playlist_id.clone(),
            }))
        },
        button("nxt").on_press(Message::Action(Action::NextTrack {
            playlist_id: current_playlist_id.clone()
        })),
        button("shffle").on_press(Message::Action(Action::ShufflePlaylist {
            playlist_id: current_playlist_id.clone()
        })),
        button("lop").on_press(Message::Action(Action::LoopTrack {
            playlist_id: current_playlist_id.clone()
        })),
    ];
    column![header, album_next, controls].height(Length::Fill)
}

// requesting methods
pub async fn request_playlist(
    id: Id,
    playlist_sender: PlaylistSender,
) -> anyhow::Result<Option<TrackList>> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::RequestTracklist {
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
