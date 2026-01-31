use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Duration;

use iced::{Subscription, Theme};
use iced::{Task, widget::Column};
use tokio::sync::{mpsc, oneshot};
use tokio_util::sync::CancellationToken;
use url::Url;

use crate::service::audio::enums::LoopPolicy;
use crate::service::audio::structs::AudioProgress;
use crate::service::gui::enums::Action;
use crate::service::gui::structs::IdCounter;
use crate::service::id::structs::Id;
use crate::service::playlist::PlaylistSender;
use crate::service::playlist::enums::{PlaylistInitStatus, PlaylistMessage};
use crate::service::playlist::structs::{Album, OwnedPlaylist, PlaylistMetadata, Track, TrackList};
use crate::util::sync::ReceiverHandle;
use builders::{home, player};
use enums::{EventMessage, Message, Page};

mod builders;
pub mod enums;
mod structs;
mod styling;
mod util;
mod widgetbuilder;
mod widgets;

const RECENT_PLAYLIST_SIZE: usize = 3;
const ALBUM_DISPLAY_SIZE: usize = 3;

struct App {
    // Communication
    _shutdown_token: CancellationToken,
    playlist_sender: PlaylistSender,
    tasks: HashMap<u64, ReceiverHandle<Message>>,
    event_bus: ReceiverHandle<EventMessage>,

    // Internal state
    playlist_url: String,
    id_counter: IdCounter,
    current_track_index: u32,
    total_tracks: u32,
    page: Page,
    track_search_text: String,
    loaded_playlist_metadata: Vec<PlaylistMetadata>,
    // caches
    downloaded_tracks: HashSet<Id>,
    downloaded_albums: Vec<Album>,
    downloading_tracks: HashSet<Id>,
    recent_playlists: VecDeque<PlaylistMetadata>,
    playlist_scrolloffsets: HashMap<Id, f32>,

    current_owned_playlist: Option<OwnedPlaylist>,
    current_playlist_tracklist: Option<TrackList>,
    download_stopping_playlists: HashSet<Id>,
    downloading_playlists: HashSet<Id>,

    playing_playlists: HashSet<Id>,
    paused_playlists: HashSet<Id>,
    playlist_loop_policies: HashMap<Id, LoopPolicy>,
    playlist_playling_tracks: HashMap<Id, Track>,

    track_progress: AudioProgress,
    track_seeking: bool,
    volume: f64,
    theme: Theme,
}

#[derive(Clone)]
struct GuiFlags {
    shutdown_token: CancellationToken,
    event_receiver: ReceiverHandle<EventMessage>,
    playlist_sender: PlaylistSender,
    id_counter: IdCounter,
}

impl App {
    fn new(flags: GuiFlags) -> (Self, Task<Message>) {
        let playlist_sender_clone = flags.playlist_sender.clone();
        (
            Self {
                _shutdown_token: flags.shutdown_token,
                playlist_sender: flags.playlist_sender,
                tasks: HashMap::new(),
                event_bus: flags.event_receiver,
                playlist_url: String::new(),
                id_counter: flags.id_counter,
                current_track_index: 0,
                total_tracks: 0,
                page: Page::Home,
                loaded_playlist_metadata: Vec::new(),
                current_owned_playlist: None,
                current_playlist_tracklist: None,
                downloaded_tracks: HashSet::new(),
                downloading_playlists: HashSet::new(),
                download_stopping_playlists: HashSet::new(),
                downloading_tracks: HashSet::new(),
                playing_playlists: HashSet::new(),
                paused_playlists: HashSet::new(),
                track_progress: AudioProgress::new(Duration::from_secs(0), Duration::from_secs(0)),
                playlist_loop_policies: HashMap::new(),
                downloaded_albums: Vec::new(),
                track_seeking: false,
                volume: 1.0,
                playlist_playling_tracks: HashMap::new(),
                recent_playlists: VecDeque::with_capacity(RECENT_PLAYLIST_SIZE),
                track_search_text: String::new(),
                playlist_scrolloffsets: HashMap::new(),
                theme: Theme::KanagawaDragon,
            },
            Task::perform(
                util::request_downloaded_tracks(playlist_sender_clone),
                |maybe_tracks| {
                    if let Ok(tracks) = maybe_tracks {
                        Message::DownloadedTrackListReceived(tracks)
                    } else {
                        Message::DownloadedTrackListReceived(HashSet::new())
                    }
                },
            ),
        )
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PlaylistTextEdit(txt) => {
                self.playlist_url = txt;
                Task::none()
            }
            Message::PlaylistURLSubmit => {
                println!("playlist url: {}", self.playlist_url);
                // try to create a url
                if let Ok(url) = Url::parse(&self.playlist_url) {
                    Task::perform(
                        submit_playlist_url(
                            self.id_counter.next(),
                            url,
                            self.playlist_sender.clone(),
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
                        self.loaded_playlist_metadata = playlist_data;
                        // sort the data
                        util::sort_playlist_metadata(&mut self.loaded_playlist_metadata);
                    }
                    EventMessage::TrackDownloadFinished { id, success } => {
                        // a given track finished downloading.
                        println!("Track download finished");
                        // add downloaded track to list and remove it from the downloading tracks list
                        self.downloading_tracks.remove(&id);
                        if success {
                            self.downloaded_tracks.insert(id);
                        }
                    }
                    EventMessage::TrackUpdated { track } => {
                        println!("track updated in gui");
                        // A track's data just updated. If its in the current playlist, update it for rendering.
                        if let Some(curr_play) = &mut self.current_owned_playlist {
                            if let Some(pos) =
                                curr_play.tracks.iter().position(|t| t.id() == track.id())
                            {
                                // replace the current track with the new one
                                println!("updating playlist @ gui");
                                curr_play.tracks[pos] = track;
                                // update tracklist
                                if let Some(tracklist) = &mut self.current_playlist_tracklist {
                                    tracklist.replace_tracks(curr_play.tracks.clone());
                                }
                            }
                        }
                    }
                    EventMessage::DownloadedAlbumsReceived(albums) => {
                        // convert hashmap to vec
                        let mut album_vec = albums.into_values().collect();
                        self.downloaded_albums.append(&mut album_vec);
                        // sort albums
                        util::sort_albums(&mut self.downloaded_albums);
                    }
                    EventMessage::AlbumDataDownloaded { album } => {
                        if !self.downloaded_albums.contains(&album) {
                            self.downloaded_albums.push(album);
                            util::sort_albums(&mut self.downloaded_albums);
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
                self.tasks.remove(&id);
                Task::none()
            }
            Message::PlaylistInitTaskStarted(task_id, handle) => {
                self.tasks.insert(task_id, handle);
                self.current_track_index = 0;
                self.total_tracks = 0;
                Task::none()
            }
            Message::PlaylistInitStatus(status) => {
                match status {
                    PlaylistInitStatus::Progress { current, total } => {
                        self.current_track_index = current;
                        self.total_tracks = total;
                    }
                    PlaylistInitStatus::Complete(metadata) => {
                        self.loaded_playlist_metadata.push(metadata);
                        // sort the data
                        util::sort_playlist_metadata(&mut self.loaded_playlist_metadata);
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
                Task::none()
            }
            Message::PlaylistSelect(playlist_metadata) => {
                println!("selected metadata: {playlist_metadata:?}");
                // request playlist
                let playlist_sender_clone = self.playlist_sender.clone();
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
                        self.id_counter.next(),
                        self.playlist_sender.clone(),
                        None,
                    ),
                    |handle| Message::TaskStarted {
                        handle: handle.unwrap(),
                    },
                );

                // update the recent playlists
                util::update_recent_playlists(
                    &mut self.recent_playlists,
                    owned_playlist.metadata.clone(),
                );

                let current_tracklist = TrackList::from_owned_playlist_ref(&owned_playlist);
                self.current_playlist_tracklist = Some(current_tracklist);

                self.paused_playlists
                    .insert(owned_playlist.metadata.id().clone());
                self.current_owned_playlist = Some(owned_playlist);
                self.page = Page::Player;

                task
            }
            Message::DownloadedTrackListReceived(tracks) => {
                // add to the list of downloaded tracks
                self.downloaded_tracks.extend(tracks);
                Task::none()
            }
            Message::Action(action) => {
                match action {
                    Action::Home => {
                        // reset everything player-wise
                        self.current_owned_playlist = None;
                        self.current_playlist_tracklist = None;
                        self.page = Page::Home;
                        Task::none()
                    }
                    Action::DownloadPlaylist { playlist_id } => {
                        // send request to playlist service to download
                        let playlist_sender_clone = self.playlist_sender.clone();
                        let next_id = self.id_counter.next();
                        // get the current tracklist for this playlist
                        let tracklist = self.current_playlist_tracklist.clone().unwrap();
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
                        let playlist_sender_clone = self.playlist_sender.clone();
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
                        let playlist_sender_clone = self.playlist_sender.clone();
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
                        let playlist_sender_clone = self.playlist_sender.clone();
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
                        let playlist_sender_clone = self.playlist_sender.clone();
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
                        let playlist_sender_clone = self.playlist_sender.clone();
                        Task::perform(
                            util::pause_current_playlist_track(
                                playlist_id.clone(),
                                playlist_sender_clone,
                            ),
                            |_result| Message::TrackAudioPauseResult { playlist_id },
                        )
                    }
                    Action::NextTrack { playlist_id } => {
                        let playlist_sender_clone = self.playlist_sender.clone();
                        Task::perform(
                            util::skip_current_playlist_track(
                                playlist_id.clone(),
                                playlist_sender_clone,
                            ),
                            |_result| Message::TrackAudioResumeResult { playlist_id },
                        )
                    }
                    Action::PreviousTrack { playlist_id } => {
                        let playlist_sender_clone = self.playlist_sender.clone();
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
                        let playlist_sender_clone = self.playlist_sender.clone();
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
                        self.track_progress.update_progress(progress);
                        self.track_seeking = true;

                        // if the playlist is currently playing, pause it
                        if self.playing_playlists.contains(&playlist_id) {
                            let playlist_sender_clone = self.playlist_sender.clone();
                            return Task::perform(
                                util::pause_current_playlist_track(
                                    playlist_id.clone(),
                                    playlist_sender_clone,
                                ),
                                |_result| Message::TrackAudioPauseResult { playlist_id },
                            );
                        }
                        Task::none()
                    }
                    Action::StopSeekingAudio { playlist_id } => {
                        // if the playlist is currently paused, then resume it
                        let progress = self.track_progress.progress();
                        if self.paused_playlists.contains(&playlist_id) {
                            let playlist_sender_clone = self.playlist_sender.clone();
                            return Task::perform(
                                util::resume_current_playlist_track(
                                    playlist_id.clone(),
                                    playlist_sender_clone,
                                    Some(progress),
                                ),
                                |_result| Message::TrackAudioResumeResult { playlist_id },
                            );
                        }
                        Task::none()
                    }
                    Action::LoopTrack { playlist_id } => {
                        // loop button pressed; advance policy to next one
                        let current_policy = {
                            match self.playlist_loop_policies.remove(&playlist_id) {
                                Some(policy) => policy,
                                None => LoopPolicy::NoLooping,
                            }
                        };
                        let next_policy = current_policy.next();

                        // send request to playlist service
                        let playlist_sender_clone = self.playlist_sender.clone();
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

                        if !matches!(next_policy, LoopPolicy::NoLooping) {
                            self.playlist_loop_policies.insert(playlist_id, next_policy);
                        }
                        task
                    }
                    Action::SetVolume { volume } => {
                        // if the volume here is different, send a req
                        if self.volume != volume {
                            let playlist_sender_clone = self.playlist_sender.clone();
                            self.volume = volume;
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
                self.tasks.insert(receiver_handle.id(), receiver_handle);
                // mark this playlist as downloading
                self.downloading_playlists.insert(id);
                Task::none()
            }
            Message::DownloadPlaylistEnded { id } => {
                self.downloading_playlists.remove(&id);
                self.download_stopping_playlists.remove(&id);
                println!(
                    "Download ended. Downloading playlists: {:?}; Stopping playlists: {:?}",
                    self.downloading_playlists, self.download_stopping_playlists
                );
                Task::none()
            }
            Message::PlaylistDownloadCancelStarted { id } => {
                self.downloading_playlists.remove(&id);
                self.download_stopping_playlists.insert(id);
                println!(
                    "Cancel started. Downloading playlists: {:?}; Stopping playlists: {:?}",
                    self.downloading_playlists, self.download_stopping_playlists
                );
                Task::none()
            }
            Message::TrackDownloadStarted { id } => {
                // a given track started downloading.
                println!("track download started");
                self.downloading_tracks.insert(id);
                Task::none()
            }
            Message::TrackDownloadStatus { id: _, data: _ } => {
                // A given track's download status updated.
                Task::none()
            }
            Message::PlaylistOrderUpdated { id, tracklist } => {
                println!("Playlist order updated");
                self.current_playlist_tracklist = Some(tracklist);
                // mark the playlist as playing because thats what happens
                self.paused_playlists.remove(&id);
                self.playing_playlists.insert(id);
                Task::none()
            }
            Message::TrackAudioProgress { id: _, progress } => {
                // if the user is currently seeking, ignore this
                if self.track_seeking {
                    return Task::none();
                }
                self.track_progress = progress;
                Task::none()
            }
            Message::TrackAudioStart {
                id,
                maybe_playlist_id,
            } => {
                println!("track audio start");
                if let Some(pid) = maybe_playlist_id {
                    if let Some(curr_playlist) = &self.current_owned_playlist {
                        if let Some(track) = curr_playlist.tracks.iter().find(|t| *t.id() == id) {
                            self.playlist_playling_tracks.insert(pid, track.clone());
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
                if let Some(playlist_id) = &maybe_playlist_id {
                    self.playlist_loop_policies.remove(playlist_id);
                }
                // keeping track of the current playing track in the current playlist
                if let Some(pid) = maybe_playlist_id {
                    if let Some(curr_playlist) = &self.current_owned_playlist {
                        if *curr_playlist.metadata.id() == pid && curr_playlist.contains_track(&pid)
                        {
                            self.playlist_playling_tracks.remove(&pid);
                        }
                    }
                }
                Task::none()
            }
            Message::TaskStarted { handle } => {
                self.tasks.insert(handle.id(), handle);
                Task::none()
            }
            Message::TrackAudioPauseResult { playlist_id } => {
                if self.playing_playlists.remove(&playlist_id) {
                    self.paused_playlists.insert(playlist_id);
                }
                Task::none()
            }
            Message::TrackAudioPreviousResult { playlist_id: _ } => Task::none(),
            Message::TrackAudioSkipResult { playlist_id: _ } => Task::none(),
            Message::TrackAudioResumeResult { playlist_id } => {
                // when playing audio seeking cannot happen,
                // so to make sure the progress bar doesn't feel
                // weird this can be enabled after audio continues.
                self.track_seeking = false;
                if self.paused_playlists.remove(&playlist_id) {
                    self.playing_playlists.insert(playlist_id);
                }
                Task::none()
            }
            Message::PlayPlaylistEnded { playlist_id } => {
                self.playing_playlists.remove(&playlist_id);
                self.paused_playlists.remove(&playlist_id);
                Task::none()
            }
            Message::TrackLooped {
                maybe_playlist_id,
                track_id: _,
            } => {
                if let Some(playlist_id) = maybe_playlist_id {
                    // decrement the loop for gui showing
                    if let Some(prev_policy) = self.playlist_loop_policies.remove(&playlist_id) {
                        let new_policy = prev_policy.looped();
                        if !matches!(new_policy, LoopPolicy::NoLooping) {
                            // put the new policy back in
                            self.playlist_loop_policies.insert(playlist_id, new_policy);
                        }
                    }
                }
                Task::none()
            }
            Message::PlayTrackResult { playlist_id } => {
                // make sure to set the play button as 'playing'
                if let Some(pid) = playlist_id
                    && let Some(cur_playlist) = &self.current_owned_playlist
                    && self.paused_playlists.contains(cur_playlist.metadata.id())
                {
                    let curr_pid = cur_playlist.metadata.id();
                    if pid == *curr_pid {
                        // track played was this playlist
                        self.paused_playlists.remove(curr_pid);
                        self.playing_playlists.insert(curr_pid.clone());
                    }
                }
                Task::none()
            }
            Message::TrackSearchTextEdit(txt) => {
                self.track_search_text = txt;
                Task::none()
            }
            Message::TracklistScrolled {
                playlist_id,
                scrollable_viewport,
            } => {
                // for playlist tracklist scrollable
                self.playlist_scrolloffsets
                    .insert(playlist_id, scrollable_viewport.absolute_offset().y);
                Task::none()
            }
            Message::ThemeUpdated { theme } => {
                self.theme = theme;
                Task::none()
            }
            Message::SetGlobalVolumeResult => Task::none(),
            Message::SetPlaylistLoopPolicyResult { playlist_id: _ } => Task::none(),
            Message::None => Task::none(),
        }
    }

    fn view(&self) -> Column<'_, Message> {
        match self.page {
            Page::Home => home(&self),
            Page::Player => player(&self),
        }
    }
    fn subscription(&self) -> Subscription<Message> {
        let bus = self.event_bus.watch(
            |_id, msg| Message::EventRecieved(msg),
            |_id| Message::EventBusClosed,
        );
        let tasks = Subscription::batch(
            self.tasks
                .values()
                .map(|handle| handle.watch(|_id, msg| msg, |id| Message::TaskFinished(id))),
        );

        Subscription::batch(vec![bus, tasks])
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
        shutdown_token: CancellationToken,
        playlist_sender: PlaylistSender,
        event_bus_rx: mpsc::Receiver<EventMessage>,
    ) -> iced::Result {
        let mut id_counter = IdCounter::new();
        let event_recv_id = id_counter.next();

        let flags = GuiFlags {
            shutdown_token,
            playlist_sender,
            event_receiver: ReceiverHandle::new(event_recv_id, event_bus_rx),
            id_counter,
        };

        let application =
            iced::application(move || App::new(flags.clone()), App::update, App::view)
                .subscription(App::subscription)
                .theme(App::theme)
                .title("peanut")
                .exit_on_close_request(true);

        application.run()
    }
}

// helper methods
async fn submit_playlist_url(task_id: u64, url: Url, sender: PlaylistSender) -> Message {
    println!("submitting playlist url..");
    // create oneshot channel to get progress update from
    let (tx, rx) = oneshot::channel();

    let _ = sender
        .send(PlaylistMessage::InitializePlaylist {
            url,
            reply_stream: tx,
        })
        .await;

    match rx.await {
        Ok(raw_recv) => {
            println!("Sending playlist init task started msg");
            let handle = ReceiverHandle::new(task_id, raw_recv);
            Message::PlaylistInitTaskStarted(task_id, handle)
        }
        Err(_) => {
            println!("something went wrong when submitting playlist url?");
            Message::None
        }
    }
}
