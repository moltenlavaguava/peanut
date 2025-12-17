use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

#[async_trait::async_trait]
pub trait ServiceLogic<T>: Send + 'static
where
    T: Send + 'static,
{
    async fn handle_message(&mut self, msg: T);
    fn name(&self) -> &'static str;
}

pub async fn run_service<S, T>(mut service: S, mut rx: mpsc::Receiver<T>, token: CancellationToken)
where
    S: ServiceLogic<T>,
    T: Send + 'static,
{
    loop {
        tokio::select! {
            _ = token.cancelled() => break,
            Some(msg) = rx.recv() => {
                service.handle_message(msg).await;
            }
        }
    }
    println!("{} stopped.", service.name());
}
