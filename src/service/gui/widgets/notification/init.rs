use crate::service::gui::structs::{PlaylistInitData, PlaylistInitId};
use crate::service::gui::widgets::notification::{NOTIFICATION_PROGRESS_BAR_HEIGHT, Notification};
use crate::service::gui::widgets::progress_bar::default_progress_bar;
use crate::service::gui::widgets::text::default_text;
use iced::Theme;
use iced::widget::column;
use indexmap::IndexMap;

pub fn initialization_notification<'a>(
    playlist_init_data: &PlaylistInitData,
    theme: &Theme,
) -> Notification<'a> {
    let ct = match playlist_init_data.current_init_track_count {
        Some(v) => v.to_string(),
        None => String::from("??"),
    };
    let tt = match playlist_init_data.total_track_count {
        Some(v) => v.to_string(),
        None => String::from("??"),
    };
    let title = default_text(
        format!("Loading Playlist... ({}/{})", ct, tt),
        theme,
        true,
        true,
    );
    let bar_progress = {
        let ct = match playlist_init_data.current_init_track_count {
            Some(v) => v,
            None => 0,
        };
        let tt = match playlist_init_data.total_track_count {
            Some(v) => v,
            None => 1,
        };
        (ct as f32) / (tt as f32)
    };
    let progress = default_progress_bar(0.0..=100.0, bar_progress * 100.0, theme)
        .girth(NOTIFICATION_PROGRESS_BAR_HEIGHT);
    let content = column![title, progress].into();
    Notification { content }
}

pub fn initialization_notification_list<'a>(
    playlist_init_data: &'a IndexMap<PlaylistInitId, PlaylistInitData>,
    theme: &Theme,
) -> Vec<Notification<'a>> {
    playlist_init_data
        .iter()
        .map(|(_, idata)| initialization_notification(idata, theme))
        .collect()
}
