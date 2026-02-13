use super::App;
use crate::service::audio::enums::AlbumKind;
use crate::service::file;
use crate::service::file::enums::TrackDownloadState;
use crate::service::gui::enums::{Action, DownloadState, Message, Page, PlayingState};
use crate::service::gui::icons::{self};
use crate::service::gui::styling::AppTheme;
use crate::service::gui::util::{self, format_duration};
use crate::service::gui::widgets;
use crate::service::gui::widgets::button::invisible_button_padded;
use crate::service::gui::widgets::rule::{default_horizontal_rule, in_between_rule};
use crate::service::gui::widgets::scrollable::virtualized_vertical_scrollable;
use crate::service::gui::widgets::text::icon;
use crate::service::playlist::enums::Artist;
use crate::service::playlist::structs::{Album, PlaylistMetadata, Track};
use iced::widget::{Column, Container, Image, Row, column, container, row, space, text};
use iced::{Alignment, Length, Padding, Theme};
use widgets::button::{
    default as default_button, default_text as default_text_button, invisible as invisible_button,
    track_button,
};
use widgets::container::{
    home_menu_widget_container, main_content as main_content_container,
    menu_content as menu_content_container,
};
use widgets::slider::default_slider;
use widgets::text::{
    default as default_text, left_menu_bold as left_menu_bold_text,
    left_menu_sub as left_menu_sub_text, secondary as secondary_text, title as title_text,
};
use widgets::text_input::default_text_input;

// page factory functions

pub fn home(app: &App) -> Container<'_, Message> {
    let theme = &app.theme;
    let title_txt = title_text("Home", theme, true, true);
    const WIDGET_SPACING: f32 = 10.0;

    let load_playlist = default_text_button("Load", theme).on_press(Message::PlaylistURLSubmit);
    let playlist_url = default_text_input(
        "Youtube playlist URL",
        &app.home_playlists_widget_data.search_text,
        theme,
    )
    .width(Length::Fill)
    .on_input(Message::PlaylistTextEdit)
    .on_paste(Message::PlaylistTextEdit)
    .on_submit(Message::PlaylistURLSubmit);

    let upper_menu_content = menu_content_container(
        column![title_txt, default_horizontal_rule(2, theme)].spacing(4),
        theme,
    )
    .padding(
        Padding::new(4.0)
            .horizontal(4.0 + WIDGET_SPACING)
            .bottom(0.0),
    );

    let playlist_count = app.general_cache.all_playlist_metadata.len();
    let playlists = if playlist_count > 0 {
        let playlists_info = row![
            default_text("Name", theme, true, true).width(Length::FillPortion(5)),
            default_text("Track Count", theme, true, true).width(Length::FillPortion(2)),
            default_text("Length", theme, true, true).width(Length::FillPortion(1)),
            space().width(Length::FillPortion(2)),
        ]
        .spacing(4.0);

        let playlist_data_closure = move |i: usize, metadata: &PlaylistMetadata, theme: &Theme| {
            in_between_rule(
                track_button(
                    row![
                        default_text(metadata.title.clone(), theme, true, true)
                            .width(Length::FillPortion(5)),
                        default_text(&metadata.track_count, theme, true, true)
                            .width(Length::FillPortion(2)),
                        default_text(
                            util::format_long_duration(&metadata.length),
                            theme,
                            true,
                            true
                        )
                        .width(Length::FillPortion(1)),
                        space().width(Length::FillPortion(2)),
                    ]
                    .width(Length::Fill)
                    .spacing(4.0),
                    theme,
                )
                .on_press(Message::PlaylistSelect(metadata.clone())),
                2.0,
                theme.stylesheet().secondary_rule(2.0),
                10.0,
                i,
                playlist_count,
            )
            .into()
        };

        // construct scrollable
        let scroll = virtualized_vertical_scrollable(
            &app.general_cache.all_playlist_metadata,
            20.0,
            app.home_playlists_widget_data.scrolling_offset,
            theme,
            playlist_data_closure,
            theme.stylesheet().default_scrollable(),
            theme.stylesheet().sub_content(),
            |v| Message::HomePlaylistsScrolled {
                scrollable_viewport: v,
            },
            0.0,
            |s| s,
        );

        container(column![
            container(playlists_info).padding(Padding::new(4.0)),
            scroll
        ])
    } else {
        container(
            secondary_text("No playlists downloaded :(", theme, true, true)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center),
        )
    };

    let playlists_title = title_text("Playlists", theme, true, true);
    let playlist_loading = row![playlist_url, load_playlist];

    let track_count = app.general_cache.all_tracks.len();
    let tracks = if track_count > 0 {
        let tracks_info = row![
            default_text("Name", theme, true, true).width(Length::FillPortion(6)),
            default_text("Artists", theme, true, true).width(Length::FillPortion(3)),
            space().width(Length::FillPortion(1)),
        ]
        .padding(Padding::ZERO.vertical(4.0));

        let track_data_closure = move |i: usize, t: &Track, theme: &Theme| {
            in_between_rule(
                invisible_button_padded(
                    row![
                        default_text(t.title.clone(), theme, true, true)
                            .width(Length::FillPortion(6)),
                        default_text(t.artist.clone().artist(), theme, true, true)
                            .width(Length::FillPortion(3)),
                        space().width(Length::FillPortion(1)),
                    ],
                    theme,
                ),
                2.0,
                theme.stylesheet().secondary_rule(2.0),
                25.0,
                i,
                track_count,
            )
            .into()
        };

        // construct scrollable
        let scroll = virtualized_vertical_scrollable(
            &app.general_cache.all_tracks,
            20.0,
            app.home_tracks_widget_data.scrolling_offset,
            theme,
            track_data_closure,
            theme.stylesheet().default_scrollable(),
            theme.stylesheet().sub_content(),
            |v| Message::HomeTracksScrolled {
                scrollable_viewport: v,
            },
            0.0,
            |s| s,
        );

        container(column![tracks_info, scroll])
            .width(Length::Fill)
            .into()
    } else {
        main_content_container(
            secondary_text("No tracks found :(", theme, true, true)
                .align_x(Alignment::Center)
                .width(Length::Fill)
                .height(Length::Fill),
            theme,
        )
    };
    let tracks_title = title_text(
        format!("Tracks ({})", app.general_cache.all_tracks.len()),
        theme,
        true,
        true,
    );

    let album_count = app.general_cache.all_albums.len();
    let albums = if album_count > 0 {
        let albums_info = row![
            default_text("Name", theme, true, true).width(Length::FillPortion(5)),
            default_text("Artists", theme, true, true).width(Length::FillPortion(3)),
            space().width(Length::FillPortion(2)),
        ];

        let album_data_closure = move |i: usize, a: &Album, theme: &Theme| {
            let album_artist_string = a.artists.join(", ");
            in_between_rule(
                invisible_button(
                    row![
                        default_text(a.name.clone(), theme, true, true)
                            .width(Length::FillPortion(5)),
                        default_text(album_artist_string, theme, true, true)
                            .width(Length::FillPortion(3)),
                        space().width(Length::FillPortion(2)),
                    ],
                    theme,
                ),
                2.0,
                theme.stylesheet().secondary_rule(2.0),
                25.0,
                i,
                album_count,
            )
            .into()
        };

        // construct scrollable
        let scroll = virtualized_vertical_scrollable(
            &app.general_cache.all_albums,
            20.0,
            app.home_albums_widget_data.scrolling_offset,
            theme,
            album_data_closure,
            theme.stylesheet().default_scrollable(),
            theme.stylesheet().sub_content(),
            |v| Message::HomeAlbumsScrolled {
                scrollable_viewport: v,
            },
            0.0,
            |s| s,
        );

        container(column![albums_info, scroll]).into()
    } else {
        main_content_container(
            secondary_text("No albums?? what the heck man", theme, true, true)
                .align_x(Alignment::Center)
                .width(Length::Fill)
                .height(Length::Fill),
            theme,
        )
    };
    let albums_title = title_text(
        format!("Albums ({})", app.general_cache.all_albums.len()),
        theme,
        true,
        true,
    );

    let playlist_content =
        home_menu_widget_container(column![playlists_title, playlist_loading, playlists], theme)
            .width(Length::Fill);

    let tracks_content =
        home_menu_widget_container(column![tracks_title, tracks], theme).width(Length::Fill);
    let albums_content =
        home_menu_widget_container(column![albums_title, albums], theme).width(Length::Fill);

    menu_content_container(
        column![
            upper_menu_content,
            row![
                playlist_content.width(Length::FillPortion(1)),
                column![
                    tracks_content.height(Length::FillPortion(1)),
                    albums_content.height(Length::FillPortion(1))
                ]
                .spacing(WIDGET_SPACING)
                .width(Length::FillPortion(1))
            ]
            .padding(Padding::new(WIDGET_SPACING))
            .spacing(WIDGET_SPACING)
        ],
        theme,
    )
}

pub fn player(app: &App) -> Container<'_, Message> {
    // DATA COLLECTION //

    let theme = &app.theme;
    let current_render_data = {
        if let Page::Player { playlist_id } = &app.management.current_page
            && let Some(d) = app.playlist_render_data.get(&playlist_id)
        {
            d
        } else {
            return container("Somehow there's no playlist to render here??");
        }
    };
    let current_playlist_id = current_render_data.owned_playlist.metadata.id().clone();
    let track_count_digits =
        util::get_u64_digit_count(current_render_data.owned_playlist.track_count() as u64) as usize;

    // PAGE BUILDING //

    // LEFT MENU BAR

    let home_button = invisible_button(
        row![
            icon(
                icons::LEFT_ARROW,
                theme.stylesheet().left_menu_bold_text(true, true)
            ),
            left_menu_bold_text("Home", theme, true, true)
        ]
        .align_y(Alignment::Center),
        theme,
    )
    .on_press(Message::Action(Action::Home));
    let playlists_col = column![left_menu_bold_text("Playlists", theme, false, true)]
        // add up to 3 playlists
        .extend(
            util::generate_playlist_list(app)
                .map(|metadata| left_menu_sub_text(&metadata.title, theme, true, true).into()),
        );
    let albums_col = column![left_menu_bold_text("Albums", theme, false, true)].extend(
        app.general_cache
            .all_albums
            .iter()
            .take(super::ALBUM_DISPLAY_SIZE)
            .map(|album| left_menu_sub_text(&album.name, theme, true, true).into()),
    );
    // assemble left menu
    let left_menu = Column::new().width(Length::Fixed(200.0)).spacing(10.0);
    let left_menu = menu_content_container(
        left_menu
            .push(home_button)
            .push(playlists_col)
            .push(albums_col)
            .push(left_menu_bold_text("About", theme, false, true)),
        theme,
    )
    .height(Length::Fill)
    .padding(Padding::new(5.0));

    // PLAYLIST INFO (and search bar)

    let title = container(
        title_text(
            &current_render_data.owned_playlist.metadata.title,
            theme,
            false,
            true,
        )
        .height(Length::Fill),
    );
    let search_bar = default_text_input("Search..", &current_render_data.track_search_text, theme)
        .on_input({
            let pid = current_playlist_id.clone();
            move |t| Message::TrackSearchTextEdit {
                playlist_id: pid.clone(),
                search_text: t,
            }
        })
        .on_paste({
            let pid = current_playlist_id.clone();
            move |t| Message::TrackSearchTextEdit {
                playlist_id: pid.clone(),
                search_text: t,
            }
        });

    let playlist_info_search = row![
        title.width(Length::Fill),
        search_bar.width(Length::Fixed(300.0))
    ];

    // create header for tracks
    const TRACK_CAGEGORY_SPACING: f32 = 2.0;
    let tracks_header = row![
        default_text("#", theme, false, true).width(Length::FillPortion(1)),
        default_text("Name", theme, false, true).width(Length::FillPortion(8)),
        default_text("Artist", theme, false, true).width(Length::FillPortion(5)),
        default_text("Length", theme, false, true).width(Length::FillPortion(2)),
        default_text("Status", theme, false, true).width(Length::FillPortion(2)),
        default_text("Actions", theme, false, true).width(Length::FillPortion(3)),
    ]
    .spacing(TRACK_CAGEGORY_SPACING)
    .padding(Padding::ZERO.vertical(5.0));

    let upper_info = menu_content_container(column![playlist_info_search, tracks_header], theme)
        .height(Length::Fixed(60.0));

    // TRACK DATA

    // data retrieval
    let playlist_id_clone = current_playlist_id.clone();

    let filtered_tracklist: Vec<(usize, &Track)> = current_render_data
        .current_tracklist
        .iter()
        .enumerate()
        .filter(|(_, track)| {
            util::track_contains_search_term(track, &current_render_data.track_search_text)
        })
        .collect();

    let track_closure =
        move |_relative_index: usize, (index, track): &(usize, &Track), theme: &Theme| {
            // create the button
            let artist_name = match &track.artist {
                Artist::Community(name) => name.clone(),
                Artist::Official(names) => names.join(", "),
            };
            let track_length = util::format_duration(&track.length);
            let track_downloaded = app.general_cache.downloaded_tracks.contains(&track.id());
            let track_downloading = app.general_cache.downloading_tracks.contains(&track.id());
            let track_download_state = if track_downloading {
                TrackDownloadState::Downloading
            } else if track_downloaded {
                TrackDownloadState::Downloaded
            } else {
                TrackDownloadState::NotDownloaded
            };
            // create the button
            track_button(
                row![
                    // Track index text
                    default_text(
                        format!("{:0>track_count_digits$}.", index.clone() + 1),
                        theme,
                        true,
                        true
                    )
                    .width(Length::FillPortion(1)),
                    // Track title text
                    default_text(track.title.clone(), theme, true, true)
                        .width(Length::Fill)
                        .width(Length::FillPortion(8)),
                    // Track artist text
                    default_text(artist_name, theme, true, true).width(Length::FillPortion(5)),
                    // Track length text
                    default_text(track_length, theme, true, true).width(Length::FillPortion(2)),
                    // Track status text
                    default_text(
                        match track_download_state {
                            TrackDownloadState::NotDownloaded => "",
                            TrackDownloadState::Downloading => " ⬇️",
                            TrackDownloadState::Downloaded => " ✅",
                        },
                        theme,
                        false,
                        true
                    )
                    .width(Length::FillPortion(2)),
                    // TODO: Track action buttons
                    text("").width(Length::FillPortion(3))
                ]
                .spacing(TRACK_CAGEGORY_SPACING),
                theme,
            )
            .on_press(Message::Action(Action::PlayTrack {
                playlist_id: playlist_id_clone.clone(),
                track_index: *index as u64,
            }))
            .into()
        };

    // let track_closure = move |_relative_index: usize,
    //                           (index, track): &(usize, &Track),
    //                           theme: &Theme| text("hi").into();

    let track_text_ss = theme.stylesheet().default_text(false, true);
    let track_button_ss = theme.stylesheet().track_button();
    let track_entry_height = track_text_ss.text_size + track_button_ss.padding.y();
    let playlist_id_clone = current_playlist_id.clone();
    let tracklist = virtualized_vertical_scrollable(
        filtered_tracklist,
        track_entry_height,
        current_render_data.scroll_offset,
        theme,
        track_closure,
        theme.stylesheet().default_scrollable(),
        theme.stylesheet().main_content(),
        move |v| Message::TracklistScrolled {
            playlist_id: playlist_id_clone.clone(),
            scrollable_viewport: v,
        },
        0.0,
        |s| s,
    );

    // LOWER CONTROLS
    const LOWER_CONTROLS_HEIGHT: f32 = 75.0;
    let default_text_style = theme.stylesheet().default_text(true, true);

    let volume_icon = if app.settings.volume > 0.66 {
        icons::VOLUME_HIGH
    } else if app.settings.volume > 0.33 {
        icons::VOLUME_MEDIUM
    } else if app.settings.volume > 0.0 {
        icons::VOLUME_LOW
    } else {
        icons::VOLUME_MUTE
    };
    let volume_text = icon(volume_icon, default_text_style);
    let volume_bar = default_slider(
        0.0..=100.0,
        app.settings.volume * 100.0,
        |volume| {
            Message::Action(Action::SetVolume {
                volume: volume / 100.0,
            })
        },
        theme,
    )
    .width(Length::Fixed(150.0));

    let volume_interface = row![volume_text, volume_bar].spacing(5.0);

    // buttons

    let download_button = if let DownloadState::Downloding = current_render_data.download_state {
        default_button(icon(icons::STOP_DOWNLOAD, default_text_style), theme).on_press(
            Message::Action(Action::StopPlaylistDownload {
                playlist_id: current_playlist_id.clone(),
            }),
        )
    } else if let DownloadState::StopPending = current_render_data.download_state {
        default_button(icon(icons::PENDING, default_text_style), theme)
    } else {
        default_button(icon(icons::START_DOWNLOAD, default_text_style), theme).on_press(
            Message::Action(Action::DownloadPlaylist {
                playlist_id: current_playlist_id.clone(),
            }),
        )
    };

    let loop_button = default_button(icon(icons::LOOP, default_text_style), theme).on_press(
        Message::Action(Action::LoopTrack {
            playlist_id: current_playlist_id.clone(),
        }),
    );
    let shuffle_button = default_button(icon(icons::SHUFFLE, default_text_style), theme).on_press(
        Message::Action(Action::ShufflePlaylist {
            playlist_id: current_playlist_id.clone(),
        }),
    );
    let organize_button = default_button(icon(icons::ORGANIZE, default_text_style), theme)
        .on_press(Message::Action(Action::OrganizePlaylist {
            playlist_id: current_playlist_id.clone(),
        }));
    let previous_button = default_button(icon(icons::PREVIOUS, default_text_style), theme)
        .on_press(Message::Action(Action::PreviousTrack {
            playlist_id: current_playlist_id.clone(),
        }));
    let next_button = default_button(icon(icons::SKIP, default_text_style), theme).on_press(
        Message::Action(Action::NextTrack {
            playlist_id: current_playlist_id.clone(),
        }),
    );
    let play_button = if let PlayingState::Playing = current_render_data.playing_state {
        default_button(icon(icons::PAUSE, default_text_style), theme).on_press(Message::Action(
            Action::PauseTrack {
                playlist_id: current_playlist_id.clone(),
            },
        ))
    } else if matches!(current_render_data.playing_state, PlayingState::Paused)
        || matches!(current_render_data.playing_state, PlayingState::Seeking)
    {
        default_button(icon(icons::PLAY, default_text_style), theme).on_press(Message::Action(
            Action::ResumeTrack {
                playlist_id: current_playlist_id.clone(),
            },
        ))
    } else {
        default_button(icon(icons::PLAY, default_text_style), theme)
    };

    // progress bar
    let current_time = current_render_data.playing_track_progress.current();
    let total_time = current_render_data.playing_track_progress.total();

    let current_txt = text(format_duration(current_time));
    let total_txt = text(format_duration(total_time));

    let current_playlist_id_clone = current_playlist_id.clone();
    let progress_bar_slider = default_slider(
        0.0..=100.0,
        current_render_data.playing_track_progress.progress() * 100.0,
        move |progress| {
            Message::Action(Action::SeekAudio {
                playlist_id: current_playlist_id_clone.clone(),
                progress: progress / 100.0,
            })
        },
        theme,
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
    let core_and_progress = column![progress_bar, core_buttons].align_x(Alignment::Center);

    let lower_middle = container(core_and_progress).width(Length::FillPortion(1));

    // Left and right share remaining space equally
    // left: album cover, track title, and artist
    let mut left_controls = Row::new().width(Length::FillPortion(1));
    if let Some(track) = &current_render_data.current_track {
        // album cover first
        match &track.album_kind {
            AlbumKind::Album(album) => {
                let album_cover_path = file::util::album_filename_from_id(&album.id());
                if let Ok(path) = album_cover_path {
                    // make image with static size
                    let img = Image::new(path)
                        .width(Length::Fixed(LOWER_CONTROLS_HEIGHT))
                        .height(Length::Fixed(LOWER_CONTROLS_HEIGHT));
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
            default_text(&track.title, theme, true, true),
            secondary_text(track_artist, theme, true, true)
        ])
        .height(Length::Fill)
        .align_y(Alignment::Center);
        left_controls = left_controls.push(track_info)
    } else {
        left_controls = left_controls
            .push(column![
                default_text("Playlist not loaded", theme, true, true),
                secondary_text(
                    "If this is your first time, download something then come back to this page :D",
                    theme,
                    true,
                    true
                ),
            ])
            .height(Length::Fill)
            .align_y(Alignment::Center);
    }

    let right_controls =
        row![space().width(Length::Fill), volume_interface].width(Length::FillPortion(1));
    let controls = row![left_controls, lower_middle, right_controls]
        .align_y(Alignment::Center)
        .padding(Padding::new(5.0));

    let lower_controls = menu_content_container(column![controls], theme)
        .height(Length::Fixed(LOWER_CONTROLS_HEIGHT))
        .align_y(Alignment::Center);

    // put it all together

    let playlist_and_track_data = column![upper_info, tracklist];
    let upper_portion = row![left_menu, playlist_and_track_data];
    let page = container(column![upper_portion, lower_controls]);

    page
}
