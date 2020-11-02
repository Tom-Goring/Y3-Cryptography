use iced::{
    button, text_input, Button, Column, Container, HorizontalAlignment, Length, Row, Text,
    TextInput,
};

use crate::Message;

use super::style;

use crypto;
use crypto::credit::CreditCardVerificationError;
use crypto::isbn::ISBNVerificationError;

#[derive(Clone, Debug)]
pub enum Update {
    IsbnInputChange(String),
    CreditInputChange(String),
    IsbnSubmit,
    CreditSubmit,
    ClearOutput,
}

use Update::*;

#[derive(Default)]
pub struct Week1 {
    pub isbn_input: text_input::State,
    pub credit_input: text_input::State,
    pub isbn_value: String,
    pub credit_value: String,
    pub isbn_button: button::State,
    pub credit_button: button::State,
    pub clear_button: button::State,
    pub output: String,
    pub output_err: bool,
    pub output_success: bool,
    pub theme: style::Theme,
}

impl Week1 {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn update(&mut self, msg: Update) {
        match msg {
            IsbnInputChange(s) => {
                self.isbn_value = s;
            }
            CreditInputChange(s) => {
                self.credit_value = s;
            }
            IsbnSubmit => {
                let result = crypto::isbn::verify_isbn(&self.isbn_value);
                match result {
                    Err(ISBNVerificationError::NonValidISBN) => {
                        self.output = String::from("Inputted ISBN is not valid!");
                        self.output_err = true;
                    }
                    Err(ISBNVerificationError::InvalidDigitsFound) => {
                        self.output =
                            String::from("A valid ISBN must contain only integers and dashes.");
                        self.output_err = true;
                    }
                    Err(ISBNVerificationError::InvalidDigitCount) => {
                        self.output = String::from("A valid ISBN must contain exactly 10 digits.");
                        self.output_err = true;
                    }
                    Ok(..) => {
                        self.output = String::from("Valid ISBN!");
                        self.output_err = false;
                        self.output_success = true;
                    }
                }
            }
            CreditSubmit => match crypto::credit::verify_credit_card(&self.credit_value) {
                Err(CreditCardVerificationError::InvalidLength) => {
                    self.output = "A valid credit card number must have 16 digits".into();
                    self.output_err = true;
                }
                Err(CreditCardVerificationError::InvalidDigitsFound) => {
                    self.output = "A valid credit card must have exactly 16 digits".into();
                    self.output_err = true;
                }
                Err(CreditCardVerificationError::InvalidCreditCard) => {
                    self.output = "Inputted credit card is not valid.".into();
                    self.output_err = true;
                }
                Ok(..) => {
                    self.output = "Valid credit card number!".into();
                    self.output_err = false;
                    self.output_success = true;
                }
            },
            ClearOutput => {
                self.output = String::from("");
                self.isbn_value = String::from("");
                self.credit_value = String::from("");
            }
        }
    }

    pub fn view(&mut self) -> Container<Message> {
        let isbn_input = TextInput::new(
            &mut self.isbn_input,
            "Enter ISBN...",
            &self.isbn_value,
            |s| Message::Week1Update(IsbnInputChange(s)),
        )
        .padding(10)
        .size(20)
        .style(self.theme);

        let credit_input = TextInput::new(
            &mut self.credit_input,
            "Enter credit card number...",
            &self.credit_value,
            |s| Message::Week1Update(CreditInputChange(s)),
        )
        .padding(10)
        .size(20)
        .style(self.theme);

        let isbn_submit = Button::new(&mut self.isbn_button, Text::new("Submit"))
            .on_press(Message::Week1Update(IsbnSubmit))
            .padding(10)
            .style(self.theme);

        let credit_submit = Button::new(&mut self.credit_button, Text::new("Submit"))
            .on_press(Message::Week1Update(CreditSubmit))
            .padding(10)
            .style(self.theme);

        let color;
        if self.output_err {
            color = [1.0, 0.0, 0.0];
        } else if self.output_success {
            color = [0.0, 1.0, 0.0];
        } else {
            color = [1.0, 1.0, 1.0];
        }

        let output = Text::new(&self.output)
            .color(color)
            .width(Length::FillPortion(100));

        let clear_button = Button::new(
            &mut self.clear_button,
            Text::new("Clear").horizontal_alignment(HorizontalAlignment::Center),
        )
        .padding(10)
        .on_press(Message::Week1Update(ClearOutput))
        .style(self.theme)
        .width(Length::FillPortion(16));

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(Row::new().spacing(10).push(isbn_input).push(isbn_submit))
            .push(
                Row::new()
                    .spacing(10)
                    .push(credit_input)
                    .push(credit_submit),
            )
            .push(Row::new().spacing(10).push(output).push(clear_button));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.theme)
            .into()
    }
}
