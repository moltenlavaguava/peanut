use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use atomic_float::AtomicF64;
use kira::{AudioManager, AudioManagerSettings, Tween};
use tokio::sync::mpsc;

use anyhow::anyhow;

use crate::{
    service::{
        audio::{
            enums::{AudioMessage, LoopPolicy},
            structs::AudioHandleWrapper,
        },
        gui::enums::EventSender,
        id::structs::Id,
        playlist::enums::PlaylistMessage,
    },
    util::service::ServiceLogic,
};

pub mod enums;
pub mod structs;
mod util;

pub type AudioSender = mpsc::Sender<AudioMessage>;

/// Handles playlist management.
pub struct AudioService {
    _event_sender: EventSender,
    audio_sender: AudioSender,
    manager: AudioManager,
    playing_cache: HashMap<Id, AudioHandleWrapper>,
}

pub struct AudioFlags {
    pub event_sender: EventSender,
    pub audio_sender: AudioSender,
}

impl AudioService {
    pub fn new(flags: AudioFlags) -> Self {
        let manager = AudioManager::new(AudioManagerSettings::default())
            .expect("Failed to create audio manager");
        Self {
            manager,
            audio_sender: flags.audio_sender,
            _event_sender: flags.event_sender,
            playing_cache: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl ServiceLogic<AudioMessage> for AudioService {
    fn name(&self) -> &'static str {
        "AudioService"
    }
    async fn handle_message(&mut self, msg: AudioMessage) {
        match msg {
            AudioMessage::PlayAudio {
                id,
                progress_sender,
                on_end,
                audio_config,
                on_loop,
                maybe_playlist_id,
            } => {
                if self.playing_cache.contains_key(&id) {
                    println!("failed to play audio; id is already present in cache");
                }
                // create arc to share last known position
                let last_known_pos_arc = Arc::new(AtomicF64::new(0.0));
                let seek_count_arc = Arc::new(AtomicU64::new(0));
                let handle = util::play_audio(
                    id.clone(),
                    progress_sender,
                    self.audio_sender.clone(),
                    audio_config,
                    &mut self.manager,
                    Arc::clone(&last_known_pos_arc),
                    Arc::clone(&seek_count_arc),
                )
                .await;
                match handle {
                    Ok((handle, audio_duration)) => {
                        let handle_wrapper = AudioHandleWrapper {
                            handle,
                            on_end,
                            audio_duration,
                            last_known_pos: last_known_pos_arc,
                            seek_count: seek_count_arc,
                            loop_policy: LoopPolicy::NoLooping,
                            on_loop,
                            maybe_playlist_id,
                        };
                        self.playing_cache.insert(id, handle_wrapper);
                    }
                    Err(e) => {
                        println!("Ran into issue when playing audio: {e}");
                    }
                }
            }
            AudioMessage::AudioFinished { id, result } => {
                println!("audio finished");
                if let Some(handle) = self.playing_cache.remove(&id) {
                    let _ = handle.on_end.send(result);
                }
            }
            AudioMessage::PauseAudio { id, result } => match self.playing_cache.get(&id) {
                Some(wrapper) => {
                    let mut guard = wrapper.handle.lock();
                    guard.pause(Tween::default());
                    let _ = result.send(Ok(()));
                }
                None => {
                    let _ = result.send(Err(anyhow!("Audio not currently playing")));
                }
            },
            AudioMessage::ResumeAudio { id, result } => match self.playing_cache.get(&id) {
                Some(wrapper) => {
                    let mut guard = wrapper.handle.lock();
                    guard.resume(Tween::default());
                    let _ = result.send(Ok(()));
                }
                None => {
                    let _ = result.send(Err(anyhow!("Audio not currently playing")));
                }
            },
            AudioMessage::StopAudio { id, result } => match self.playing_cache.get(&id) {
                Some(wrapper) => {
                    let mut guard = wrapper.handle.lock();
                    guard.stop(Tween::default());
                    let _ = result.send(Ok(()));
                }
                None => {
                    let _ = result.send(Err(anyhow!("Audio not currently playing")));
                }
            },
            AudioMessage::SeekAudio {
                id,
                percentage,
                result,
            } => match self.playing_cache.get_mut(&id) {
                Some(wrapper) => {
                    // handle all guard stuff before using awaits
                    let (current_pos, last_known_pos) = {
                        let mut guard = wrapper.handle.lock();

                        let current_pos = guard.position();
                        let last_known_pos = wrapper.last_known_pos.load(Ordering::Relaxed);

                        // update the current seek count for looping purposes
                        wrapper.seek_count.fetch_add(1, Ordering::Relaxed);

                        let seek_secs = percentage * wrapper.audio_duration.as_secs_f64();
                        guard.seek_to(seek_secs);

                        // update history
                        wrapper.last_known_pos.store(seek_secs, Ordering::Relaxed);
                        (current_pos, last_known_pos)
                    };

                    // cushion added for weird floating point stuff
                    if current_pos < last_known_pos - 0.5 {
                        // println!("caught loop in audio seek");
                        let _ = self
                            .audio_sender
                            .send(AudioMessage::AudioLooped { id: id.clone() })
                            .await;
                    }
                }
                None => {
                    let _ = result.send(Err(anyhow!("Audio not currently playing")));
                }
            },
            AudioMessage::SetAudioLoop {
                id,
                loop_policy,
                result,
            } => match self.playing_cache.get_mut(&id) {
                Some(wrapper) => {
                    println!("setting loop policy @ audio ({loop_policy:?})");
                    let mut guard = wrapper.handle.lock();
                    // act on the loop policy
                    match loop_policy {
                        LoopPolicy::NoLooping => {
                            guard.set_loop_region(None);
                        }
                        LoopPolicy::Once => {
                            guard.set_loop_region(0.0..);
                        }
                        LoopPolicy::Infinite => {
                            guard.set_loop_region(0.0..);
                        }
                    }
                    // store it in the wrapper
                    wrapper.loop_policy = loop_policy;
                }
                None => {
                    let _ = result.send(Err(anyhow!("Audio not currently playing")));
                }
            },
            AudioMessage::AudioLooped { id } => {
                println!("Audio looped");
                if let Some(wrapper) = self.playing_cache.get_mut(&id) {
                    // send update to playlist service and handle unlooping logic
                    let _ = wrapper
                        .on_loop
                        .send(PlaylistMessage::TrackLooped {
                            maybe_playlist_id: wrapper.maybe_playlist_id.clone(),
                            track_id: id,
                        })
                        .await;

                    let next_policy = wrapper.loop_policy.clone().looped();
                    // actually act on that policy
                    if let LoopPolicy::NoLooping = next_policy {
                        // stop looping
                        let mut guard = wrapper.handle.lock();
                        guard.set_loop_region(None);
                    }

                    wrapper.loop_policy = next_policy;
                }
            }
            AudioMessage::SetAudioVolume { id, volume, result } => {
                if let Some(wrapper) = self.playing_cache.get_mut(&id) {
                    let mut guard = wrapper.handle.lock();
                    guard.set_volume(util::linear_to_db(volume) as f32, Tween::default());

                    let _ = result.send(Ok(()));
                } else {
                    let _ = result.send(Err(anyhow!("Audio not currently playing")));
                }
            }
        }
    }
}
