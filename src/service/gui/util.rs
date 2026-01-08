// page factory functions

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
use crate::service::playlist::structs::{Playlist, TrackMetadata};

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
    if let None = app.current_playlist {
        return column![text(
            "somehow there's no playlist to load on this screen lol"
        )];
    }
    let current_playlist = app.current_playlist.as_ref().unwrap();

    let title = text(&current_playlist.title);
    let home_button = button("home").on_press(Message::Action(Action::Home));
    let header = row![home_button, title];

    let tracklist = current_playlist.tracks.iter().map(|track| {
        // create a metadata object for each track to know when important information changes between renders
        let track_downloaded = app.downloaded_tracks.contains(&track.id());
        let track_metadata = TrackMetadata {
            downloaded: track_downloaded,
            title: Arc::from(track.title.as_str()),
        };
        // create a lazy button
        lazy(track_metadata, |metadata| {
            button(text(metadata.title.to_string()))
        })
        .into()
    });
    let album_next = row![scrollable(column(tracklist))].height(Length::Fill);

    let current_playlist_id = app.current_playlist.as_ref().unwrap().id();
    let controls = row![
        // all the buttons lol
        button("SODAA 🗣🔥").on_press(Message::Action(Action::DownloadPlaylist {
            playlist_id: current_playlist_id.clone()
        })),
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
) -> anyhow::Result<Option<Playlist>> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::RequestPlaylist {
            id,
            result_sender: tx,
        })
        .await?;
    rx.await.map_err(|err| anyhow::Error::from(err))
}
