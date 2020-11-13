use crate::gui::style;
use crate::Message;
use iced::{
    button, text_input, Button, Column, Container, Element, HorizontalAlignment, Length, Row, Text,
    TextInput, VerticalAlignment,
};
use sha1::Sha1;

#[derive(Clone, Debug)]
pub enum Update {
    PasswordStringChanged(String),
    PasswordStringSubmitted,
    OutputCleared,
}

#[derive(Default)]
pub struct Week4 {
    password_input_field: text_input::State,
    password_submit_button: button::State,
    password: String,
    password_hash: String,
    clear_button: button::State,
}

impl Week4 {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn update(&mut self, msg: Update) {
        match msg {
            Update::PasswordStringChanged(s) => {
                self.password = s;
            }
            Update::PasswordStringSubmitted => {
                let mut hasher = Sha1::new();
                hasher.update(self.password.as_bytes());
                let result = hasher.digest();
                self.password_hash = result.to_string();
            }
            Update::OutputCleared => {
                self.password = "".into();
                self.password_hash = "".into();
            }
        }
    }

    pub fn view(&mut self) -> Container<Message> {
        let password_input_field = TextInput::new(
            &mut self.password_input_field,
            "Enter password:",
            &self.password,
            |s| Message::Week4Update(Update::PasswordStringChanged(s)),
        )
        .padding(10)
        .size(20)
        .style(style::Theme::default());
        let password_submit_button =
            Button::new(&mut self.password_submit_button, Text::new("Submit"))
                .on_press(Message::Week4Update(Update::PasswordStringSubmitted))
                .padding(10)
                .style(style::Theme::default());

        let password_row = Row::new()
            .spacing(10)
            .push(password_input_field)
            .push(password_submit_button);

        let output = Text::new(&self.password_hash)
            .width(Length::FillPortion(100))
            .vertical_alignment(VerticalAlignment::Center);

        let clear = Button::new(
            &mut self.clear_button,
            Text::new("Clear")
                .vertical_alignment(VerticalAlignment::Center)
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .padding(10)
        .on_press(Message::Week4Update(Update::OutputCleared))
        .style(style::Theme::default())
        .width(Length::FillPortion(16));

        let output_row = Row::new().spacing(10).push(output).push(clear);

        let content: Element<_> = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(password_row)
            .push(output_row)
            .into();

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(style::Theme::default())
            .into()
    }
}
