// page factory functions

use iced::Length;
use iced::widget::{Column, Space, button, column, container, progress_bar, row, text, text_input};

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

    let prog_bar = progress_bar(
        0.0..=app.total_tracks as f32,
        app.current_track_index as f32,
    );
    let text_prog = text(format!(
        "{}/{} tracks init'd",
        app.current_track_index, app.total_tracks
    ));

    let prog_row = row![prog_bar, text_prog];

    let content = container(column![row![playlist_url, load_file], prog_row])
        .width(Length::Fill)
        .height(Length::Fill);

    let footer_text = text("unused text at the bottom :p");

    let footer = row![footer_text];

    column![header, content, footer]
}

pub fn player(app: &App) -> Column<'_, Message> {
    column![]
}
