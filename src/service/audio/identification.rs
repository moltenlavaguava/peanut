// Module for trying to extract track data from youtube video titles

// create regexes

use std::str::FromStr;

use musicbrainz_rs::{
    MusicBrainzClient, Search,
    entity::{artist_credit::ArtistCredit, recording::Recording, release::ReleaseStatus},
};
use regex::Regex;
use url::Url;

use crate::service::{
    audio::{
        enums::{AlbumKind, ExtractorConfidence},
        structs::{UpdatedTrackData, YoutubeTitleMetadata},
    },
    id::{enums::Platform, structs::Id},
    playlist::{
        enums::{Artist, MediaType},
        structs::{Album, Track},
    },
};

lazy_static::lazy_static! {
    // Quoted regex
    // Mostly for calamity mod music format
    // Matches: <anything> Music - "title"
    static ref RE_QUOTED: Regex = Regex::new(r#"(?i)Music\s*-\s*["“](.+?)["”]"#).unwrap();

    // Standard split regex
    // matches a - b (with any amount of separators)
    static ref RE_SPLIT: Regex = Regex::new(r"\s+[-|:~]\s+").unwrap();

    // Removes garbage
    static ref RE_GARBAGE: Regex = Regex::new(r"(?i)(\(|\[).*(official|video|audio|lyrics|hq|4k|hd|mix|remix|topic|ost|soundtrack|theme|\.ogg|download).*?(\)|\])").unwrap();

    // Matches () or [] at the end of the name
    static ref RE_END_PARENTHESES: Regex = Regex::new(r"\s*(\(|\[).*?(\)|\])$").unwrap();

    // Removes any 'feat' garbage
    static ref RE_FEAT: Regex = Regex::new(r"(?i)\s(ft\.|feat\.|featuring)\s.*").unwrap();

    // Removes numbers at the start
    static ref RE_NUM_START: Regex = Regex::new(r"^\d+[\.\-\s]\s*").unwrap();
}

pub fn extract_metadata(track: &Track) -> YoutubeTitleMetadata {
    let raw_title = track.title.clone();
    let channel_name = {
        let artist_enum = track.artist.clone();
        match artist_enum {
            Artist::Official(artist) => {
                // trust the uploader is the artist
                // build metadata accordingly
                return YoutubeTitleMetadata {
                    track_title: clean_string(&raw_title),
                    main_artist_string: artist[0].clone(),
                    extract_confidence: ExtractorConfidence::High,
                };
            }
            Artist::Community(artist) => artist,
        }
    };

    // Check 1: see if this channel is a topic channel
    if channel_name.contains(" - Topic") {
        return YoutubeTitleMetadata {
            track_title: raw_title,
            main_artist_string: channel_name.replace(" - Topic", "").trim().to_string(),
            extract_confidence: ExtractorConfidence::High,
        };
    };

    // Check 2: see if the title has 'official' in it
    if raw_title.to_lowercase().contains("official") {
        let mut clean_title = clean_string(&raw_title);

        // check if the start of the title is the artist
        if clean_title
            .to_lowercase()
            .starts_with(&channel_name.to_lowercase())
        {
            // remove the artist name from the title
            clean_title = clean_title[channel_name.len()..].to_string();
            // remove any separators
            clean_title = clean_title
                .trim_start_matches(|c: char| {
                    c == '-' || c == '|' || c == ':' || c == '~' || c.is_whitespace()
                })
                .to_string();
        }

        // handle leftovers
        if RE_SPLIT.is_match(&clean_title) {
            let parts: Vec<&str> = RE_SPLIT.split(&clean_title).collect();
            if let Some(last_part) = parts.last() {
                clean_title = last_part.trim().to_string();
            }
        }

        return YoutubeTitleMetadata {
            track_title: clean_title,
            main_artist_string: channel_name,
            extract_confidence: ExtractorConfidence::High,
        };
    }

    // Check 3: 'music' is in the title alongside a track in quotes
    if let Some(caps) = RE_QUOTED.captures(&raw_title) {
        return YoutubeTitleMetadata {
            track_title: clean_string(&caps[1]),
            main_artist_string: channel_name,
            extract_confidence: ExtractorConfidence::High,
        };
    }

    // Check 4: normal 'artist - title' split
    if RE_SPLIT.is_match(&raw_title) {
        let parts: Vec<&str> = RE_SPLIT.split(&raw_title).collect();

        let candidates: Vec<&str> = parts
            .iter()
            // Removes any 'noise words' from the candidate list.
            // currently not being used
            // .filter(|p| !is_noise_word(p))
            .map(|s| *s)
            .collect();

        let len = candidates.len();

        // specifically for celeste bsides and anything else that may follow this format
        if len >= 3 {
            let has_track_num = candidates
                .iter()
                .any(|s| s.trim().chars().all(char::is_numeric));

            if has_track_num {
                return YoutubeTitleMetadata {
                    main_artist_string: clean_string(candidates[len - 2]),
                    track_title: clean_string(candidates[len - 1]),
                    extract_confidence: ExtractorConfidence::Low,
                };
            }
        }

        if candidates.len() >= 2 {
            // assume first word is artist / ost or something, second is title
            let mut confidence = ExtractorConfidence::Medium;
            let main_artist_string = {
                let artist_maybe = candidates[0];
                if is_music_word(artist_maybe) {
                    confidence = ExtractorConfidence::Low;
                    channel_name
                } else {
                    artist_maybe.to_string()
                }
            };
            let mut track_title = clean_string(candidates[1]);
            // if the track title and artist are the same, just go for the first option and hope for the best
            if track_title.to_lowercase() == main_artist_string.to_lowercase() {
                track_title = clean_string(candidates[0]);
            }
            println!("Track: {track:?} is candidate for split");
            return YoutubeTitleMetadata {
                track_title: track_title,
                main_artist_string,
                extract_confidence: confidence,
            };
        }
    }

    // Check 5: fallback
    let clean_title = clean_string(&raw_title);
    YoutubeTitleMetadata {
        track_title: clean_title,
        main_artist_string: channel_name,
        extract_confidence: ExtractorConfidence::Low,
    }
}

fn clean_string(string: &str) -> String {
    // remove garbage from string
    let mut string = string.to_string();
    // normal garbage (disabled)
    string = RE_GARBAGE.replace_all(&string, "").to_string();
    // remove anything inside () at the end of the string
    string = RE_END_PARENTHESES.replace_all(&string, "").to_string();
    // features
    string = RE_FEAT.replace_all(&string, "").to_string();
    // starting numbers (ex: 1. song)
    string = RE_NUM_START.replace_all(&string, "").to_string();
    string.trim().to_string()
}

fn is_music_word(s: &str) -> bool {
    let s = s.to_lowercase();
    s.contains("ost")
        || s.contains("soundtrack")
        || s.contains("music")
        || s.contains("remix")
        || s.contains("audio")
}

pub async fn verify_track_information(
    title_metadata: YoutubeTitleMetadata,
    client: &MusicBrainzClient,
) -> Option<UpdatedTrackData> {
    // send request to musicbrainz
    println!("searching..?");
    let safe_title = title_metadata.track_title.replace("\"", "");
    let safe_artist = title_metadata.main_artist_string.replace("\"", "");
    let query = format!(
        r#"query=recording:"{}"~ AND artist:"{}"~"#,
        safe_title, safe_artist
    );
    let result = Recording::search(query)
        // .with_artists()
        // .with_releases()
        .execute_with_client(&client)
        .await
        .ok();

    match result {
        None => None,
        Some(search_result) => {
            if search_result.count != 1 {
                None
            } else {
                let recording_data = search_result.entities.first().unwrap();
                let title = recording_data.title.clone();
                let album_kind = get_album_from_recording(&recording_data);
                match &recording_data.artist_credit {
                    None => None,
                    Some(credits) => {
                        let artists = get_artists(&credits);
                        Some(UpdatedTrackData {
                            title,
                            album_kind,
                            artists,
                        })
                    }
                }
            }
        }
    }
}

fn get_album_from_recording(recording: &Recording) -> AlbumKind {
    match &recording.releases {
        None => return AlbumKind::Single,
        Some(releases) => {
            if let Some(r) = releases.iter().find(|r| match r.status.as_ref() {
                Some(status) => matches!(status, ReleaseStatus::Official),
                None => false,
            }) {
                if let Some(artists) = &r.artist_credit {
                    let artists = get_artists(artists);
                    let album_title = r.title.clone();
                    let img_url = Url::from_str(&format!(
                        "https://coverartarchive.org/release/{}/front-500",
                        &r.id
                    ))
                    .unwrap();
                    let album_id = Id::new(Platform::MusicBrainz, MediaType::Album, r.id.clone());
                    return AlbumKind::Album(Album {
                        name: album_title,
                        source_id: album_id.clone(),
                        dyn_id: album_id,
                        artists,
                        img_url,
                    });
                }
            }
        }
    }

    AlbumKind::Unknown
}
fn get_artists(artist_credits: &Vec<ArtistCredit>) -> Vec<String> {
    artist_credits
        .iter()
        .map(|c| c.artist.name.clone())
        .collect()
}
