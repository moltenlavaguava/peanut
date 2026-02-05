use iced::{
    Element, Padding, Theme,
    widget::{Container, container},
};

use crate::service::gui::styling::{AppTheme, ContainerStyle};

fn build_container<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    style: ContainerStyle,
) -> Container<'a, Message> {
    container(content).style(style.style())
}
pub fn menu_content<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    theme: &Theme,
) -> Container<'a, Message> {
    let ss = theme.stylesheet().menu_content();
    build_container(content, ss)
}
pub fn main_content<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    theme: &Theme,
) -> Container<'a, Message> {
    let ss = theme.stylesheet().main_content();
    build_container(content, ss)
}
pub fn home_menu_widget_container<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    theme: &Theme,
) -> Container<'a, Message> {
    let ss = theme.stylesheet().home_widget_container();
    build_container(content, ss).padding(Padding::new(8.0))
}
