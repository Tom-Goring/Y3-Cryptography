use iced::{button, text_input, Button, Column, Container, Length, Row, Text, TextInput};

use super::style;
use crate::Message;

#[derive(Clone, Debug)]
pub enum Update {
    InputSubmitted,
    InputChanged(String),
}

#[derive(Default)]
pub struct Week2 {
    theme: style::Theme,

    input_field: text_input::State,
    input_value: String,
    submit: button::State,
    output: String,
}

impl Week2 {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn update(&mut self, msg: Update) {
        match msg {
            Update::InputChanged(s) => {
                self.input_value = s;
            }
            Update::InputSubmitted => unimplemented!(),
        }
    }

    pub fn view(&mut self) -> Container<Message> {
        let input_field = TextInput::new(
            &mut self.input_field,
            "Enter number...",
            &self.input_value,
            |s| Message::Week2Update(Update::InputChanged(s)),
        )
        .padding(10)
        .size(20)
        .style(self.theme);

        let submit_button = Button::new(&mut self.submit, Text::new("Submit"))
            .on_press(Message::Week2Update(Update::InputSubmitted))
            .padding(10)
            .style(self.theme);

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(Row::new().spacing(10).push(input_field).push(submit_button));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(style::Theme::default())
            .into()
    }
}
