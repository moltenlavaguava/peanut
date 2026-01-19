use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};

use atomic_float::AtomicF64;
use kira::{
    AudioManager, Tween,
    sound::{
        PlaybackState,
        static_sound::{StaticSoundData, StaticSoundHandle},
    },
};
use parking_lot::Mutex;
use tokio::sync::mpsc;

use crate::service::{
    audio::{
        AudioSender,
        enums::AudioMessage,
        structs::{AudioConfig, AudioProgress},
    },
    file,
    id::structs::Id,
};

// how often each audio file's heartbeat will poll (delay, in ms)
const AUDIO_HEARTBEAT_RATE: u64 = 100;

pub async fn play_audio(
    track_id: Id,
    progress_sender: mpsc::Sender<(Id, AudioProgress)>,
    audio_sender: AudioSender,
    audio_config: AudioConfig,
    audio_manager: &mut AudioManager,
    last_known_pos_arc: Arc<AtomicF64>,
    seek_count_arc: Arc<AtomicU64>,
) -> anyhow::Result<(Arc<Mutex<StaticSoundHandle>>, Duration)> {
    // get the file path from the track id
    let path = file::util::track_file_path_from_id(&track_id)?;
    // do audio things
    let mut data = tokio::task::spawn_blocking(move || StaticSoundData::from_file(path)).await??;
    data = data.volume(linear_to_db(audio_config.volume()) as f32);
    // data = data.volume(-100.0);
    let duration = data.duration();

    // play the audio and get its handle
    let mut handle = audio_manager.play(data)?;

    if audio_config.start_paused() {
        handle.pause(Tween::default());
    }

    // needed to make audio service still have access to handle
    let handle_arc = Arc::new(Mutex::new(handle));
    let handle_arc_clone = Arc::clone(&handle_arc);

    // spawn task monitoring audio
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(AUDIO_HEARTBEAT_RATE));

        let mut local_seek_count = seek_count_arc.load(Ordering::Relaxed);
        // if the sender was dropped, then don't try to keep sending
        let mut dropped = false;
        loop {
            // wait for next tick
            interval.tick().await;

            // get the seek count before the audio logic
            let start_seek_count = seek_count_arc.load(Ordering::Relaxed);

            // run heartbeat logic
            let (pos, state) = {
                let guard = handle_arc_clone.lock();
                (guard.position(), guard.state())
            };

            // if the audio is stopped, then stop too
            if let PlaybackState::Stopped = state {
                break;
            }

            // load last known position + seek count after getting audio data
            let last_known_pos = last_known_pos_arc.load(Ordering::Relaxed);
            let end_seek_count = seek_count_arc.load(Ordering::Relaxed);

            // ~ loop logic ~

            // if something happened while audio data was read, then don't do anything
            // not sure how this can happen but ai suggested it
            if start_seek_count != end_seek_count {
                local_seek_count = end_seek_count;
                continue;
            }

            // if a seek happened between last tick and now; update the values
            // but don't trigger loop checking or gui updating
            if start_seek_count != local_seek_count {
                // apparently the audio engine lags, and this ensures
                // we don't do anything if the lag is too crazy
                let lag = (pos - last_known_pos).abs();

                if lag > 0.5 {
                    continue;
                }
                local_seek_count = start_seek_count;
            } else {
                // standard loop detection. cushion added for weird floating point stuff
                if pos < last_known_pos - 0.5 {
                    let _ = audio_sender
                        .send(AudioMessage::AudioLooped {
                            id: track_id.clone(),
                        })
                        .await;
                }

                // update the gui if the position changed and a seek didn't happen last frame
                if pos != last_known_pos && !dropped {
                    // send the update
                    let progress =
                        AudioProgress::new(Duration::from_secs_f64(pos), duration.clone());
                    // if the recv was dropped, then stop heartbeating
                    if let Err(_) = progress_sender.send((track_id.clone(), progress)).await {
                        dropped = true;
                    }
                }
            }

            // update last known pos if no seek happened
            if seek_count_arc.load(Ordering::Relaxed) == start_seek_count {
                // println!("Updating last known pos.. ({last_known_pos}s), current pos: {pos}s");
                last_known_pos_arc.store(pos, Ordering::Relaxed);
            }
        }

        // audio ended; send the end signal
        audio_sender
            .send(AudioMessage::AudioFinished {
                id: track_id,
                result: Ok(()),
            })
            .await
            .unwrap();
    });

    Ok((handle_arc, duration))
}

pub fn linear_to_db(linear: f64) -> f64 {
    if linear <= 0.0001 {
        -100.0 // floor for silence
    } else {
        20.0 * linear.log10()
    }
}
