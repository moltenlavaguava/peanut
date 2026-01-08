// page factory functions

use iced::Length;
use iced::widget::{Column, Space, button, column, container, row, scrollable, text, text_input};
use tokio::sync::oneshot;

use crate::service::id::structs::Id;
use crate::service::playlist::PlaylistSender;
use crate::service::playlist::enums::PlaylistMessage;
use crate::service::playlist::structs::Playlist;

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
    let header = row![title];

    let tracklist = current_playlist
        .tracks
        .iter()
        .map(|track| button(&*track.title).into());
    let album_next = row![scrollable(column(tracklist))].height(Length::Fill);

    let controls = row![
        // all the buttons lol
        button("SODAA 🗣🔥"),
        button("orgnze"),
        button("prev"),
        button("ply"),
        button("nxt"),
        button("shffle"),
        button("lop"),
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
