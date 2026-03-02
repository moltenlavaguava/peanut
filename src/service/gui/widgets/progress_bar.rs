use std::ops::RangeInclusive;

use iced::{
    Theme,
    widget::{ProgressBar, progress_bar},
};

use crate::service::gui::styling::{AppTheme, ProgressBarStyle};

fn build_progress_bar<'a>(
    range: RangeInclusive<f32>,
    value: f32,
    style: ProgressBarStyle,
) -> ProgressBar<'a> {
    progress_bar(range, value).style(style.style())
}

pub fn default_progress_bar<'a>(
    range: RangeInclusive<f32>,
    value: f32,
    theme: &Theme,
) -> ProgressBar<'a> {
    let style = theme.stylesheet().default_progress_bar();
    build_progress_bar(range, value, style)
}
