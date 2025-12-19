use url::Url;

pub enum PlaylistMessage {
    InitializePlaylist { url: Url },
}

pub enum TrackSource {
    Youtube,
}
