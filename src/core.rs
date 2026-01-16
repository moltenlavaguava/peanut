use crate::service::audio::{AudioFlags, AudioService};
use crate::service::gui::GuiService;
use crate::service::playlist::{PlaylistFlags, PlaylistService};
use crate::service::process::ProcessService;
use crate::util::service::run_service;
use futures::future;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

/// Handles starting and shutdown of the program.
pub struct CoreService;

impl CoreService {
    pub fn spawn() -> () {
        // Handler creation
        let (t_bus, r_bus) = mpsc::channel(100);
        let (t_process, r_process) = mpsc::channel(100);
        let (t_playlist, r_playlist) = mpsc::channel(100);
        let (t_audio, r_audio) = mpsc::channel(100);

        // Service creation

        // gui service
        let gui_service = GuiService::new();

        // playlist service
        let playlist_flags = PlaylistFlags {
            event_sender: t_bus.clone(),
            process_sender: t_process.clone(),
            playlist_sender: t_playlist.clone(),
            audio_sender: t_audio.clone(),
        };
        let playlist_service = PlaylistService::new(playlist_flags);

        // process service
        let process_service = ProcessService::new(t_bus.clone());

        // audio service
        let audio_flags = AudioFlags {
            audio_sender: t_audio.clone(),
            event_sender: t_bus.clone(),
        };
        let audio_service = AudioService::new(audio_flags);

        // Runtime creation
        let runtime = Runtime::new().expect("Failed to create tokio runtime");
        let _guard = runtime.enter(); // must assign variable to guard i believe

        let cancel_token = CancellationToken::new();

        // start all of the listeners

        // playlist service
        let playlist_cancel_token = cancel_token.clone();
        let playlist_handle = tokio::spawn(async move {
            run_service(playlist_service, r_playlist, playlist_cancel_token).await
        });

        // process service
        let process_cancel_token = cancel_token.clone();
        let process_handle = tokio::spawn(async move {
            run_service(process_service, r_process, process_cancel_token).await
        });

        // audio service
        let audio_cancel_token = cancel_token.clone();
        let audio_handle =
            tokio::spawn(
                async move { run_service(audio_service, r_audio, audio_cancel_token).await },
            );

        // start the (blocking) gui loop
        let _ = gui_service.start_loop(cancel_token.clone(), t_playlist, r_bus);

        // send signal to shutdown program
        cancel_token.cancel();

        // after the gui loop has finished, ensure all the async loops are cleaned up
        let _ = runtime.block_on(future::join_all(vec![
            playlist_handle,
            process_handle,
            audio_handle,
        ]));
    }
}
