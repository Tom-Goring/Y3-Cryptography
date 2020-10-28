use iced::{button, Align, Button, Container, Length, Row, Text};

use super::style;
use crate::Message;

#[derive(Default)]
pub struct ToolBar {
    week1_button: button::State,
    week2_button: button::State,
}

impl ToolBar {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn view(&mut self) -> Container<Message> {
        Container::new(
            Row::new()
                .width(Length::Fill)
                .align_items(Align::Center)
                .spacing(24)
                .push(
                    Button::new(&mut self.week1_button, Text::new("Week 1"))
                        .on_press(Message::ViewWeek1)
                        .style(style::Theme::default()),
                )
                .push(
                    Button::new(&mut self.week2_button, Text::new("Week 2"))
                        .on_press(Message::ViewWeek2)
                        .style(style::Theme::default()),
                ),
        )
        .align_y(Align::Start)
        .width(Length::Fill)
        .padding(4)
    }
}
