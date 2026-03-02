use iced::Element;
use iced::widget::text;

pub struct Modal {}
impl Modal {
    pub fn build<'a, Message>(self) -> Element<'a, Message> {
        text("g").into()
    }
}
