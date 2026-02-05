use iced::{
    Alignment, Pixels, Theme,
    widget::{Text, text},
};

use crate::service::gui::{
    icons::{ICON_FONT, IconChar},
    styling::{AppTheme, TextStyle},
};

fn build_text<'a>(txt: impl text::IntoFragment<'a>, style: TextStyle) -> Text<'a> {
    let mut t = text(txt).size(Pixels(style.text_size)).font(style.font);

    // apply other styling
    if !style.wrap {
        t = t.height(style.text_size).wrapping(text::Wrapping::Glyph);
    }
    if style.center_y {
        t = t.align_y(Alignment::Center);
    }

    // apply main style
    t.style(style.style())
}

pub fn default<'a>(
    txt: impl text::IntoFragment<'a>,
    theme: &Theme,
    wrap: bool,
    center_y: bool,
) -> Text<'a> {
    let style = theme.stylesheet().default_text(wrap, center_y);
    build_text(txt, style)
}
pub fn secondary<'a>(
    txt: impl text::IntoFragment<'a>,
    theme: &Theme,
    wrap: bool,
    center_y: bool,
) -> Text<'a> {
    let style = theme.stylesheet().secondary_text(wrap, center_y);
    build_text(txt, style)
}
pub fn title<'a>(
    txt: impl text::IntoFragment<'a>,
    theme: &Theme,
    wrap: bool,
    center_y: bool,
) -> Text<'a> {
    let style = theme.stylesheet().title_text(wrap, center_y);
    build_text(txt, style)
}
pub fn left_menu_bold<'a>(
    txt: impl text::IntoFragment<'a>,
    theme: &Theme,
    wrap: bool,
    center_y: bool,
) -> Text<'a> {
    let style = theme.stylesheet().left_menu_bold_text(wrap, center_y);
    build_text(txt, style)
}
pub fn left_menu_sub<'a>(
    txt: impl text::IntoFragment<'a>,
    theme: &Theme,
    wrap: bool,
    center_y: bool,
) -> Text<'a> {
    let style = theme.stylesheet().left_menu_sub_text(wrap, center_y);
    build_text(txt, style)
}
pub fn icon<'a>(icon: IconChar, mut style: TextStyle) -> Text<'a> {
    style.font = ICON_FONT;
    build_text(icon.0, style)
}
