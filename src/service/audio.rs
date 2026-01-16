use std::collections::HashMap;

use kira::{AudioManager, AudioManagerSettings};
use tokio::sync::mpsc;

use crate::{
    service::{
        audio::{enums::AudioMessage, structs::AudioHandleWrapper},
        gui::enums::EventSender,
        id::structs::Id,
    },
    util::service::ServiceLogic,
};

pub mod enums;
mod structs;
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
            } => {
                println!("playing audio in audio srevice");
                if self.playing_cache.contains_key(&id) {
                    println!("failed to play audio; id is already present in cache");
                }
                let handle = util::play_audio(
                    id.clone(),
                    progress_sender,
                    self.audio_sender.clone(),
                    audio_config,
                    &mut self.manager,
                );
                match handle {
                    Ok(handle) => {
                        let handle_wrapper = AudioHandleWrapper { handle, on_end };
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
        }
    }
}
