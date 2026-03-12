use iced::Theme;
use iced::widget::column;
use indexmap::IndexMap;

use crate::service::{
    gui::{
        util::truncate_with_ellipsis,
        widgets::{
            notification::{NOTIFICATION_PROGRESS_BAR_HEIGHT, Notification},
            progress_bar::default_progress_bar,
            text::default_text,
        },
    },
    id::structs::Id,
    playlist::structs::TrackDownloadData,
};

pub fn download_notification<'a>(
    track_download_data: &TrackDownloadData,
    theme: &Theme,
) -> Notification<'a> {
    const MAX_TRACK_CHARS: usize = 35;
    let shortened_track_title =
        truncate_with_ellipsis(&track_download_data.track.title, MAX_TRACK_CHARS);
    let title = default_text(
        format!("Downloading {}", shortened_track_title),
        theme,
        true,
        true,
    );
    let progress_bar = default_progress_bar(
        0.0..=100.0,
        track_download_data.progress.unwrap_or(0.0),
        theme,
    )
    .girth(NOTIFICATION_PROGRESS_BAR_HEIGHT);
    let content = column![title, progress_bar].into();
    Notification { content }
}

pub fn download_notification_list<'a>(
    downloading_track_data: &IndexMap<Id, TrackDownloadData>,
    theme: &Theme,
) -> Vec<Notification<'a>> {
    downloading_track_data
        .iter()
        .map(|(_, tdata)| download_notification(&tdata, theme))
        .collect()
}
