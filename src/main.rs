use iced::{Align, Column, Container, Element, Length, Sandbox, Settings};

mod gui;

use gui::Theme;
use gui::ToolBar;
use gui::Week1;
use gui::Week2;

use crate::gui::week1;
use crate::gui::week2;

#[derive(Debug, Clone)]
pub enum Message {
    ViewWeek1,
    ViewWeek2,
    Week1Update(week1::Update),
    Week2Update(week2::Update),
}

enum Page {
    Week1,
    Week2,
}

impl Default for Page {
    fn default() -> Self {
        Page::Week1
    }
}

pub struct CryptographyGUI {
    current_page: Page,
    toolbar: ToolBar,
    week1: Week1,
    week2: Week2,
    theme: Theme,
}

impl Sandbox for CryptographyGUI {
    type Message = Message;

    fn new() -> Self {
        Self {
            toolbar: gui::ToolBar::new(),
            week1: gui::Week1::new(),
            week2: gui::Week2::new(),
            current_page: Page::default(),
            theme: gui::Theme::default(),
        }
    }

    fn title(&self) -> String {
        String::from("Cryptography Y3 - Tom Goring")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ViewWeek1 => self.current_page = Page::Week1,
            Message::ViewWeek2 => self.current_page = Page::Week2,
            Message::Week1Update(change) => self.week1.update(change),
            Message::Week2Update(change) => self.week2.update(change),
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = Column::new()
            .padding(8)
            .spacing(8)
            .align_items(Align::Center)
            .push(self.toolbar.view())
            .push(match &self.current_page {
                Page::Week1 => self.week1.view(),
                Page::Week2 => self.week2.view(),
            });

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .style(self.theme)
            .into()
    }
}

fn main() -> iced::Result {
    CryptographyGUI::run(Settings::default())
}
