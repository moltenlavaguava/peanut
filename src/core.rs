use crate::service::{file::FileService, gui::GuiService};
use crate::util::service::run_service;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

/// Handles starting and shutdown of the program.
pub struct CoreService;

impl CoreService {
    pub fn spawn() -> () {
        // Handler creation
        let (t_file, r_file) = mpsc::channel(100);
        let (t_bus, r_bus) = mpsc::channel(100);

        // Service creation
        let file_service = FileService::new(t_bus.clone());
        let gui_service = GuiService::new();

        // Runtime creation
        let runtime = Runtime::new().expect("Failed to create tokio runtime");
        let _guard = runtime.enter(); // must assign variable to guard i believe

        let cancel_token = CancellationToken::new();

        // start all of the listeners
        let file_cancel_token = cancel_token.clone();
        let file_handle =
            tokio::spawn(async move { run_service(file_service, r_file, file_cancel_token).await });

        // start the (blocking) gui loop
        let _ = gui_service.start_loop(cancel_token.clone(), t_file.clone(), r_bus);

        // send signal to shutdown program
        cancel_token.cancel();

        // after the gui loop has finished, ensure all the async loops are cleaned up
        let _ = runtime.block_on(file_handle);
    }
}
