use iced::{
    Alignment, Pixels,
    widget::{Text, text},
};

pub fn one_line_text(
    t: impl Into<String>,
    height: Pixels,
    font_size: f32,
    center_text_y: bool,
) -> Text<'static> {
    let mut txt = text(t.into())
        .height(height)
        .size(Pixels(font_size))
        .wrapping(text::Wrapping::Glyph);
    if center_text_y {
        txt = txt.align_y(Alignment::Center)
    }
    txt
}
