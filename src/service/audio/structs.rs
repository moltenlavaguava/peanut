use std::{sync::Arc, time::Duration};

use kira::sound::{FromFileError, streaming::StreamingSoundHandle};
use parking_lot::Mutex;
use tokio::sync::oneshot;

pub struct AudioConfig {}
impl AudioConfig {
    pub fn new() -> Self {
        Self {}
    }
}

// Small wrapper for audio handles; contains other information relevant to the handle.
pub struct AudioHandleWrapper {
    pub handle: Arc<Mutex<StreamingSoundHandle<FromFileError>>>,
    pub on_end: oneshot::Sender<anyhow::Result<()>>,
}

pub struct AudioProgress {
    current: Duration,
    total: Duration,
}
impl AudioProgress {
    pub fn new(current: Duration, total: Duration) -> Self {
        Self { current, total }
    }
    // gives progress as a decimal (eg. 0.26)
    pub fn progress(&self) -> f32 {
        self.current().as_millis() as f32 / self.total().as_millis() as f32
    }
    pub fn current(&self) -> &Duration {
        &self.current
    }
    pub fn total(&self) -> &Duration {
        &self.total
    }
}
