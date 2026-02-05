use musicbrainz_rs::MusicBrainzClient;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};

use crate::service::{
    audio::structs::{AudioConfig, AudioProgress},
    id::structs::Id,
    playlist::{PlaylistSender, structs::Album},
};

#[derive(Debug)]
pub enum AudioMessage {
    PlayAudio {
        id: Id,
        maybe_playlist_id: Option<Id>,
        audio_config: AudioConfig,
        progress_sender: mpsc::Sender<(Id, AudioProgress)>,
        on_end: oneshot::Sender<anyhow::Result<()>>,
        on_loop: PlaylistSender,
    },
    AudioFinished {
        id: Id,
        result: anyhow::Result<()>,
    },
    PauseAudio {
        id: Id,
        result: oneshot::Sender<anyhow::Result<()>>,
    },
    ResumeAudio {
        id: Id,
        result: oneshot::Sender<anyhow::Result<()>>,
    },
    StopAudio {
        id: Id,
        result: oneshot::Sender<anyhow::Result<()>>,
    },
    SeekAudio {
        id: Id,
        percentage: f64,
        result: oneshot::Sender<anyhow::Result<()>>,
    },
    SetAudioLoop {
        id: Id,
        loop_policy: LoopPolicy,
        result: oneshot::Sender<anyhow::Result<()>>,
    },
    AudioLooped {
        id: Id,
    },
    SetAudioVolume {
        id: Id,
        volume: f64,
        result: oneshot::Sender<anyhow::Result<()>>,
    },
    GetMusicBrainzClient {
        result: oneshot::Sender<MusicBrainzClient>,
    },
}

#[derive(Debug, Clone)]
pub enum LoopPolicy {
    NoLooping,
    Once,
    Infinite,
}
impl LoopPolicy {
    // takes the current policy and moves to the next one in the list
    // when activated by the loop button.
    pub fn next(self) -> Self {
        match self {
            Self::Infinite => Self::NoLooping,
            Self::NoLooping => Self::Once,
            Self::Once => Self::Infinite,
        }
    }
    // returns the new looping policy for when this policy looped.
    // basically a 'downgrade.'
    pub fn looped(self) -> Self {
        match self {
            Self::Infinite => Self::Infinite,
            _ => Self::NoLooping,
        }
    }
}

/// Confidence enum used for scrubbing Youtube track titles.
/// Each variant has a distinct confidence level:
/// High: has "official" in title, meaning uploader is likely artist
/// Medium: standard title split
/// Low: assume whole title is track title, and uploader is artist
#[derive(Debug)]
pub enum ExtractorConfidence {
    High,
    Medium,
    Low,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum AlbumKind {
    Album(Album),
    Single,
    Unknown,
}
