use iced::{
    button, text_input, Align, Button, Column, Container, Element, HorizontalAlignment, Length,
    Row, Text, TextInput, VerticalAlignment,
};

use super::style;
use crate::Message;

#[derive(Clone, Debug)]
pub enum Update {
    SixDigitInputSubmitted,
    SixDigitInputChanged(String),
    TenDigitInputSubmitted,
    TenDigitInputChanged(String),
    ClearOutput,
}

enum OutputState {
    Success,
    Normal,
    Error,
}

impl Default for OutputState {
    fn default() -> Self {
        OutputState::Normal
    }
}

#[derive(Default)]
pub struct Week2 {
    theme: style::Theme,

    input_field: text_input::State,
    input_value: String,
    submit: button::State,

    syndrome_input_field: text_input::State,
    syndrome_input: String,
    submit_syndrome: button::State,

    output: String,
    output_state: OutputState,
    clear_button: button::State,
}

impl Week2 {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn update(&mut self, msg: Update) {
        match msg {
            Update::SixDigitInputChanged(s) => {
                self.input_value = s;
            }
            Update::SixDigitInputSubmitted => {
                let weights = [
                    &crypto::hamming::D7[..],
                    &crypto::hamming::D8[..],
                    &crypto::hamming::D9[..],
                    &crypto::hamming::D10[..],
                ];
                let output = crypto::hamming::calculate_digits(&weights, &self.input_value, 6);
                self.handle_output(output);
            }
            Update::TenDigitInputSubmitted => {
                let weights = [
                    &crypto::hamming::S1[..],
                    &crypto::hamming::S2[..],
                    &crypto::hamming::S3[..],
                    &crypto::hamming::S4[..],
                ];
                let output = crypto::hamming::calculate_digits(&weights, &self.syndrome_input, 10);
                self.handle_output(output);
            }
            Update::TenDigitInputChanged(s) => {
                self.syndrome_input = s;
            }
            Update::ClearOutput => {
                self.output = "".into();
                self.output_state = OutputState::Normal;
                self.syndrome_input = "".into();
                self.input_value = "".into();
            }
        }
    }

    fn handle_output(&mut self, result: Result<String, String>) {
        match result {
            Ok(output) => {
                self.output = output;
                self.output_state = OutputState::Success;
            }
            Err(error) => {
                self.output = error;
                self.output_state = OutputState::Error;
            }
        }
    }

    pub fn view(&mut self) -> Container<Message> {
        let input_field = TextInput::new(
            &mut self.input_field,
            "Enter number...",
            &self.input_value,
            |s| Message::Week2Update(Update::SixDigitInputChanged(s)),
        )
        .padding(10)
        .size(20)
        .style(self.theme);

        let submit_button = Button::new(&mut self.submit, Text::new("Submit"))
            .on_press(Message::Week2Update(Update::SixDigitInputSubmitted))
            .padding(10)
            .style(self.theme);

        let input_row = Row::new().spacing(10).push(input_field).push(submit_button);

        let syndrome_field = TextInput::new(
            &mut self.syndrome_input_field,
            "Enter number...",
            &self.syndrome_input,
            |s| Message::Week2Update(Update::TenDigitInputChanged(s)),
        )
        .padding(10)
        .size(20)
        .style(self.theme);

        let syndrome_submit_button = Button::new(&mut self.submit_syndrome, Text::new("Submit"))
            .on_press(Message::Week2Update(Update::TenDigitInputSubmitted))
            .padding(10)
            .style(self.theme);

        let syndrome_row = Row::new()
            .spacing(10)
            .push(syndrome_field)
            .push(syndrome_submit_button);

        let color = match self.output_state {
            OutputState::Success => [0.0, 1.0, 0.0],
            OutputState::Normal => [1.0, 1.0, 1.0],
            OutputState::Error => [1.0, 0.0, 0.0],
        };

        let output = Text::new(&self.output)
            .width(Length::FillPortion(100))
            .color(color)
            .vertical_alignment(VerticalAlignment::Center);

        let clear_button = Button::new(
            &mut self.clear_button,
            Text::new("Clear")
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center),
        )
        .padding(10)
        .on_press(Message::Week2Update(Update::ClearOutput))
        .style(self.theme)
        .width(Length::FillPortion(16));

        let output_row = Row::new()
            .spacing(10)
            .push(output)
            .push(clear_button)
            .align_items(Align::Center);

        let content: Element<_> = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(input_row)
            .push(syndrome_row)
            .push(output_row)
            .into();

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(style::Theme::default())
    }
}
