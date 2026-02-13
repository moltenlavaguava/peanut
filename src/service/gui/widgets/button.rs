use iced::{
    Element, Theme,
    widget::{Button, button, text},
};

use crate::service::gui::styling::{AppTheme, ButtonStyle};

fn style_button<'a, Message>(btn: Button<'a, Message>, style: ButtonStyle) -> Button<'a, Message> {
    let btn = btn.padding(style.padding);
    btn.style(style.style())
}

pub fn default<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    theme: &Theme,
) -> Button<'a, Message> {
    let ss = theme.stylesheet().default_button();
    style_button(button(content), ss)
}
pub fn default_text<'a, Message>(
    txt: impl text::IntoFragment<'a>,
    theme: &Theme,
) -> Button<'a, Message> {
    let tw = super::text::default(txt, theme, false, true);
    default(tw, theme)
}
pub fn track_button<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    theme: &Theme,
) -> Button<'a, Message> {
    let ss = theme.stylesheet().track_button();
    style_button(button(content), ss)
}
pub fn invisible<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    theme: &Theme,
) -> Button<'a, Message> {
    let style = theme.stylesheet().invisible_button();
    style_button(button(content), style)
}
pub fn invisible_button_padded<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    theme: &Theme,
) -> Button<'a, Message> {
    invisible(content, &theme).padding(theme.stylesheet().default_button().padding)
}
