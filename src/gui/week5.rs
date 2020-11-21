use crate::gui::style;
use crate::Message;
use iced::{
    button, text_input, Button, Color, Column, Container, Element, HorizontalAlignment, Length,
    Row, Text, TextInput, VerticalAlignment,
};

use crypto;

#[derive(Clone, Debug)]
pub enum Update {
    HashStringChanged(String),
    HashStringSubmitted,
    OutputCleared,
}

#[derive(Default)]
pub struct Week5 {
    hash_input_field: text_input::State,
    hash_submit_button: button::State,
    hash: String,
    original_password: String,
    clear_button: button::State,
}

impl Week5 {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn update(&mut self, msg: Update) {
        match msg {
            Update::HashStringChanged(s) => {
                self.hash = s;
            }
            Update::HashStringSubmitted => {
                self.original_password = crypto::sha_cracker::crack(&self.hash, 6).unwrap();
            }
            Update::OutputCleared => {
                self.original_password = "".into();
                self.hash = "".into();
            }
        }
    }

    pub fn view(&mut self) -> Container<Message> {
        let warning = Text::new("WARNING: THIS WILL UTILISE 100% OF EVERY CORE ON YOUR MACHINE UNTIL THE PASSWORD IS CRACKED. THIS COULD TAKE A VERY LONG TIME ON A WEEK MACHINE. THINK CAREFULLY.").color(Color::from_rgb(1.0, 0.0, 0.0));

        let hash_input_field = TextInput::new(
            &mut self.hash_input_field,
            "Enter password:",
            &self.hash,
            |s| Message::Week5Update(Update::HashStringChanged(s)),
        )
        .padding(10)
        .size(20)
        .style(style::Theme::default());

        let hash_submit_button = Button::new(&mut self.hash_submit_button, Text::new("Submit"))
            .on_press(Message::Week5Update(Update::HashStringSubmitted))
            .padding(10)
            .style(style::Theme::default());

        let hash_row = Row::new()
            .spacing(10)
            .push(hash_input_field)
            .push(hash_submit_button);

        let output = Text::new(&self.original_password)
            .width(Length::FillPortion(100))
            .vertical_alignment(VerticalAlignment::Center);

        let clear = Button::new(
            &mut self.clear_button,
            Text::new("Clear")
                .vertical_alignment(VerticalAlignment::Center)
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .padding(10)
        .on_press(Message::Week5Update(Update::OutputCleared))
        .style(style::Theme::default())
        .width(Length::FillPortion(16));

        let output_row = Row::new().spacing(10).push(output).push(clear);

        let content: Element<_> = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(warning)
            .push(hash_row)
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
