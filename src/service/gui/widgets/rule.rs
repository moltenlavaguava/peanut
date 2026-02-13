use iced::{
    Element, Length, Padding, Pixels, Theme,
    widget::{Rule, column, container, rule},
};

use crate::service::gui::styling::{AppTheme, RuleStyle};

pub fn default_horizontal_rule<'a>(height: impl Into<Pixels>, theme: &Theme) -> Rule<'a> {
    rule::horizontal(height).style(theme.stylesheet().default_rule(2.0).style())
}

#[allow(dead_code)]
pub fn secondary_horizontal_rule<'a>(height: impl Into<Pixels>, theme: &Theme) -> Rule<'a> {
    rule::horizontal(height).style(theme.stylesheet().secondary_rule(6.0).style())
}
pub fn in_between_rule<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    height: impl Into<Pixels>,
    style: RuleStyle,
    horizontal_padding: f32,
    // used for determining if this specific element needs a rule at the end
    current: usize,
    total: usize,
) -> Element<'a, Message>
where
    Message: 'a,
{
    if current + 1 < total {
        // add a rule to separate
        let r = rule::horizontal(height).style(style.style());
        column![
            container(content).width(Length::Fill),
            container(r).padding(Padding::ZERO.horizontal(horizontal_padding))
        ]
        .into()
    } else {
        content.into()
    }
}
