use crate::service::audio::enums::LoopPolicy;
use crate::service::file::enums::TrackDownloadState;
use crate::service::gui::enums::{Action, Message};
use crate::service::gui::structs::TrackRenderData;
use crate::service::gui::util::{self, format_duration};
use iced::Length::FillPortion;
use iced::widget::{
    Column, Space, button, column, container, lazy, row, scrollable, slider, space, text,
    text_input,
};
use iced::{Alignment, Length};

use super::App;

// page factory functions

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
    // DATA COLLECTION //

    let current_owned_playlist = match &app.current_owned_playlist {
        None => {
            return column![text(
                "somehow there's no playlist to load on this screen lol"
            )];
        }
        Some(p) => p,
    };
    let current_tracklist = match &app.current_playlist_tracklist {
        None => return column![text("Somehow there's no tracklist for this page :p")],
        Some(t) => t,
    };
    let current_playlist_id = current_owned_playlist.metadata.id().clone();
    let current_playlist_playing = app.paused_playlists.contains(&current_playlist_id);
    let track_count_digits =
        util::get_u64_digit_count(current_owned_playlist.length() as u64) as usize;

    // PAGE BUILDING //

    let title = text(&current_owned_playlist.metadata.title);
    let home_button = button("home").on_press(Message::Action(Action::Home));
    let header = row![home_button, title];

    let tracklist = current_tracklist.iter().enumerate().map(|(index, track)| {
        // create a metadata object for each track to know when important information changes between renders
        let track_downloaded = app.downloaded_tracks.contains(&track.id());
        let track_downloading = app.downloading_tracks.contains(&track.id());
        let track_download_state = if track_downloading {
            TrackDownloadState::Downloading
        } else if track_downloaded {
            TrackDownloadState::Downloaded
        } else {
            TrackDownloadState::NotDownloaded
        };
        let pid = current_playlist_id.clone();
        let render_data = TrackRenderData {
            download_state: track_download_state,
            title: track.title.clone(),
            index: index as u64,
        };
        // create a lazy button
        lazy(render_data, move |render_data| {
            row![
                text(format!("{:0>track_count_digits$}.", render_data.index + 1)),
                button(text(format!(
                    "{}{}",
                    render_data.title.to_string(),
                    match render_data.download_state {
                        TrackDownloadState::NotDownloaded => "",
                        TrackDownloadState::Downloading => " ⬇️",
                        TrackDownloadState::Downloaded => " ✅",
                    }
                )))
                .on_press(Message::Action(Action::PlayTrack {
                    playlist_id: pid.clone(),
                    track_index: index as u64,
                }))
                .width(Length::Fill)
            ]
            .align_y(Alignment::Center)
        })
        .into()
    });
    let tracklist_container =
        row![scrollable(column(tracklist).width(Length::Fill))].height(Length::Fill);

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

    let loop_button_text = format!("loop{}", {
        match app.playlist_loop_policies.get(&current_playlist_id) {
            Some(policy) => match policy {
                LoopPolicy::NoLooping => "",
                LoopPolicy::Once => " (1x)",
                LoopPolicy::Infinite => " (inf)",
            },
            None => "",
        }
    });

    let volume_text = text("volume?");
    let volume_bar = slider(0.0..=100.0, app.volume * 100.0, |volume| {
        Message::Action(Action::SetVolume {
            volume: volume / 100.0,
        })
    });

    let volume_interface = row![volume_text, volume_bar];

    let controls_row = row![
        // all the buttons lol
        download_button,
        button("orgnze").on_press(Message::Action(Action::OrganizePlaylist {
            playlist_id: current_playlist_id.clone()
        })),
        button("prev").on_press(Message::Action(Action::PreviousTrack {
            playlist_id: current_playlist_id.clone()
        })),
        if app.playing_playlists.contains(&current_playlist_id) {
            button("pause").on_press(Message::Action(Action::PauseTrack {
                playlist_id: current_playlist_id.clone(),
            }))
        } else if current_playlist_playing {
            button("play").on_press(Message::Action(Action::ResumeTrack {
                playlist_id: current_playlist_id.clone(),
            }))
        } else {
            button("...")
        },
        button("nxt").on_press(Message::Action(Action::NextTrack {
            playlist_id: current_playlist_id.clone()
        })),
        button("shffle").on_press(Message::Action(Action::ShufflePlaylist {
            playlist_id: current_playlist_id.clone()
        })),
        button(text(loop_button_text)).on_press(Message::Action(Action::LoopTrack {
            playlist_id: current_playlist_id.clone()
        })),
    ];
    let main_controls = container(controls_row).width(Length::Shrink);
    // Left and right share remaining space equally
    let left_controls = row![
        volume_interface.width(Length::FillPortion(1)),
        space().width(FillPortion(1))
    ]
    .width(Length::FillPortion(1));
    let right_controls = space().width(Length::FillPortion(1));
    let controls = row![left_controls, main_controls, right_controls];

    let current_playlist_id_clone = current_playlist_id.clone();
    let progress_bar_slider = slider(
        0.0..=100.0,
        app.track_progress.progress() * 100.0,
        move |progress| {
            Message::Action(Action::SeekAudio {
                playlist_id: current_playlist_id.clone(),
                progress: progress / 100.0,
            })
        },
    )
    .on_release(Message::Action(Action::StopSeekingAudio {
        playlist_id: current_playlist_id_clone,
    }));

    let current_time = app.track_progress.current();
    let total_time = app.track_progress.total();

    let current_txt = text(format_duration(current_time));
    let total_txt = text(format_duration(total_time));

    let progress_bar = row![current_txt, progress_bar_slider, total_txt];

    column![header, tracklist_container, progress_bar, controls].height(Length::Fill)
}
