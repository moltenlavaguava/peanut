use iced::event::Status;
use iced::keyboard::key;
use iced::{Element, Event, Task, event, keyboard};
use iced::{Subscription, Theme};
use indexmap::IndexMap;
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use url::Url;

use crate::service::audio::enums::LoopPolicy;
use crate::service::audio::structs::AudioProgress;
use crate::service::gui::enums::{Action, DownloadState, PlayingState};
use crate::service::gui::structs::{
    GeneralCache, GuiCommunication, GuiManagement, GuiSettings, HomeAlbumsWidgetData,
    HomePlaylistsWidgetData, HomeTracksWidgetData, IdCounter, PlaylistInitData, PlaylistInitId,
    PlaylistInitIdCounter, PlaylistRenderData, TaskId,
};
use crate::service::gui::util::delay_task;
use crate::service::gui::widgets::modal::ModalMessage;
use crate::service::gui::widgets::modal::new_playlist::NewPlaylistModal;
use crate::service::id::structs::Id;
use crate::service::playlist::PlaylistSender;
use crate::service::playlist::enums::{PlaylistInitStatus, PlaylistMessage};
use crate::service::playlist::structs::Tracklist;
use crate::util::sync::ReceiverHandle;
use builders::{home, player};
use enums::{EventMessage, Message, Page};

mod builders;
pub mod enums;
mod icons;
pub mod structs;
mod styling;
mod util;
mod widgets;

const RECENT_PLAYLIST_SIZE: usize = 3;
const ALBUM_DISPLAY_SIZE: usize = 3;

struct App {
    communication: GuiCommunication,
    management: GuiManagement,
    settings: GuiSettings,
    home_playlists_widget_data: HomePlaylistsWidgetData,
    home_tracks_widget_data: HomeTracksWidgetData,
    home_albums_widget_data: HomeAlbumsWidgetData,
    general_cache: GeneralCache,
    playlist_render_data: IndexMap<Id, PlaylistRenderData>,
    playlist_init_data: IndexMap<PlaylistInitId, PlaylistInitData>,
    theme: Theme,
}

#[derive(Clone)]
struct GuiFlags {
    event_receiver: ReceiverHandle<EventMessage>,
    playlist_sender: PlaylistSender,
}

impl App {
    fn new(flags: GuiFlags) -> (Self, Task<Message>) {
        let playlist_sender_clone = flags.playlist_sender.clone();
        let communication = GuiCommunication {
            playlist_sender: flags.playlist_sender,
            active_tasks: HashMap::new(),
            event_bus: flags.event_receiver,
        };
        let management = GuiManagement {
            id_counter: IdCounter::new(),
            playlist_init_id_counter: PlaylistInitIdCounter::new(),
            current_page: Page::Home,
        };
        let home_playlists_widget_data = HomePlaylistsWidgetData::default();
        let home_tracks_widget_data = HomeTracksWidgetData::default();
        let home_albums_widget_data = HomeAlbumsWidgetData::default();
        let general_cache = GeneralCache {
            all_albums: Vec::new(),
            all_tracks: Vec::new(),
            downloaded_tracks: HashSet::new(),
            downloading_track_data: IndexMap::new(),
            all_playlist_metadata: Vec::new(),
            recent_playlists: VecDeque::with_capacity(RECENT_PLAYLIST_SIZE),
            active_modal: None,
        };
        let settings = GuiSettings { volume: 1.0 };
        let playlist_render_data = IndexMap::new();
        let playlist_init_data = IndexMap::new();
        let theme = Theme::Dark;
        (
            Self {
                communication,
                management,
                home_playlists_widget_data,
                home_tracks_widget_data,
                home_albums_widget_data,
                general_cache,
                settings,
                playlist_render_data,
                playlist_init_data,
                theme,
            },
            Task::perform(
                util::request_downloaded_tracks(playlist_sender_clone),
                |maybe_tracks| {
                    if let Ok(tracks) = maybe_tracks {
                        Message::DownloadedTracklistReceived(tracks)
                    } else {
                        Message::DownloadedTracklistReceived(HashSet::new())
                    }
                },
            ),
        )
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PlaylistTextEdit(txt) => {
                self.home_playlists_widget_data.search_text = txt;
                Task::none()
            }
            Message::PlaylistURLSubmit => {
                println!(
                    "playlist url: {}",
                    self.home_playlists_widget_data.search_text
                );
                // try to create a url
                if let Ok(url) = Url::parse(&self.home_playlists_widget_data.search_text) {
                    Task::perform(
                        submit_playlist_url(
                            self.management.id_counter.next(),
                            self.management.playlist_init_id_counter.next(),
                            url,
                            self.communication.playlist_sender.clone(),
                        ),
                        |msg| msg,
                    )
                } else {
                    Task::none()
                }
            }
            Message::EventRecieved(msg) => {
                match msg {
                    EventMessage::InitialPlaylistsInitalized(playlist_data) => {
                        self.general_cache.all_playlist_metadata = playlist_data;
                        // sort the data
                        util::sort_playlist_metadata(&mut self.general_cache.all_playlist_metadata);
                    }
                    EventMessage::TrackDownloadFinished { id, success } => {
                        // a given track finished downloading.
                        println!("Track download finished");
                        // add downloaded track to list and remove it from the downloading tracks list
                        self.general_cache.downloading_track_data.swap_remove(&id);
                        if success {
                            self.general_cache.downloaded_tracks.insert(id);
                        }
                    }
                    EventMessage::TrackUpdated { track } => {
                        println!("track updated in gui");
                        // A track's data just updated. If its in the current playlist, update it for rendering.
                        if let Page::Player { playlist_id } = &self.management.current_page {
                            // a page is currently loaded; ensure there's actually playlist data for this page
                            if let Some(render_data) =
                                self.playlist_render_data.get_mut(playlist_id)
                            {
                                // a playlist is here; update it
                                if let Some(pos) = render_data
                                    .owned_playlist
                                    .tracks
                                    .0
                                    .iter()
                                    .position(|t| t.id() == track.id())
                                {
                                    // replace the current track with the new one
                                    println!("updating playlist @ gui");
                                    render_data.owned_playlist.tracks.0[pos] = track;
                                    // update tracklist
                                    render_data
                                        .current_tracklist
                                        .replace_tracks(render_data.owned_playlist.tracks.clone());
                                }
                            }
                        }
                    }
                    EventMessage::DownloadedAlbumsReceived(albums) => {
                        // convert hashmap to vec
                        let mut album_vec = albums.into_values().collect();
                        self.general_cache.all_albums.append(&mut album_vec);
                        // sort albums
                        util::sort_albums(&mut self.general_cache.all_albums);
                    }
                    EventMessage::AlbumDataDownloaded { album } => {
                        if !self.general_cache.all_albums.contains(&album) {
                            self.general_cache.all_albums.push(album);
                            util::sort_albums(&mut self.general_cache.all_albums);
                        }
                    }
                    EventMessage::TrackCacheUpdated {
                        tracks_added,
                        tracks_removed,
                    } => {
                        if let Some(added_tracks) = tracks_added {
                            // convert hashmap to vec
                            let t_vec: Vec<_> = added_tracks.into_values().collect();
                            self.general_cache.all_tracks.extend(t_vec);
                            // organize
                            util::sort_track_cache(self);
                        }
                        if let Some(removed_tracks) = tracks_removed {
                            for t in removed_tracks.iter() {
                                if let Some(i) = self
                                    .general_cache
                                    .all_tracks
                                    .iter()
                                    .position(|s| s.id() == t)
                                {
                                    self.general_cache.all_tracks.remove(i);
                                }
                            }
                        }
                    }
                };
                Task::none()
            }
            Message::EventBusClosed => {
                println!("Event bus closed");
                Task::none()
            }
            Message::TaskFinished(id) => {
                self.communication.active_tasks.remove(&id);
                Task::none()
            }
            Message::PlaylistInitTaskStarted(task_id, playlist_init_id, handle) => {
                self.communication.active_tasks.insert(task_id, handle);
                let init_data = PlaylistInitData {
                    current_init_track_count: Some(0),
                    total_track_count: None,
                    name: None,
                    platform_display_id: None,
                };
                self.playlist_init_data.insert(playlist_init_id, init_data);
                Task::none()
            }
            Message::PlaylistInitStatus { status, id } => {
                // pull data for this init
                let init_data = match self.playlist_init_data.get_mut(&id) {
                    Some(d) => d,
                    None => return Task::none(),
                };
                let mut end_task = Task::none();
                if !matches!(status, PlaylistInitStatus::Progress { .. }) {
                    // Playlist init finished somehow; schedule a removal
                    end_task = delay_task(
                        Duration::from_secs(1),
                        Message::RemovePlaylistInitData { init_id: id },
                    )
                }
                // handle every type
                match status {
                    PlaylistInitStatus::Progress { current, total } => {
                        init_data.current_init_track_count = Some(current);
                        init_data.total_track_count = Some(total);
                    }
                    PlaylistInitStatus::Complete(metadata) => {
                        self.general_cache.all_playlist_metadata.push(metadata);
                        // sort the data
                        util::sort_playlist_metadata(&mut self.general_cache.all_playlist_metadata);
                    }
                    PlaylistInitStatus::Fail => {
                        println!("received msg that playlist init failed");
                    }
                    PlaylistInitStatus::Duplicate(metadata) => {
                        println!(
                            "received msg that playlist {} was a duplicate",
                            metadata.title
                        );
                    }
                }
                end_task
            }
            Message::PlaylistSelect(playlist_metadata) => {
                println!("selected metadata: {playlist_metadata:?}");

                // if there's already render data and it isn't unloaded, don't send a request
                if let Some(render_data) = self.playlist_render_data.get(playlist_metadata.id())
                    && !matches!(render_data.playing_state, PlayingState::Unloaded)
                {
                    println!("Loaded playlist already exists; doing nothing");
                    util::handle_playlist_load(self, None, playlist_metadata);
                    return Task::none();
                }

                let playlist_sender_clone = self.communication.playlist_sender.clone();
                Task::perform(
                    util::request_owned_playlist(
                        playlist_metadata.id().clone(),
                        playlist_sender_clone,
                    ),
                    |output| Message::PlaylistSelectAccepted(output.unwrap().unwrap()),
                )
            }
            Message::PlaylistSelectAccepted(owned_playlist) => {
                // change the page + set the current playlist + start 'playing' the playlist
                let task = Task::perform(
                    util::play_playlist(
                        owned_playlist.metadata.id().clone(),
                        self.management.id_counter.next(),
                        self.communication.playlist_sender.clone(),
                        None,
                        self.settings.volume,
                    ),
                    |handle| Message::TaskStarted {
                        handle: handle.unwrap(),
                    },
                );

                let current_tracklist = Tracklist::from_owned_playlist_ref(&owned_playlist);
                let new_metadata = owned_playlist.metadata.clone();
                let render_data = PlaylistRenderData {
                    playlist_id: owned_playlist.metadata.id().clone(),
                    current_track: None,
                    owned_playlist: owned_playlist,
                    playing_track_loop_policy: LoopPolicy::NoLooping,
                    playing_track_progress: AudioProgress::new(Duration::ZERO, Duration::ZERO),
                    current_tracklist,
                    playing_state: PlayingState::None,
                    download_state: DownloadState::Idle,
                    scroll_offset: 0.0,
                    track_search_text: String::new(),
                };

                util::handle_playlist_load(self, Some(render_data), new_metadata);

                task
            }
            Message::DownloadedTracklistReceived(tracks) => {
                // add to the list of downloaded tracks
                self.general_cache.downloaded_tracks.extend(tracks);
                Task::none()
            }
            Message::Action(action) => {
                match action {
                    Action::Home => {
                        // reset everything player-wise
                        self.management.current_page = Page::Home;
                        // clear any playlist data that is unloaded to save memory
                        self.playlist_render_data.retain(|_, rdata| {
                            !matches!(rdata.playing_state, PlayingState::Unloaded)
                        });
                        Task::none()
                    }
                    Action::DownloadPlaylist { playlist_id } => {
                        // send request to playlist service to download
                        let playlist_sender_clone = self.communication.playlist_sender.clone();
                        let next_id = self.management.id_counter.next();
                        // get the current tracklist for this playlist
                        let tracklist = {
                            match &self.management.current_page {
                                Page::Home => {
                                    println!("Download attempted when in home page");
                                    return Task::none();
                                }
                                Page::Player { playlist_id } => {
                                    if let Some(rdata) = self.playlist_render_data.get(playlist_id)
                                    {
                                        rdata.current_tracklist.clone()
                                    } else {
                                        println!("Render data not found while in home page??");
                                        return Task::none();
                                    }
                                }
                            }
                        };
                        Task::perform(
                            util::download_playlist(
                                playlist_id,
                                playlist_sender_clone.clone(),
                                next_id,
                                tracklist,
                            ),
                            |maybe_msg| {
                                if let Ok(msg) = maybe_msg {
                                    msg
                                } else {
                                    println!(
                                        "Message from download playlist was an error; probably channel dropped"
                                    );
                                    Message::None
                                }
                            },
                        )
                    }
                    Action::StopPlaylistDownload { playlist_id } => {
                        let playlist_sender_clone = self.communication.playlist_sender.clone();
                        Task::perform(
                            util::stop_playlist_download(
                                playlist_id.clone(),
                                playlist_sender_clone,
                            ),
                            |result| {
                                if let Err(e) = result {
                                    println!(
                                        "An error occured while stopping the playlist download: {}",
                                        e
                                    )
                                }
                                Message::PlaylistDownloadCancelStarted { id: playlist_id }
                            },
                        )
                    }
                    Action::ShufflePlaylist { playlist_id } => {
                        println!("shuffle playlist on gui end");
                        let playlist_sender_clone = self.communication.playlist_sender.clone();
                        Task::perform(
                            util::shuffle_playlist(
                                playlist_id.clone(),
                                playlist_sender_clone,
                                None,
                            ),
                            |result| {
                                if let Ok(tracklist) = result {
                                    Message::PlaylistOrderUpdated {
                                        id: playlist_id,
                                        tracklist,
                                    }
                                } else {
                                    Message::None
                                }
                            },
                        )
                    }
                    Action::OrganizePlaylist { playlist_id } => {
                        println!("organize playlist on gui end");
                        let playlist_sender_clone = self.communication.playlist_sender.clone();
                        Task::perform(
                            util::organize_playlist(
                                playlist_id.clone(),
                                playlist_sender_clone,
                                None,
                            ),
                            |result| {
                                if let Ok(tracklist) = result {
                                    Message::PlaylistOrderUpdated {
                                        id: playlist_id,
                                        tracklist,
                                    }
                                } else {
                                    Message::None
                                }
                            },
                        )
                    }
                    Action::ResumeTrack { playlist_id } => {
                        let playlist_sender_clone = self.communication.playlist_sender.clone();
                        Task::perform(
                            util::resume_current_playlist_track(
                                playlist_id.clone(),
                                playlist_sender_clone,
                                None,
                            ),
                            |_result| Message::TrackAudioResumeResult { playlist_id },
                        )
                    }
                    Action::PauseTrack { playlist_id } => {
                        let playlist_sender_clone = self.communication.playlist_sender.clone();
                        Task::perform(
                            util::pause_current_playlist_track(
                                playlist_id.clone(),
                                playlist_sender_clone,
                            ),
                            |_result| Message::TrackAudioPauseResult { playlist_id },
                        )
                    }
                    Action::NextTrack { playlist_id } => {
                        let playlist_sender_clone = self.communication.playlist_sender.clone();
                        Task::perform(
                            util::skip_current_playlist_track(
                                playlist_id.clone(),
                                playlist_sender_clone,
                            ),
                            |_result| Message::TrackAudioResumeResult { playlist_id },
                        )
                    }
                    Action::PreviousTrack { playlist_id } => {
                        let playlist_sender_clone = self.communication.playlist_sender.clone();
                        Task::perform(
                            util::previous_current_playlist_track(
                                playlist_id.clone(),
                                playlist_sender_clone,
                            ),
                            |_result| Message::TrackAudioResumeResult { playlist_id },
                        )
                    }
                    Action::PlayTrack {
                        playlist_id,
                        track_index,
                    } => {
                        let playlist_sender_clone = self.communication.playlist_sender.clone();
                        Task::perform(
                            util::play_track_in_playlist(
                                playlist_id.clone(),
                                playlist_sender_clone,
                                track_index,
                            ),
                            |_result| Message::PlayTrackResult {
                                playlist_id: Some(playlist_id),
                            },
                        )
                    }
                    Action::SeekAudio {
                        playlist_id,
                        progress,
                    } => {
                        // set the internal progress
                        if let Some(render_data) = self.playlist_render_data.get_mut(&playlist_id) {
                            render_data.playing_track_progress.update_progress(progress);
                            // if the playlist is currently playing, pause it
                            let t = if matches!(render_data.playing_state, PlayingState::Playing) {
                                let playlist_sender_clone =
                                    self.communication.playlist_sender.clone();
                                return Task::perform(
                                    util::pause_current_playlist_track(
                                        playlist_id.clone(),
                                        playlist_sender_clone,
                                    ),
                                    |_result| Message::TrackAudioPauseResult { playlist_id },
                                );
                            } else {
                                Task::none()
                            };
                            // update playing state
                            render_data.playing_state = PlayingState::Seeking;
                            return t;
                        }
                        Task::none()
                    }
                    Action::StopSeekingAudio { playlist_id } => {
                        // if the playlist is currently paused, then resume it
                        if let Some(render_data) = self.playlist_render_data.get_mut(&playlist_id) {
                            if !matches!(render_data.playing_state, PlayingState::Playing) {
                                let playlist_sender_clone =
                                    self.communication.playlist_sender.clone();
                                return Task::perform(
                                    util::resume_current_playlist_track(
                                        playlist_id.clone(),
                                        playlist_sender_clone,
                                        Some(render_data.playing_track_progress.progress()),
                                    ),
                                    |_result| Message::TrackAudioResumeResult { playlist_id },
                                );
                            }
                        }
                        Task::none()
                    }
                    Action::LoopTrack { playlist_id } => {
                        // loop button pressed; advance policy to next one
                        if let Some(render_data) = self.playlist_render_data.get_mut(&playlist_id) {
                            let next_policy = render_data.playing_track_loop_policy.next();

                            // send request to playlist service
                            let playlist_sender_clone = self.communication.playlist_sender.clone();
                            let playlist_id_clone = playlist_id.clone();
                            let task = Task::perform(
                                util::set_playlist_loop_policy(
                                    playlist_id.clone(),
                                    next_policy.clone(),
                                    playlist_sender_clone,
                                ),
                                |_r| Message::SetPlaylistLoopPolicyResult {
                                    playlist_id: playlist_id_clone,
                                },
                            );

                            render_data.playing_track_loop_policy = next_policy;
                            task
                        } else {
                            Task::none()
                        }
                    }
                    Action::SetVolume { volume } => {
                        // if the volume here is different, send a req
                        if self.settings.volume != volume {
                            let playlist_sender_clone = self.communication.playlist_sender.clone();
                            self.settings.volume = volume;
                            return Task::perform(
                                util::update_volume_in_playlist_service(
                                    volume,
                                    playlist_sender_clone,
                                ),
                                |_r| Message::SetGlobalVolumeResult,
                            );
                        }
                        Task::none()
                    }
                }
            }
            Message::DownloadPlaylistStarted {
                id,
                receiver_handle,
            } => {
                // start the (listening) task
                self.communication
                    .active_tasks
                    .insert(receiver_handle.id(), receiver_handle);
                // mark this playlist as downloading
                if let Some(render_data) = self.playlist_render_data.get_mut(&id) {
                    render_data.download_state = DownloadState::Downloding;
                }
                Task::none()
            }
            Message::DownloadPlaylistEnded { id } => {
                if let Some(render_data) = self.playlist_render_data.get_mut(&id) {
                    render_data.download_state = DownloadState::Idle;
                }
                println!("Download ended");
                Task::none()
            }
            Message::PlaylistDownloadCancelStarted { id } => {
                if let Some(render_data) = self.playlist_render_data.get_mut(&id) {
                    render_data.download_state = DownloadState::StopPending;
                }
                println!("Cancel started");
                Task::none()
            }
            Message::TrackDownloadStarted { id, data } => {
                // a given track started downloading.
                println!("track download started");
                self.general_cache.downloading_track_data.insert(id, data);
                Task::none()
            }
            Message::TrackDownloadStatus { id, data } => {
                // A given track's download status updated.
                self.general_cache.downloading_track_data.insert(id, data);
                Task::none()
            }
            Message::PlaylistOrderUpdated { id, tracklist } => {
                println!("Playlist order updated");
                if let Some(render_data) = self.playlist_render_data.get_mut(&id) {
                    render_data.current_tracklist = tracklist;
                    // mark the playlist as playing because thats what happens
                    render_data.playing_state = PlayingState::Paused;
                }
                Task::none()
            }
            Message::TrackAudioProgress {
                id: _,
                progress,
                maybe_playlist_id,
            } => {
                if let Some(playlist_id) = maybe_playlist_id
                    && let Some(render_data) = self.playlist_render_data.get_mut(&playlist_id)
                {
                    // if the user is currently seeking (includes single clicks), ignore this
                    if matches!(render_data.playing_state, PlayingState::Seeking)
                        || matches!(render_data.playing_state, PlayingState::Paused)
                    {
                        return Task::none();
                    }
                    render_data.playing_track_progress = progress;
                }
                Task::none()
            }
            Message::TrackAudioStart {
                id,
                maybe_playlist_id,
                start_paused,
            } => {
                println!("track audio start");
                if let Some(pid) = maybe_playlist_id {
                    if let Some(render_data) = self.playlist_render_data.get_mut(&pid) {
                        // update the playing status
                        if start_paused {
                            render_data.playing_state = PlayingState::Paused;
                        } else {
                            render_data.playing_state = PlayingState::Playing;
                        }
                        if let Some(track) = render_data
                            .owned_playlist
                            .tracks
                            .0
                            .iter()
                            .find(|t| *t.id() == id)
                        {
                            render_data.current_track = Some(track.clone());
                        }
                    }
                }
                Task::none()
            }
            Message::TrackAudioEnd {
                id: _,
                maybe_playlist_id,
            } => {
                println!("track audio end");
                // remove the loop policy from the playlist this was in if it exists
                if let Some(pid) = &maybe_playlist_id {
                    if let Some(render_data) = self.playlist_render_data.get_mut(pid) {
                        render_data.playing_state = PlayingState::None;
                        render_data.playing_track_loop_policy = LoopPolicy::NoLooping;
                        render_data.current_track = None;
                    }
                }
                Task::none()
            }
            Message::TaskStarted { handle } => {
                self.communication.active_tasks.insert(handle.id(), handle);
                Task::none()
            }
            Message::TrackAudioPauseResult { playlist_id } => {
                if let Some(render_data) = self.playlist_render_data.get_mut(&playlist_id) {
                    render_data.playing_state = PlayingState::Paused;
                }
                Task::none()
            }
            Message::TrackAudioPreviousResult { playlist_id: _ } => Task::none(),
            Message::TrackAudioSkipResult { playlist_id: _ } => Task::none(),
            Message::TrackAudioResumeResult { playlist_id } => {
                // when playing audio seeking cannot happen
                if let Some(render_data) = self.playlist_render_data.get_mut(&playlist_id) {
                    render_data.playing_state = PlayingState::Playing;
                }
                Task::none()
            }
            Message::PlayPlaylistEnded { playlist_id } => {
                if let Some(render_data) = self.playlist_render_data.get_mut(&playlist_id) {
                    render_data.playing_state = PlayingState::Unloaded;
                }
                Task::none()
            }
            Message::TrackLooped {
                maybe_playlist_id,
                track_id: _,
            } => {
                if let Some(pid) = maybe_playlist_id
                    && let Some(render_data) = self.playlist_render_data.get_mut(&pid)
                {
                    render_data.playing_track_loop_policy =
                        render_data.playing_track_loop_policy.looped();
                }
                Task::none()
            }
            Message::PlayTrackResult { playlist_id } => {
                if let Some(pid) = playlist_id
                    && let Some(render_data) = self.playlist_render_data.get_mut(&pid)
                {
                    render_data.playing_state = PlayingState::Playing;
                }
                Task::none()
            }
            Message::TrackSearchTextEdit {
                playlist_id,
                search_text,
            } => {
                if let Some(render_data) = self.playlist_render_data.get_mut(&playlist_id) {
                    render_data.track_search_text = search_text
                }
                Task::none()
            }
            // For player tracklist only
            Message::TracklistScrolled {
                playlist_id,
                scrollable_viewport,
            } => {
                if let Some(render_data) = self.playlist_render_data.get_mut(&playlist_id) {
                    render_data.scroll_offset = scrollable_viewport.absolute_offset().y;
                }
                Task::none()
            }
            Message::ThemeUpdated { theme } => {
                self.theme = theme;
                Task::none()
            }
            Message::HomeAlbumsScrolled {
                scrollable_viewport,
            } => {
                self.home_albums_widget_data.scrolling_offset =
                    scrollable_viewport.absolute_offset().y;
                Task::none()
            }
            Message::HomeTracksScrolled {
                scrollable_viewport,
            } => {
                self.home_tracks_widget_data.scrolling_offset =
                    scrollable_viewport.absolute_offset().y;
                Task::none()
            }
            Message::HomePlaylistsScrolled {
                scrollable_viewport,
            } => {
                self.home_playlists_widget_data.scrolling_offset =
                    scrollable_viewport.absolute_offset().y;
                Task::none()
            }
            Message::SetGlobalVolumeResult => Task::none(),
            Message::SetPlaylistLoopPolicyResult { playlist_id: _ } => Task::none(),
            Message::RemovePlaylistInitData { init_id } => {
                self.playlist_init_data.swap_remove(&init_id);
                Task::none()
            }
            Message::StopPlaylist { playlist_id } => {
                let playlist_sender = self.communication.playlist_sender.clone();
                Task::perform(
                    util::stop_playlist(playlist_id.clone(), playlist_sender),
                    |r| {
                        if let Err(e) = r {
                            println!("An error occured while stopping the playlist: {e}")
                        }
                        Message::ManualPlaylistEnded { playlist_id }
                    },
                )
            }
            Message::ManualPlaylistEnded { playlist_id } => {
                // this playlist was stopped by the user. remove any relevant data
                if let Page::Home = self.management.current_page {
                    self.playlist_render_data.swap_remove(&playlist_id);
                } else {
                    if let Some(d) = self.playlist_render_data.get_mut(&playlist_id) {
                        d.playing_state = PlayingState::Unloaded
                    }
                }
                Task::none()
            }
            Message::ModalMessage(mmsg) => {
                if let Some(modal) = &mut self.general_cache.active_modal {
                    // Special: if the message is close modal, then handle it
                    match mmsg {
                        ModalMessage::HideModal => {
                            util::hide_modal(self);
                            Task::none()
                        }
                        _ => modal.update(mmsg).map(Message::ModalMessage),
                    }
                } else {
                    Task::none()
                }
            }
            Message::NewPlaylist => {
                // set the current modal to the new playlist modal,
                // regardless of what was previously there
                self.general_cache.active_modal = Some(NewPlaylistModal::new().into());
                Task::none()
            }
            Message::HideModal => {
                // Note: this clears all the modal data
                util::hide_modal(self);
                Task::none()
            }
            Message::SystemEvent(e) => {
                // println!("got event: {e:?}");
                match e {
                    Event::Keyboard(keyboard::Event::KeyPressed {
                        key: keyboard::Key::Named(key::Named::Escape),
                        ..
                    }) => {
                        // Hide modal
                        util::hide_modal(self);
                        Task::none()
                    }
                    _ => Task::none(),
                }
            }
            Message::None => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        match self.management.current_page {
            Page::Home => home(&self),
            Page::Player { playlist_id: _ } => player(&self),
        }
    }
    fn subscription(&self) -> Subscription<Message> {
        let bus = self.communication.event_bus.watch(
            |_id, msg| Message::EventRecieved(msg),
            |_id| Message::EventBusClosed,
        );
        let tasks = Subscription::batch(
            self.communication
                .active_tasks
                .values()
                .map(|handle| handle.watch(|_id, msg| msg, |id| Message::TaskFinished(id))),
        );
        let event = event::listen_with(|event, status, _id| {
            if let Status::Ignored = status {
                Some(Message::SystemEvent(event))
            } else {
                None
            }
        });
        Subscription::batch(vec![bus, tasks, event])
    }
    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

/// Handles GUI processing.
pub struct GuiService {
    // shutdown_token: Option<CancellationToken>,
}

impl GuiService {
    /// Creates the GuiService.
    pub fn new() -> Self {
        GuiService {
            // shutdown_token: Some(shutdown_token),
        }
    }
    pub fn start_loop(
        &self,
        playlist_sender: PlaylistSender,
        event_bus_rx: mpsc::Receiver<EventMessage>,
    ) -> iced::Result {
        let mut id_counter = IdCounter::new();
        let event_recv_id = id_counter.next();

        let flags = GuiFlags {
            playlist_sender,
            event_receiver: ReceiverHandle::new(event_recv_id, event_bus_rx),
        };

        let icon_font_data = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fonts/peanuticons.ttf"
        ));

        let application =
            iced::application(move || App::new(flags.clone()), App::update, App::view)
                .subscription(App::subscription)
                .theme(App::theme)
                .title("peanut")
                .font(icon_font_data)
                .exit_on_close_request(true);

        application.run()
    }
}

// helper methods
async fn submit_playlist_url(
    task_id: TaskId,
    playlist_init_id: PlaylistInitId,
    url: Url,
    sender: PlaylistSender,
) -> Message {
    println!("submitting playlist url..");
    // create oneshot channel to get progress update from
    let (tx, rx) = oneshot::channel();

    let _ = sender
        .send(PlaylistMessage::InitializePlaylist {
            url,
            playlist_init_id,
            reply_stream: tx,
        })
        .await;

    match rx.await {
        Ok(raw_recv) => {
            println!("Sending playlist init task started msg");
            let handle = ReceiverHandle::new(task_id, raw_recv);
            Message::PlaylistInitTaskStarted(task_id, playlist_init_id, handle)
        }
        Err(_) => {
            println!("something went wrong when submitting playlist url?");
            Message::None
        }
    }
}
