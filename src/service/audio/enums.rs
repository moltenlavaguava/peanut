use tokio::sync::{mpsc, oneshot};

use crate::service::{
    audio::structs::{AudioConfig, AudioProgress},
    id::structs::Id,
};

pub enum AudioMessage {
    PlayAudio {
        id: Id,
        audio_config: AudioConfig,
        progress_sender: mpsc::Sender<AudioProgress>,
        on_end: oneshot::Sender<anyhow::Result<()>>,
    },
    AudioFinished {
        id: Id,
        result: anyhow::Result<()>,
    },
}
