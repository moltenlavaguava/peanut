use iced::{
    Alignment, Element, Length, Theme,
    widget::{Stack, column, container, opaque, space},
};

use crate::service::gui::{
    enums::Message,
    widgets::{
        modal::Modal,
        notification::{Notification, NotificationRenderData},
    },
};

pub fn build_page<'a>(
    content: impl Into<Element<'a, Message>>,
    notifications: Option<Vec<Notification<'a>>>,
    notification_render_data: NotificationRenderData,
    modal: Option<Modal>,
    theme: &Theme,
) -> Element<'a, Message> {
    let mut stack = Stack::new().push(content);
    if let Some(notifications) = notifications {
        // build the notification list
        let n_list = notifications.into_iter().map(|n| n.view(theme));
        let n_col = column(n_list)
            .spacing(notification_render_data.spacing)
            .align_x(Alignment::End);
        let c = container(n_col)
            .align_right(Length::Fill)
            .align_bottom(Length::Fill)
            .padding(notification_render_data.side_padding);
        stack = stack.push(c);
    }
    if let Some(modal) = modal {
        // add the modal on top of everything else
        let bg = opaque(space());
        let content = modal.build();
        stack = stack.push(bg).push(content);
    }
    stack.into()
}
