use std::ops::RangeInclusive;

use iced::{
    Theme,
    widget::{Slider, slider},
};

use crate::service::gui::styling::{AppTheme, SliderStyle};

fn build_slider<'a, T, Message>(
    range: RangeInclusive<T>,
    value: T,
    on_change: impl Fn(T) -> Message + 'a,
    style: SliderStyle,
) -> Slider<'a, T, Message>
where
    T: Copy + From<u8> + std::cmp::PartialOrd,
    Message: Clone,
{
    slider(range, value, on_change).style(style.style())
}

pub fn default_slider<'a, T, Message>(
    range: RangeInclusive<T>,
    value: T,
    on_change: impl Fn(T) -> Message + 'a,
    theme: &Theme,
) -> Slider<'a, T, Message>
where
    T: Copy + From<u8> + std::cmp::PartialOrd,
    Message: Clone,
{
    let style = theme.stylesheet().default_slider();
    build_slider(range, value, on_change, style)
}
