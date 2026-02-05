use std::collections::HashMap;

use crate::service::{
    id::structs::Id,
    playlist::structs::{Track, TrackIdVec, TrackVec},
};

/// Takes the playlist service's track cache and returns a playlist specific list of tracks.
pub fn clone_tracks_from_cache(
    track_ids: TrackIdVec,
    track_cache: &HashMap<Id, Track>,
) -> TrackVec {
    TrackVec(
        track_ids
            .0
            .iter()
            .filter_map(|id| track_cache.get(&id).map(|t| t.clone()))
            .collect(),
    )
}
