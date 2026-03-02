use crate::service::gui::enums::{Action, Message, PlayingState};
use crate::service::gui::icons;
use crate::service::gui::styling::AppTheme;
use crate::service::gui::widgets::button::invisible_button;
use crate::service::gui::widgets::notification::NOTIFICATION_PROGRESS_BAR_HEIGHT;
use crate::service::gui::widgets::progress_bar::default_progress_bar;
use crate::service::gui::widgets::text::{default_text, icon_text};
use crate::service::{
    gui::{structs::PlaylistRenderData, widgets::notification::Notification},
    id::structs::Id,
};
use iced::widget::{column, container, row};
use iced::{Alignment, Length, Theme};
use indexmap::IndexMap;

pub fn playing_notification<'a>(
    playlist_render_data: &PlaylistRenderData,
    theme: &Theme,
) -> Notification<'a> {
    let current_track_text = match &playlist_render_data.current_track {
        Some(t) => &t.title,
        None => "",
    }; // build content for notification
    let title = default_text(
        format!(
            "{} ({})",
            current_track_text, &playlist_render_data.owned_playlist.metadata.title
        ),
        theme,
        true,
        true,
    );
    let progress = default_progress_bar(
        0.0..=100.0,
        playlist_render_data.playing_track_progress.progress() * 100.0,
        theme,
    )
    .girth(NOTIFICATION_PROGRESS_BAR_HEIGHT);

    let default_text_style = theme.stylesheet().default_text(true, true);
    let previous_button = invisible_button(
        icon_text(icons::PREVIOUS, default_text_style).size(20.0),
        theme,
    )
    .on_press(Message::Action(Action::PreviousTrack {
        playlist_id: playlist_render_data.owned_playlist.metadata.id().clone(),
    }));
    let next_button =
        invisible_button(icon_text(icons::SKIP, default_text_style).size(20.0), theme).on_press(
            Message::Action(Action::NextTrack {
                playlist_id: playlist_render_data.owned_playlist.metadata.id().clone(),
            }),
        );
    let play_button = if let PlayingState::Playing = playlist_render_data.playing_state {
        invisible_button(
            icon_text(icons::PAUSE, default_text_style).size(20.0),
            theme,
        )
        .on_press(Message::Action(Action::PauseTrack {
            playlist_id: playlist_render_data.owned_playlist.metadata.id().clone(),
        }))
    } else if matches!(playlist_render_data.playing_state, PlayingState::Paused)
        || matches!(playlist_render_data.playing_state, PlayingState::Seeking)
    {
        invisible_button(icon_text(icons::PLAY, default_text_style).size(20.0), theme).on_press(
            Message::Action(Action::ResumeTrack {
                playlist_id: playlist_render_data.owned_playlist.metadata.id().clone(),
            }),
        )
    } else {
        invisible_button(icon_text(icons::PLAY, default_text_style).size(20.0), theme)
    };
    let actions = container(row![previous_button, play_button, next_button])
        .width(Length::Fill)
        .align_x(Alignment::Center);
    let content = column![title, progress, actions].into();
    Notification { content }
}

pub fn playing_notification_list<'a>(
    playlist_render_data: &IndexMap<Id, PlaylistRenderData>,
    theme: &Theme,
) -> Vec<Notification<'a>> {
    playlist_render_data
        .iter()
        .map(|(_, rdata)| playing_notification(rdata, theme))
        .collect()
}
