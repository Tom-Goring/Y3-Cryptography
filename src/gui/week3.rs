use crate::gui::style;
use crate::Message;
use iced::{
    button, text_input, Align, Button, Column, Container, Element, HorizontalAlignment, Length,
    Row, Text, TextInput, VerticalAlignment,
};

enum CodingResult {
    Success,
    Normal,
    Error,
}

impl Default for CodingResult {
    fn default() -> Self {
        Self::Normal
    }
}

impl CodingResult {
    fn to_rgb(&self) -> [f32; 3] {
        match self {
            Self::Success => [0.0, 1.0, 0.0],
            Self::Normal => [1.0, 1.0, 1.0],
            Self::Error => [1.0, 0.0, 0.0],
        }
    }
}

#[derive(Clone, Debug)]
pub enum Update {
    EncodeValueChanged(String),
    EncodeValueSubmitted,
    DecodeValueChanged(String),
    DecodeValueSubmitted,
    OutputCleared,
}

#[derive(Default)]
pub struct Week3 {
    encode_input_field: text_input::State,
    value_to_encode: String,
    submit_value_to_encode: button::State,
    encoded_output: String,
    encoding_output_result: CodingResult,

    decode_input_field: text_input::State,
    value_to_decode: String,
    submit_value_to_decode: button::State,
    decoded_output: String,
    decoding_output_result: CodingResult,

    clear_button: button::State,
}

impl Week3 {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn update(&mut self, msg: Update) {
        match msg {
            Update::EncodeValueChanged(new_value) => self.value_to_encode = new_value,
            Update::DecodeValueChanged(new_value) => self.value_to_decode = new_value,
            Update::EncodeValueSubmitted => {
                let output = crypto::bch::encode_bch(&self.value_to_encode);
                match output {
                    Ok(output) => {
                        self.encoded_output = output;
                        self.encoding_output_result = CodingResult::Success
                    }
                    Err(err) => match err {
                        crypto::bch::CodingResult::EncodingError(err) => {
                            self.encoded_output = err;
                            self.encoding_output_result = CodingResult::Error;
                        }
                        _ => {
                            panic!("Encoding function somehow returned a decoding error, exiting!")
                        }
                    },
                }
            }
            Update::DecodeValueSubmitted => match crypto::bch::decode_bch(&self.value_to_decode) {
                Ok(output) => {
                    self.decoded_output = output;
                    self.decoding_output_result = CodingResult::Success
                }
                Err(err) => match err {
                    crypto::bch::CodingResult::InputError(error) => {
                        self.decoded_output = error;
                        self.decoding_output_result = CodingResult::Error
                    }
                    crypto::bch::CodingResult::SingleError(
                        corrected_output,
                        error_position,
                        error_magnitude,
                    ) => {
                        self.decoded_output = format!(
                            "Error detected in input value at position {} and of magnitude {}. \
                                Corrected output is {}",
                            error_position, error_magnitude, corrected_output
                        );
                        self.decoding_output_result = CodingResult::Error;
                    }
                    crypto::bch::CodingResult::DoubleError(
                        corrected_output,
                        position,
                        magnitude,
                    ) => {
                        self.decoded_output =
                                format!(
                                    "Two errors detected in given input: \
                                    One at position {} of magnitude {}, and one at position {} at magnitude {}. \
                                    Corrected output is {}", 
                                    position.0, magnitude.0, position.1, magnitude.1, corrected_output);
                        self.decoding_output_result = CodingResult::Error;
                    }
                    crypto::bch::CodingResult::TripleError(reason) => {
                        self.decoded_output = reason;
                        self.decoding_output_result = CodingResult::Error;
                    }
                    _ => panic!("Decoding function somehow returned an encoding error, exiting!"),
                },
            },
            Update::OutputCleared => {
                self.value_to_decode = "".into();
                self.value_to_encode = "".into();
                self.encoded_output = "".into();
                self.decoded_output = "".into();
                self.decoding_output_result = CodingResult::default();
                self.encoding_output_result = CodingResult::default();
            }
        }
    }

    pub fn view(&mut self) -> Container<Message> {
        let encode_input_field = TextInput::new(
            &mut self.encode_input_field,
            "Enter value to encode:",
            &self.value_to_encode,
            |s| Message::Week3Update(Update::EncodeValueChanged(s)),
        )
        .padding(10)
        .size(20)
        .style(style::Theme::default());

        let submit_value_to_encode =
            Button::new(&mut self.submit_value_to_encode, Text::new("Submit"))
                .on_press(Message::Week3Update(Update::EncodeValueSubmitted))
                .padding(10)
                .style(style::Theme::default());

        let encoding_row = Row::new()
            .spacing(10)
            .push(encode_input_field)
            .push(submit_value_to_encode);

        let encoded_output = Text::new(&self.encoded_output)
            .width(Length::Fill)
            .vertical_alignment(VerticalAlignment::Center)
            .color(self.encoding_output_result.to_rgb());

        let decode_input_field = TextInput::new(
            &mut self.decode_input_field,
            "Enter value to decode:",
            &self.value_to_decode,
            |s| Message::Week3Update(Update::DecodeValueChanged(s)),
        )
        .padding(10)
        .size(20)
        .style(style::Theme::default());

        let submit_value_to_decode =
            Button::new(&mut self.submit_value_to_decode, Text::new("Submit"))
                .on_press(Message::Week3Update(Update::DecodeValueSubmitted))
                .padding(10)
                .style(style::Theme::default());

        let decoding_row = Row::new()
            .spacing(10)
            .push(decode_input_field)
            .push(submit_value_to_decode);

        let decoded_output = Text::new(&self.decoded_output)
            .width(Length::Fill)
            .vertical_alignment(VerticalAlignment::Center)
            .color(self.decoding_output_result.to_rgb());

        let clear = Button::new(
            &mut self.clear_button,
            Text::new("Clear")
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center),
        )
        .padding(10)
        .on_press(Message::Week3Update(Update::OutputCleared))
        .style(style::Theme::default())
        .width(Length::Units(75));

        let clear_row = Column::new()
            .push(clear)
            .width(Length::Fill)
            .align_items(Align::End);

        let content: Element<_> = Column::new()
            .spacing(10)
            .padding(20)
            .max_width(600)
            .push(encoding_row)
            .push(encoded_output)
            .push(decoding_row)
            .push(decoded_output)
            .push(clear_row)
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
