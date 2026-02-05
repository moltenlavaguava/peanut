use iced::{
    Theme,
    widget::{TextInput, text_input},
};

use crate::service::gui::styling::{AppTheme, TextInputStyle};

fn build_text_input<'a, Message: Clone>(
    placeholder: &str,
    value: &str,
    style: TextInputStyle,
) -> TextInput<'a, Message> {
    text_input(placeholder, value).style(style.style())
}

pub fn default_text_input<'a, Message: Clone>(
    placeholder: &str,
    value: &str,
    theme: &Theme,
) -> TextInput<'a, Message> {
    let style = theme.stylesheet().default_text_input();
    build_text_input(placeholder, value, style)
}
