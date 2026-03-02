use iced::{Element, Padding, Theme};

use crate::service::gui::{enums::Message, widgets::container::notification_container};

pub mod download;
pub mod init;
pub mod playing;

#[derive(Debug, Clone, Copy)]
pub struct NotificationRenderData {
    pub side_padding: Padding,
    pub spacing: f32,
}

const NOTIFICATION_PROGRESS_BAR_HEIGHT: f32 = 10.0;
// Note: this constant is used in `notification.rs`
const NOTIFICATION_WIDTH: f32 = 400.0;

pub struct Notification<'a> {
    content: Element<'a, Message>,
}
impl<'a> Notification<'a> {
    pub fn view(self, theme: &Theme) -> Element<'a, Message> {
        notification_container(self.content, theme)
            .width(NOTIFICATION_WIDTH)
            .into()
    }
}
