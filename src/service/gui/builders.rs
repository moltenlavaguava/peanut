use crate::service::audio::enums::{AlbumKind, LoopPolicy};
use crate::service::file;
use crate::service::file::enums::TrackDownloadState;
use crate::service::gui::enums::{Action, Message};
use crate::service::gui::util::{self, format_duration};
use crate::service::gui::widgetbuilder::one_line_text;
use crate::service::playlist::enums::Artist;
use crate::service::playlist::structs::Track;
use iced::Length::FillPortion;
use iced::widget::{
    self, Column, Image, Row, Space, button, column, container, pick_list, responsive, row,
    scrollable, slider, space, text, text_input,
};
use iced::{Alignment, Color, Length, Pixels, Theme};

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

    // LEFT MENU BAR

    let home_button = button("Home").on_press(Message::Action(Action::Home));
    // assemble left menu
    let mut left_menu = Column::new().width(Length::Fixed(200.0));
    left_menu = left_menu
        .push(home_button)
        .push(text("Playlists"))
        // add up to 3 playlists
        .extend(
            util::generate_playlist_list(app).map(|metadata| {
                one_line_text(metadata.title.clone(), Pixels(20.0), 16.0, true).into()
            }),
        )
        .push(text("Albums"))
        // add up to 3 albums
        .extend(
            app.downloaded_albums
                .iter()
                .take(super::ALBUM_DISPLAY_SIZE)
                .map(|album| one_line_text(album.name.clone(), Pixels(20.0), 16.0, true).into()),
        )
        .push(text("Browse"))
        .push(text("About"));

    // PLAYLIST INFO (and search bar)

    let title = text(&current_owned_playlist.metadata.title);
    let search_bar = text_input("Search..", &app.track_search_text)
        .on_input(Message::TrackSearchTextEdit)
        .on_paste(Message::TrackSearchTextEdit);
    let theme_changer = pick_list(Theme::ALL, Some(&app.theme), |theme| {
        Message::ThemeUpdated { theme }
    });
    let playlist_info = row![title, search_bar, theme_changer];

    // TRACK DATA

    // data retrieval
    let scroll_offset = {
        if let Some(scroll) = app.playlist_scrolloffsets.get(&current_playlist_id) {
            *scroll
        } else {
            0.0
        }
    };
    let playlist_id_clone = current_playlist_id.clone();

    let filtered_tracklist: Vec<(usize, &Track)> = current_tracklist
        .iter()
        .enumerate()
        .filter(|(_, track)| util::track_contains_search_term(track, &app.track_search_text))
        .collect();
    let filtered_length = filtered_tracklist.len();

    let tracklist = responsive(move |size| {
        let viewport_height = size.height;
        const TRACK_ENTRY_HEIGHT: f32 = 20.0;

        let total_content_height = filtered_length as f32 * TRACK_ENTRY_HEIGHT;
        let max_scroll = (total_content_height - viewport_height).max(0.0);
        let safe_scroll_offset = scroll_offset.min(max_scroll).max(0.0);
        let mut start_index = (safe_scroll_offset / TRACK_ENTRY_HEIGHT).floor() as usize;

        // if no scrolling is necessary, then don't have start offset
        let max_tracks = (viewport_height / TRACK_ENTRY_HEIGHT).floor() as usize;
        if filtered_length <= max_tracks {
            start_index = 0;
        }

        let visible_count = (viewport_height / TRACK_ENTRY_HEIGHT).ceil() as usize;

        // Ensure we don't go out of bounds
        let end_index = (start_index + visible_count + 4).min(filtered_length);

        let top_height = start_index as f32 * TRACK_ENTRY_HEIGHT;
        let bottom_spacer_height =
            (filtered_length.saturating_sub(end_index)) as f32 * TRACK_ENTRY_HEIGHT;

        // create virtual list
        let visible_items = column(filtered_tracklist[start_index..end_index].iter().map(
            |(index, track)| {
                // create the button
                let artist_name = match &track.artist {
                    Artist::Community(name) => name.clone(),
                    Artist::Official(names) => names.join(", "),
                };
                let track_length = util::format_duration(&track.length);
                let track_downloaded = app.downloaded_tracks.contains(&track.id());
                let track_downloading = app.downloading_tracks.contains(&track.id());
                let track_download_state = if track_downloading {
                    TrackDownloadState::Downloading
                } else if track_downloaded {
                    TrackDownloadState::Downloaded
                } else {
                    TrackDownloadState::NotDownloaded
                };
                // create the button
                button(row![
                    // Track index text
                    one_line_text(
                        &format!("{:0>track_count_digits$}.", index.clone() + 1),
                        Pixels(20.0),
                        16.0,
                        true
                    )
                    .width(Length::FillPortion(1)),
                    // Track title text
                    one_line_text(track.title.to_string(), Pixels(20.0), 16.0, true)
                        .width(Length::Fill)
                        .width(Length::FillPortion(8)),
                    // Track artist text
                    one_line_text(artist_name.clone(), Pixels(20.0), 16.0, true)
                        .width(Length::FillPortion(5)),
                    // Track length text
                    one_line_text(track_length, Pixels(20.0), 16.0, true)
                        .width(Length::FillPortion(2)),
                    // Track status text
                    one_line_text(
                        match track_download_state {
                            TrackDownloadState::NotDownloaded => "",
                            TrackDownloadState::Downloading => " ⬇️",
                            TrackDownloadState::Downloaded => " ✅",
                        },
                        Pixels(20.0),
                        16.0,
                        true
                    )
                    .width(Length::FillPortion(2)),
                    // TODO: Track action buttons
                    text("").width(Length::FillPortion(3))
                ])
                .style(|_theme, _status| widget::button::Style {
                    background: None,
                    text_color: Color::from_rgb(1.0, 1.0, 1.0),
                    ..widget::button::Style::default()
                })
                .on_press(Message::Action(Action::PlayTrack {
                    playlist_id: playlist_id_clone.clone(),
                    track_index: *index as u64,
                }))
                .into()
            },
        ));

        let playlist_id_clone = playlist_id_clone.clone();
        scrollable(column![
            space().height(Length::Fixed(top_height)),
            visible_items,
            space().height(Length::Fixed(bottom_spacer_height)),
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .on_scroll(move |v| Message::TracklistScrolled {
            playlist_id: playlist_id_clone.clone(),
            scrollable_viewport: v,
        })
        .into()
    });

    // create header for tracks
    let tracks_header = row![
        one_line_text("#", Pixels(20.0), 16.0, true).width(Length::FillPortion(1)),
        one_line_text("Name", Pixels(20.0), 16.0, true).width(Length::FillPortion(8)),
        one_line_text("Artist", Pixels(20.0), 16.0, true).width(Length::FillPortion(5)),
        one_line_text("Length", Pixels(20.0), 16.0, true).width(Length::FillPortion(2)),
        one_line_text("Status", Pixels(20.0), 16.0, true).width(Length::FillPortion(2)),
        one_line_text("Actions", Pixels(20.0), 16.0, true).width(Length::FillPortion(3)),
    ];
    let track_data = column![tracks_header, tracklist];

    // LOWER CONTROLS

    let volume_text = text("volume?");
    let volume_bar = slider(0.0..=100.0, app.volume * 100.0, |volume| {
        Message::Action(Action::SetVolume {
            volume: volume / 100.0,
        })
    });

    let volume_interface = row![volume_text, volume_bar];

    // buttons

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
    let loop_button = button(text(loop_button_text)).on_press(Message::Action(Action::LoopTrack {
        playlist_id: current_playlist_id.clone(),
    }));
    let shuffle_button = button("shffle").on_press(Message::Action(Action::ShufflePlaylist {
        playlist_id: current_playlist_id.clone(),
    }));
    let organize_button = button("orgnze").on_press(Message::Action(Action::OrganizePlaylist {
        playlist_id: current_playlist_id.clone(),
    }));
    let previous_button = button("prev").on_press(Message::Action(Action::PreviousTrack {
        playlist_id: current_playlist_id.clone(),
    }));
    let next_button = button("nxt").on_press(Message::Action(Action::NextTrack {
        playlist_id: current_playlist_id.clone(),
    }));
    let play_button = if app.playing_playlists.contains(&current_playlist_id) {
        button("pause").on_press(Message::Action(Action::PauseTrack {
            playlist_id: current_playlist_id.clone(),
        }))
    } else if current_playlist_playing {
        button("play").on_press(Message::Action(Action::ResumeTrack {
            playlist_id: current_playlist_id.clone(),
        }))
    } else {
        button("...")
    };

    // progress bar
    let current_time = app.track_progress.current();
    let total_time = app.track_progress.total();

    let current_txt = text(format_duration(current_time));
    let total_txt = text(format_duration(total_time));

    let current_playlist_id_clone = current_playlist_id.clone();
    let progress_bar_slider = slider(
        0.0..=100.0,
        app.track_progress.progress() * 100.0,
        move |progress| {
            Message::Action(Action::SeekAudio {
                playlist_id: current_playlist_id_clone.clone(),
                progress: progress / 100.0,
            })
        },
    )
    .on_release(Message::Action(Action::StopSeekingAudio {
        playlist_id: current_playlist_id.clone(),
    }));

    let progress_bar = row![current_txt, progress_bar_slider, total_txt];

    // building lower middle portion
    let core_buttons = row![
        download_button,
        organize_button,
        previous_button,
        play_button,
        next_button,
        shuffle_button,
        loop_button,
    ];
    let core_and_progress = column![progress_bar, core_buttons];

    let lower_middle = container(core_and_progress).width(Length::Shrink);

    // Left and right share remaining space equally
    // left: album cover, track title, and artist
    let mut left_controls = Row::new().width(Length::FillPortion(1));
    let current_track_id = app.playlist_playling_tracks.get(&current_playlist_id);
    if let Some(track) = current_track_id {
        // album cover first
        match &track.album_kind {
            AlbumKind::Album(album) => {
                let album_cover_path = file::util::album_filename_from_id(&album.id());
                if let Ok(path) = album_cover_path {
                    // make image with static size
                    let img = Image::new(path)
                        .width(Length::Fixed(75.0))
                        .height(Length::Fixed(75.0));
                    left_controls = left_controls.push(img);
                }
            }
            _ => {}
        }
        // then other text
        let track_artist = match &track.artist {
            Artist::Community(uploader) => uploader.to_string(),
            Artist::Official(a) => a.join(", "),
        };
        let track_info = container(column![
            one_line_text(&track.title, Pixels(20.0), 16.0, true),
            one_line_text(track_artist, Pixels(15.0), 12.0, true)
        ])
        .align_y(Alignment::Center);
        left_controls = left_controls.push(track_info)
    }

    let right_controls = row![
        space().width(FillPortion(1)),
        volume_interface.width(Length::FillPortion(1)),
    ];
    let controls = row![left_controls, lower_middle, right_controls].align_y(Alignment::Center);

    let lower_controls = column![controls];

    // put it all together

    let playlist_and_track_data = column![playlist_info, track_data];
    let upper_portion = row![left_menu, playlist_and_track_data];
    let page = column![upper_portion, lower_controls];

    page
}
