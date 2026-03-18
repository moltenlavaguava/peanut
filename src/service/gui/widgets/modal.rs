use std::fmt::Debug;

use iced::{
    Element, Length, Padding, Task, Theme,
    widget::{column, container, mouse_area, opaque, space},
};

use crate::service::gui::widgets::{
    container::{default_modal_background_container, default_modal_container},
    modal::new_playlist::{NewPlaylistModal, NewPlaylistModalMsg},
};

pub mod new_playlist;

trait AbstractModal: Into<Modal> {
    type ModalMsg;
    // Only creates interior content. Does not make or handle modal mangement / container.
    fn view(&self, theme: &Theme) -> Element<'_, Self::ModalMsg>;
    fn update(&mut self, message: Self::ModalMsg) -> Task<Self::ModalMsg>;
    // Wrapper to create the modal container body (centered)
    fn build(&self, theme: &Theme) -> Element<'_, Self::ModalMsg> {
        // make main container opaque to make mouse clicks on modal itself not kill it
        let modal = container(opaque(
            default_modal_container(self.view(theme), theme).height(Length::Fill),
        ));
        // customize modal sizing
        let sized_modal: Element<Self::ModalMsg> = match self.fill_height() {
            ModalFillAmount::Shrink => modal.height(Length::Shrink).into(),
            ModalFillAmount::FillPercentage(p) => {
                // create spacers on top and bottom
                let sh = (200 - 2 * p) / 2;
                column![
                    space().height(Length::FillPortion(sh)),
                    modal.height(Length::FillPortion(2 * p)),
                    space().height(Length::FillPortion(sh))
                ]
                .into()
            }
            ModalFillAmount::Offset(o) => {
                // add padding to top and bottom
                container(modal).padding(Padding::ZERO.vertical(o)).into()
            }
            ModalFillAmount::Fill => modal.height(Length::Fill).into(),
        };
        default_modal_background_container(sized_modal, theme)
            .center(Length::Fill)
            .into()
    }
    fn fill_height(&self) -> ModalFillAmount {
        ModalFillAmount::Shrink
    }
}

#[derive(Debug, Clone)]
pub enum ModalMessage {
    NewPlaylist(NewPlaylistModalMsg),
    HideModal,
}

#[derive(Debug, Clone)]
enum ModalFillAmount {
    Shrink,
    Fill,
    /// Percentage of total width, out of 100
    FillPercentage(u16),
    Offset(u32),
}

#[derive(Debug, Clone)]
pub enum Modal {
    NewPlaylist(NewPlaylistModal),
}
impl Modal {
    pub fn view(&self, theme: &Theme) -> Element<'_, ModalMessage> {
        let main_modal_content = match self {
            Self::NewPlaylist(m) => m.build(theme).map(ModalMessage::NewPlaylist),
        };
        opaque(mouse_area(main_modal_content).on_press(ModalMessage::HideModal))
    }
    pub fn update(&mut self, msg: ModalMessage) -> Task<ModalMessage> {
        match (self, msg) {
            (Modal::NewPlaylist(w), ModalMessage::NewPlaylist(m)) => {
                w.update(m).map(ModalMessage::NewPlaylist)
            }
            _ => Task::none(),
        }
    }
}
