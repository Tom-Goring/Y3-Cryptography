use iced::{
    button, text_input, Align, Button, Column, Element, Row, Sandbox, Settings, Text, TextInput,
};

pub fn main() {
    Counter::run(Settings::default());
}

#[derive(Default)]
struct Counter {
    isbn: String,
    isbn_input: text_input::State,
    isbn_submit: button::State,
    isbn_message: String,

    credit_card_no: String,
    credit_card_input: text_input::State,
    credit_submit: button::State,
    credit_message: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ISBNInputChanged(String),
    CreditInputChanged(String),
    ISBNSubmitted,
    CreditNumSubmitted,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CreditInputChanged(value) => {
                self.credit_card_no = value;
            }
            Message::ISBNInputChanged(value) => {
                self.isbn = value;
            }
            Message::CreditNumSubmitted => {
                self.credit_message = String::from("Credit Valid!");
                // TODO: Add credit verification
            }
            Message::ISBNSubmitted => {
                self.isbn_message = String::from("ISBN Valid!");
                // TODO: Add isbn verification
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Row::new()
                    .padding(20)
                    .push(TextInput::new(
                        &mut self.isbn_input,
                        "",
                        &*self.isbn,
                        Message::ISBNInputChanged,
                    ))
                    .push(Button::new(&mut self.isbn_submit, Text::new("Submit")).on_press(Message::ISBNSubmitted)),
            )
            .push(
                Text::new(&self.isbn_message)
            )
            .push(
                Row::new()
                    .padding(20)
                    .push(TextInput::new(
                        &mut self.credit_card_input,
                        "",
                        &*self.credit_card_no,
                        Message::CreditInputChanged,
                    ))
                    .push(Button::new(&mut self.credit_submit, Text::new("Submit")).on_press(Message::CreditNumSubmitted)),
            ).push(Text::new(&self.credit_message))
            .into()
    }
}
