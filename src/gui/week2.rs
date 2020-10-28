use iced::{Container, Length, Text};

use super::style;
use crate::Message;

#[derive(Default)]
pub struct Week2 {}

impl Week2 {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn view(&mut self) -> Container<Message> {
        let content = Text::new("Placeholder for week 2");
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(style::Theme::default())
            .into()
    }
}
