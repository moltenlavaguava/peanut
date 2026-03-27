use iced::{
    Element, Theme,
    widget::{Button, button, text},
};

use crate::service::gui::styling::{AppTheme, ButtonStyle};

fn style_button<'a, Message>(btn: Button<'a, Message>, style: ButtonStyle) -> Button<'a, Message> {
    let btn = btn.padding(style.padding);
    btn.style(style.style())
}

pub fn default_button<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    theme: &Theme,
) -> Button<'a, Message> {
    let ss = theme.stylesheet().default_button();
    style_button(button(content), ss)
}
pub fn secondary_button<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    theme: &Theme,
) -> Button<'a, Message> {
    let ss = theme.stylesheet().secondary_button();
    style_button(button(content), ss)
}
pub fn secondary_text_button<'a, Message>(
    txt: impl text::IntoFragment<'a>,
    theme: &Theme,
) -> Button<'a, Message> {
    let tw = super::text::default_text(txt, theme, false, true);
    secondary_button(tw, theme)
}

pub fn default_text_button<'a, Message>(
    txt: impl text::IntoFragment<'a>,
    theme: &Theme,
) -> Button<'a, Message> {
    let tw = super::text::default_text(txt, theme, false, true);
    default_button(tw, theme)
}
pub fn track_button<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    theme: &Theme,
) -> Button<'a, Message> {
    let ss = theme.stylesheet().track_button();
    style_button(button(content), ss)
}
pub fn invisible_button<'a, Message>(
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
    invisible_button(content, &theme).padding(theme.stylesheet().default_button().padding)
}
