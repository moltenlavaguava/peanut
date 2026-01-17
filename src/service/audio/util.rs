use std::{sync::Arc, time::Duration};

use kira::{
    AudioManager,
    sound::{
        FromFileError, PlaybackState,
        streaming::{StreamingSoundData, StreamingSoundHandle},
    },
};
use parking_lot::Mutex;
use tokio::sync::mpsc;

use crate::service::{
    audio::{
        enums::AudioMessage,
        structs::{AudioConfig, AudioProgress},
    },
    file,
    id::structs::Id,
};

// how often each audio file's heartbeat will poll (delay, in ms)
const AUDIO_HEARTBEAT_RATE: u64 = 100;

pub fn play_audio(
    track_id: Id,
    progress_sender: mpsc::Sender<(Id, AudioProgress)>,
    on_end: mpsc::Sender<AudioMessage>,
    _audio_config: AudioConfig,
    audio_manager: &mut AudioManager,
) -> anyhow::Result<Arc<Mutex<StreamingSoundHandle<FromFileError>>>> {
    // get the file path from the track id
    let path = file::util::track_file_path_from_id(&track_id)?;
    // do audio things
    println!("doing data");
    let data = StreamingSoundData::from_file(path)?;
    println!("got data");
    let duration = data.duration();

    // play the audio and get its handle
    let handle = audio_manager.play(data)?;
    // needed to make audio service still have access to handle
    let handle_arc = Arc::new(Mutex::new(handle));
    let handle_arc_clone = Arc::clone(&handle_arc);

    // spawn task monitoring audio
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(AUDIO_HEARTBEAT_RATE));
        let mut last_pos = -1.0;
        // if the sender was dropped, then don't try to keep sending
        let mut dropped = false;
        loop {
            // run heartbeat logic
            let (pos, state) = {
                let guard = handle_arc_clone.lock();
                (guard.position(), guard.state())
            };

            // if the current pos is actually different, then do something
            if pos != last_pos && !dropped {
                // send the update
                let progress = AudioProgress::new(Duration::from_secs_f64(pos), duration.clone());
                // if the recv was dropped, then stop heartbeating
                if let Err(_) = progress_sender.send((track_id.clone(), progress)).await {
                    dropped = true;
                }
            }

            // if the audio is stopped, then stop too
            if let PlaybackState::Stopped = state {
                break;
            }

            last_pos = pos;
            // wait for next tick
            interval.tick().await;
        }

        // audio ended; send the end signal
        on_end
            .send(AudioMessage::AudioFinished {
                id: track_id,
                result: Ok(()),
            })
            .await
            .unwrap();
    });

    Ok(handle_arc)
}
