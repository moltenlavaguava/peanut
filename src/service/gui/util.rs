// page factory functions

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use iced::Length;
use iced::widget::{
    Column, Space, button, column, container, lazy, row, scrollable, slider, text, text_input,
};
use tokio::sync::{mpsc, oneshot};

use crate::service::audio::enums::LoopPolicy;
use crate::service::gui::enums::Action;
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
    let current_playlist_id = app.current_ptracklist.as_ref().unwrap().metadata.id.clone();
    let current_playlist_playing = app.paused_playlists.contains(&current_playlist_id);

    let title = text(&current_ptracklist.metadata.title);
    let home_button = button("home").on_press(Message::Action(Action::Home));
    let header = row![home_button, title];

    let tracklist = current_ptracklist
        .list
        .iter()
        .enumerate()
        .map(|(index, track)| {
            // create a metadata object for each track to know when important information changes between renders
            let track_downloaded = app.downloaded_tracks.contains(&track.id());
            let track_downloading = app.downloading_tracks.contains(&track.id());
            let pid = current_playlist_id.clone();
            let track_metadata = TrackMetadata {
                downloaded: track_downloaded,
                downloading: track_downloading,
                title: Arc::from(track.title.as_str()),
            };
            // create a lazy button
            lazy(track_metadata, move |metadata| {
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
                .on_press(Message::Action(Action::PlayTrack {
                    playlist_id: pid.clone(),
                    track_index: index as u64,
                }))
            })
            .into()
        });
    let album_next = row![scrollable(column(tracklist))].height(Length::Fill);

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

    let controls = row![
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

    let volume_text = text("volume?");
    let volume_bar = slider(0.0..=100.0, app.volume * 100.0, |volume| {
        Message::Action(Action::SetVolume {
            volume: volume / 100.0,
        })
    });

    let volume_row = row![volume_text, volume_bar];

    column![header, album_next, progress_bar, volume_row, controls].height(Length::Fill)
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

pub async fn play_playlist(
    playlist_id: Id,
    task_id: u64,
    playlist_sender: PlaylistSender,
    tracklist: Option<TrackList>,
) -> anyhow::Result<ReceiverHandle<Message>> {
    // create a receiver handle for progress updates
    let (tx, rx) = mpsc::channel(100);
    let handle = ReceiverHandle::new(task_id, rx);

    playlist_sender
        .send(PlaylistMessage::PlayPlaylist {
            id: playlist_id,
            tracklist,
            data_sender: tx,
        })
        .await?;
    Ok(handle)
}

pub async fn pause_current_playlist_track(
    playlist_id: Id,
    playlist_sender: PlaylistSender,
) -> anyhow::Result<()> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::PauseCurrentTrack {
            playlist_id,
            result_sender: tx,
        })
        .await
        .unwrap();
    let _ = rx.await?;
    Ok(())
}

pub async fn resume_current_playlist_track(
    playlist_id: Id,
    playlist_sender: PlaylistSender,
    seek_location: Option<f32>,
) -> anyhow::Result<()> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::ResumeCurrentTrack {
            playlist_id,
            result_sender: tx,
            seek_location,
        })
        .await
        .unwrap();
    let _ = rx.await?;
    Ok(())
}

pub async fn skip_current_playlist_track(
    playlist_id: Id,
    playlist_sender: PlaylistSender,
) -> anyhow::Result<()> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::SkipCurrentTrack {
            playlist_id,
            result_sender: tx,
        })
        .await
        .unwrap();
    let _ = rx.await?;
    Ok(())
}

pub async fn previous_current_playlist_track(
    playlist_id: Id,
    playlist_sender: PlaylistSender,
) -> anyhow::Result<()> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::PreviousCurrentTrack {
            playlist_id,
            result_sender: tx,
        })
        .await
        .unwrap();
    let _ = rx.await?;
    Ok(())
}

pub async fn play_track_in_playlist(
    playlist_id: Id,
    playlist_sender: PlaylistSender,
    track_index: u64,
) -> anyhow::Result<()> {
    let (tx, rx) = oneshot::channel();
    let _ = playlist_sender
        .send(PlaylistMessage::SelectPlaylistIndex {
            playlist_id,
            track_index,
            result_sender: tx,
        })
        .await;

    let _ = rx.await?;
    Ok(())
}

pub async fn set_playlist_loop_policy(
    playlist_id: Id,
    policy: LoopPolicy,
    playlist_sender: PlaylistSender,
) -> anyhow::Result<()> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::SetPlaylistLoopPolicy {
            playlist_id,
            policy,
            result_sender: tx,
        })
        .await
        .unwrap();
    let _ = rx.await?;
    Ok(())
}

pub async fn update_volume_in_playlist_service(
    volume: f64,
    playlist_sender: PlaylistSender,
) -> anyhow::Result<()> {
    let (tx, rx) = oneshot::channel();
    playlist_sender
        .send(PlaylistMessage::UpdateGlobalVolume {
            volume,
            result_sender: tx,
        })
        .await
        .unwrap();
    let _ = rx.await?;
    Ok(())
}

pub fn format_duration(duration: &Duration) -> String {
    let total_seconds = duration.as_secs();
    let mins = total_seconds / 60;
    let secs = total_seconds - mins * 60;
    format!("{}:{:02}", mins, secs)
}
