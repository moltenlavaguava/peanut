use crate::service::file::enums::TrackDownloadState;

#[derive(Clone)]
pub struct IdCounter {
    id: u64,
}
impl IdCounter {
    pub fn new() -> Self {
        Self { id: 0 }
    }
    pub fn next(&mut self) -> u64 {
        self.id += 1;
        self.id
    }
}

#[derive(Hash, Clone, Debug)]
pub struct TrackRenderData {
    pub download_state: TrackDownloadState,
    pub title: String,
}
