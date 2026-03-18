use iced::{Alignment, Element, Length, Task, widget::text};

use crate::service::gui::widgets::modal::{AbstractModal, Modal, ModalFillAmount};

#[derive(Debug, Clone)]
pub enum NewPlaylistModalMsg {}

#[derive(Debug, Clone)]
pub struct NewPlaylistModal {}
impl AbstractModal for NewPlaylistModal {
    type ModalMsg = NewPlaylistModalMsg;

    fn view(&self, theme: &iced::Theme) -> Element<'_, Self::ModalMsg> {
        text("Hello world!").into()
    }

    fn update(&mut self, message: Self::ModalMsg) -> iced::Task<Self::ModalMsg> {
        Task::none()
    }
}
impl Into<Modal> for NewPlaylistModal {
    fn into(self) -> Modal {
        Modal::NewPlaylist(self)
    }
}
impl NewPlaylistModal {
    pub fn new() -> Self {
        Self {}
    }
}
