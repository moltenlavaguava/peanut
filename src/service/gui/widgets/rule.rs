use iced::{
    Pixels, Theme,
    widget::{Rule, rule},
};

use crate::service::gui::styling::AppTheme;

pub fn default_horizontal_rule<'a>(height: impl Into<Pixels>, theme: &Theme) -> Rule<'a> {
    rule::horizontal(height).style(theme.stylesheet().default_rule(2.0).style())
}

#[allow(dead_code)]
pub fn secondary_horizontal_rule<'a>(height: impl Into<Pixels>, theme: &Theme) -> Rule<'a> {
    rule::horizontal(height).style(theme.stylesheet().secondary_rule(6.0).style())
}
