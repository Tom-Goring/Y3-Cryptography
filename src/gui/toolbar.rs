use iced::{button, Align, Button, Container, Length, Row, Text};

use super::style;
use crate::Message;

// TODO: Make this receive a list of pages and auto construct it?

#[derive(Default)]
pub struct ToolBar {
    week1_button: button::State,
    week2_button: button::State,
    week3_button: button::State,
    week4_button: button::State,
    week5_button: button::State,
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
                )
                .push(
                    Button::new(&mut self.week3_button, Text::new("Week 3"))
                        .on_press(Message::ViewWeek3)
                        .style(style::Theme::default()),
                )
                .push(
                    Button::new(&mut self.week4_button, Text::new("Week 4"))
                        .on_press(Message::ViewWeek4)
                        .style(style::Theme::default()),
                )
                .push(
                    Button::new(&mut self.week5_button, Text::new("Week 5"))
                        .on_press(Message::ViewWeek5)
                        .style(style::Theme::default()),
                ),
        )
        .align_y(Align::Start)
        .width(Length::Fill)
        .padding(4)
    }
}
