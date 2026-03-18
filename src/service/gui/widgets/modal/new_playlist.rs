use iced::{
    Element, Task,
    widget::{button, text},
};

use crate::service::gui::{
    enums::Message,
    widgets::{
        button::default_text_button,
        modal::{AbstractModal, AbstractModalMessage, Modal, column},
    },
};

#[derive(Debug, Clone)]
pub enum NewPlaylistModalMsg {
    LocalMessage,
}

#[derive(Debug, Clone)]
pub struct NewPlaylistModal {
    c: u64,
}
impl AbstractModal<Message> for NewPlaylistModal {
    type ModalMsg = NewPlaylistModalMsg;

    fn view(
        &self,
        theme: &iced::Theme,
    ) -> Element<'_, AbstractModalMessage<Self::ModalMsg, Message>> {
        column![
            Element::from(
                default_text_button(format!("Local message! ({})", self.c), theme)
                    .on_press(NewPlaylistModalMsg::LocalMessage)
            )
            .map(AbstractModalMessage::Local),
            Element::from(default_text_button("cloes :(", theme).on_press(Message::HideModal))
                .map(AbstractModalMessage::Global)
        ]
        .into()
    }

    fn update(&mut self, message: Self::ModalMsg) -> iced::Task<Self::ModalMsg> {
        match message {
            NewPlaylistModalMsg::LocalMessage => self.c += 1,
        }
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
        Self { c: 0 }
    }
}
