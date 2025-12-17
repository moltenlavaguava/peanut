use iced::futures::SinkExt;
use iced::futures::stream::{BoxStream, StreamExt};
use iced::{Subscription, stream};
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

// Gui recevier witchcraft
// todo: add docs for all of this

#[derive(Debug)]
pub struct ReceiverHandle<T> {
    id: u64,
    rx: Arc<Mutex<Option<mpsc::Receiver<T>>>>,
}

struct WatchContext<T, M> {
    id: u64,
    rx: Arc<Mutex<Option<mpsc::Receiver<T>>>>,
    on_data: Arc<dyn Fn(u64, T) -> M + Send + Sync>,
    on_finish: Arc<dyn Fn(u64) -> M + Send + Sync + 'static>,
}

impl<T, M> Hash for WatchContext<T, M> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T, M> PartialEq for WatchContext<T, M> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T, M> Eq for WatchContext<T, M> {}
impl<T> Clone for ReceiverHandle<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            rx: self.rx.clone(),
        }
    }
}

impl<T> ReceiverHandle<T>
where
    T: Send + 'static,
{
    pub fn new(id: u64, rx: mpsc::Receiver<T>) -> Self {
        Self {
            id,
            rx: Arc::new(Mutex::new(Some(rx))),
        }
    }

    pub fn watch<M>(
        &self,
        on_data: impl Fn(u64, T) -> M + Send + Sync + 'static,
        on_finish: impl Fn(u64) -> M + Send + Sync + 'static,
    ) -> Subscription<M>
    where
        M: 'static + Send,
    {
        let context = WatchContext {
            id: self.id,
            rx: self.rx.clone(),
            on_data: Arc::new(on_data),
            on_finish: Arc::new(on_finish),
        };
        Subscription::run_with(context, stream_builder::<T, M>)
    }
}

fn stream_builder<T, M>(ctx: &WatchContext<T, M>) -> BoxStream<'static, M>
where
    T: Send + 'static,
    M: 'static + Send,
{
    let id = ctx.id;
    let safe_rx = ctx.rx.clone();
    let on_data = ctx.on_data.clone();
    let on_finish = ctx.on_finish.clone();

    stream::channel::<M>(
        100,
        move |mut output: iced::futures::channel::mpsc::Sender<M>| async move {
            let mut rx = match safe_rx.lock().await.take() {
                Some(r) => r,
                None => std::future::pending().await,
            };

            while let Some(msg) = rx.recv().await {
                let _ = output.send(on_data(id, msg)).await;
            }

            let _ = output.send(on_finish(id)).await;
        },
    )
    .boxed()
}

// types and enums used within the whole program

#[derive(Debug, Clone)]
pub enum TaskResponse {}

#[derive(Debug, Clone)]
pub enum EventMessage {
    TaskResponse(TaskResponse),
    Count(usize),
}

pub type EventSender = mpsc::Sender<EventMessage>;
