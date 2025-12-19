use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

#[async_trait::async_trait]
pub trait ServiceLogic<T>: Send + 'static
where
    T: Send + 'static,
{
    async fn on_start(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    async fn handle_message(&mut self, msg: T);
    fn name(&self) -> &'static str;
}

pub async fn run_service<S, T>(mut service: S, mut rx: mpsc::Receiver<T>, token: CancellationToken)
where
    S: ServiceLogic<T>,
    T: Send + 'static,
{
    let name = service.name();

    println!("Initializing {}...", name);
    if let Err(e) = service.on_start().await {
        eprintln!("Error: {} failed to start: {:?}", name, e);
        return;
    }
    println!("{name} started");
    loop {
        tokio::select! {
            _ = token.cancelled() => break,
            Some(msg) = rx.recv() => {
                service.handle_message(msg).await;
            }
        }
    }
    println!("{} stopped.", name);
}
