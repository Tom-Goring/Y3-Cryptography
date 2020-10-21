use iced::{button, text_input, Button, Column, Container, Element, Length, Radio, Row, Sandbox, Settings, Text, TextInput, HorizontalAlignment};

use cryptolib;
use cryptolib::ISBNVerificationError;

pub fn main() {
    CryptoGUI::run(Settings::default())
}

#[derive(Default)]
struct CryptoGUI {
    theme: style::Theme,
    isbn_input: text_input::State,
    credit_input: text_input::State,
    isbn_value: String,
    credit_value: String,
    isbn_button: button::State,
    credit_button: button::State,
    clear_button: button::State,
    output: String,
    output_err: bool,
    output_success: bool,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(style::Theme),
    ISBNChanged(String),
    CreditChanged(String),
    ISBNButtonPressed,
    CreditButtonPressed,
    ClearButtonPressed
}

impl Sandbox for CryptoGUI {
    type Message = Message;

    fn new() -> Self {
        CryptoGUI::default()
    }

    fn title(&self) -> String {
        String::from("Styling - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => self.theme = theme,
            Message::ISBNChanged(value) => self.isbn_value = value,
            Message::CreditChanged(value) => self.credit_value = value,
            Message::ISBNButtonPressed => {
                let result = cryptolib::verify_isbn(&self.isbn_value);
                    match result {
                        Err(cryptolib::ISBNVerificationError::InvalidDigitCount) => {
                            self.output = String::from("A valid ISBN must have 10 digits");
                            self.output_err = true;
                        },
                        Err(cryptolib::ISBNVerificationError::InvalidDigitsFound) => {
                            self.output = String::from("A valid ISBN must only contain integers and dashes");
                            self.output_err = true;
                        },
                        Err(cryptolib::ISBNVerificationError::NonValidISBN) => {
                            self.output = String::from("Invalid ISBN detected");
                            self.output_err = true;
                        },
                        Ok(..) => {
                            self.output = String::from("Valid ISBN!");
                            self.output_err = false;
                            self.output_success = true;
                        }
                    }
            },
            Message::CreditButtonPressed => (),
            Message::ClearButtonPressed => {
                self.output = String::from("");
                self.output_success = false;
                self.output_err = false;
                self.isbn_value = String::from("");
            },
        }
    }

    fn view(&mut self) -> Element<Message> {
        let choose_theme = style::Theme::ALL.iter().fold(
            Column::new().spacing(10).push(Text::new("Choose a theme:")),
            |column, theme| {
                column.push(
                    Radio::new(
                        *theme,
                        &format!("{:?}", theme),
                        Some(self.theme),
                        Message::ThemeChanged,
                    )
                        .style(self.theme),
                )
            },
        );

        let isbn_input = TextInput::new(
            &mut self.isbn_input,
            "Type something...",
            &self.isbn_value,
            Message::ISBNChanged,
        )
            .padding(10)
            .size(20)
            .style(self.theme);

        let isbn_button = Button::new(&mut self.isbn_button, Text::new("Submit"))
            .padding(10)
            .on_press(Message::ISBNButtonPressed)
            .style(self.theme);

        let credit_input = TextInput::new(
            &mut self.credit_input,
            "Type something...",
            &self.credit_value,
            Message::CreditChanged,
        )
            .padding(10)
            .size(20)
            .style(self.theme);

        let credit_button = Button::new(&mut self.credit_button, Text::new("Submit"))
            .padding(10)
            .on_press(Message::CreditButtonPressed)
            .style(self.theme);

        let color;
        if self.output_err {
            color = [1.0, 0.0, 0.0];
        } else if self.output_success {
            color = [0.0, 1.0, 0.0];
        } else {
            color = [0.0, 0.0, 0.0];
        }

        let text_field = Text::new(&self.output).color(color).width(Length::FillPortion(100));
        let clear_button = Button::new(&mut self.clear_button, Text::new("Clear").horizontal_alignment(HorizontalAlignment::Center)).padding(10).on_press(Message::ClearButtonPressed).style(self.theme).width(Length::FillPortion(16));

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(choose_theme)
            .push(Row::new().spacing(10).push(isbn_input).push(isbn_button))
            .push(Row::new().spacing(10).push(credit_input).push(credit_button))
            .push(Row::new().spacing(10).push(text_field).push(clear_button));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.theme)
            .into()
    }
}

mod style {
    use iced::{
        button, checkbox, container, progress_bar, radio, scrollable, slider,
        text_input,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Theme {
        Light,
        Dark,
    }

    impl Theme {
        pub const ALL: [Theme; 2] = [Theme::Light, Theme::Dark];
    }

    impl Default for Theme {
        fn default() -> Theme {
            Theme::Dark
        }
    }

    impl From<Theme> for Box<dyn container::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::Container.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn radio::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::Radio.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn text_input::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::TextInput.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn button::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => light::Button.into(),
                Theme::Dark => dark::Button.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn scrollable::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::Scrollable.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn slider::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::Slider.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn progress_bar::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::ProgressBar.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn checkbox::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::Checkbox.into(),
            }
        }
    }

    mod light {
        use iced::{button, Background, Color, Vector};

        pub struct Button;

        impl button::StyleSheet for Button {
            fn active(&self) -> button::Style {
                button::Style {
                    background: Some(Background::Color(Color::from_rgb(
                        0.11, 0.42, 0.87,
                    ))),
                    border_radius: 12,
                    shadow_offset: Vector::new(1.0, 1.0),
                    text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                    ..button::Style::default()
                }
            }

            fn hovered(&self) -> button::Style {
                button::Style {
                    text_color: Color::WHITE,
                    shadow_offset: Vector::new(1.0, 2.0),
                    ..self.active()
                }
            }
        }
    }

    mod dark {
        use iced::{
            button, checkbox, container, progress_bar, radio, scrollable,
            slider, text_input, Background, Color,
        };

        const SURFACE: Color = Color::from_rgb(
            0x40 as f32 / 255.0,
            0x44 as f32 / 255.0,
            0x4B as f32 / 255.0,
        );

        const ACCENT: Color = Color::from_rgb(
            0x6F as f32 / 255.0,
            0xFF as f32 / 255.0,
            0xE9 as f32 / 255.0,
        );

        const ACTIVE: Color = Color::from_rgb(
            0x72 as f32 / 255.0,
            0x89 as f32 / 255.0,
            0xDA as f32 / 255.0,
        );

        const HOVERED: Color = Color::from_rgb(
            0x67 as f32 / 255.0,
            0x7B as f32 / 255.0,
            0xC4 as f32 / 255.0,
        );

        pub struct Container;

        impl container::StyleSheet for Container {
            fn style(&self) -> container::Style {
                container::Style {
                    background: Some(Background::Color(Color::from_rgb8(
                        0x36, 0x39, 0x3F,
                    ))),
                    text_color: Some(Color::WHITE),
                    ..container::Style::default()
                }
            }
        }

        pub struct Radio;

        impl radio::StyleSheet for Radio {
            fn active(&self) -> radio::Style {
                radio::Style {
                    background: Background::Color(SURFACE),
                    dot_color: ACTIVE,
                    border_width: 1,
                    border_color: ACTIVE,
                }
            }

            fn hovered(&self) -> radio::Style {
                radio::Style {
                    background: Background::Color(Color { a: 0.5, ..SURFACE }),
                    ..self.active()
                }
            }
        }

        pub struct TextInput;

        impl text_input::StyleSheet for TextInput {
            fn active(&self) -> text_input::Style {
                text_input::Style {
                    background: Background::Color(SURFACE),
                    border_radius: 2,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                }
            }

            fn focused(&self) -> text_input::Style {
                text_input::Style {
                    border_width: 1,
                    border_color: ACCENT,
                    ..self.active()
                }
            }

            fn placeholder_color(&self) -> Color {
                Color::from_rgb(0.4, 0.4, 0.4)
            }

            fn value_color(&self) -> Color {
                Color::WHITE
            }

            fn selection_color(&self) -> Color {
                Color::BLACK
            }

            fn hovered(&self) -> text_input::Style {
                text_input::Style {
                    border_width: 1,
                    border_color: Color { a: 0.3, ..ACCENT },
                    ..self.focused()
                }
            }
        }

        pub struct Button;

        impl button::StyleSheet for Button {
            fn active(&self) -> button::Style {
                button::Style {
                    background: Some(Background::Color(ACTIVE)),
                    border_radius: 3,
                    text_color: Color::WHITE,
                    ..button::Style::default()
                }
            }

            fn hovered(&self) -> button::Style {
                button::Style {
                    background: Some(Background::Color(HOVERED)),
                    text_color: Color::WHITE,
                    ..self.active()
                }
            }

            fn pressed(&self) -> button::Style {
                button::Style {
                    border_width: 1,
                    border_color: Color::WHITE,
                    ..self.hovered()
                }
            }
        }

        pub struct Scrollable;

        impl scrollable::StyleSheet for Scrollable {
            fn active(&self) -> scrollable::Scrollbar {
                scrollable::Scrollbar {
                    background: Some(Background::Color(SURFACE)),
                    border_radius: 2,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                    scroller: scrollable::Scroller {
                        color: ACTIVE,
                        border_radius: 2,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    },
                }
            }

            fn hovered(&self) -> scrollable::Scrollbar {
                let active = self.active();

                scrollable::Scrollbar {
                    background: Some(Background::Color(Color {
                        a: 0.5,
                        ..SURFACE
                    })),
                    scroller: scrollable::Scroller {
                        color: HOVERED,
                        ..active.scroller
                    },
                    ..active
                }
            }

            fn dragging(&self) -> scrollable::Scrollbar {
                let hovered = self.hovered();

                scrollable::Scrollbar {
                    scroller: scrollable::Scroller {
                        color: Color::from_rgb(0.85, 0.85, 0.85),
                        ..hovered.scroller
                    },
                    ..hovered
                }
            }
        }

        pub struct Slider;

        impl slider::StyleSheet for Slider {
            fn active(&self) -> slider::Style {
                slider::Style {
                    rail_colors: (ACTIVE, Color { a: 0.1, ..ACTIVE }),
                    handle: slider::Handle {
                        shape: slider::HandleShape::Circle { radius: 9 },
                        color: ACTIVE,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    },
                }
            }

            fn hovered(&self) -> slider::Style {
                let active = self.active();

                slider::Style {
                    handle: slider::Handle {
                        color: HOVERED,
                        ..active.handle
                    },
                    ..active
                }
            }

            fn dragging(&self) -> slider::Style {
                let active = self.active();

                slider::Style {
                    handle: slider::Handle {
                        color: Color::from_rgb(0.85, 0.85, 0.85),
                        ..active.handle
                    },
                    ..active
                }
            }
        }

        pub struct ProgressBar;

        impl progress_bar::StyleSheet for ProgressBar {
            fn style(&self) -> progress_bar::Style {
                progress_bar::Style {
                    background: Background::Color(SURFACE),
                    bar: Background::Color(ACTIVE),
                    border_radius: 10,
                }
            }
        }

        pub struct Checkbox;

        impl checkbox::StyleSheet for Checkbox {
            fn active(&self, is_checked: bool) -> checkbox::Style {
                checkbox::Style {
                    background: Background::Color(if is_checked {
                        ACTIVE
                    } else {
                        SURFACE
                    }),
                    checkmark_color: Color::WHITE,
                    border_radius: 2,
                    border_width: 1,
                    border_color: ACTIVE,
                }
            }

            fn hovered(&self, is_checked: bool) -> checkbox::Style {
                checkbox::Style {
                    background: Background::Color(Color {
                        a: 0.8,
                        ..if is_checked { ACTIVE } else { SURFACE }
                    }),
                    ..self.active(is_checked)
                }
            }
        }
    }
}