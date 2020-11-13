use iced::{Align, Column, Container, Element, Length, Sandbox, Settings};

mod gui;

use gui::Theme;
use gui::ToolBar;
use gui::Week1;
use gui::Week2;

use crate::gui::week2;
use crate::gui::week3::Week3;
use crate::gui::{week1, week3};

#[derive(Debug, Clone)]
pub enum Message {
    ViewWeek1,
    ViewWeek2,
    ViewWeek3,
    Week1Update(week1::Update),
    Week2Update(week2::Update),
    Week3Update(week3::Update),
}

enum Page {
    Week1,
    Week2,
    Week3,
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
    week3: Week3,
    theme: Theme,
}

impl Sandbox for CryptographyGUI {
    type Message = Message;

    fn new() -> Self {
        Self {
            toolbar: ToolBar::new(),
            week1: Week1::new(),
            week2: Week2::new(),
            week3: Week3::new(),
            current_page: Page::default(),
            theme: Theme::default(),
        }
    }

    fn title(&self) -> String {
        String::from("Cryptography Y3 - Tom Goring")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ViewWeek1 => self.current_page = Page::Week1,
            Message::ViewWeek2 => self.current_page = Page::Week2,
            Message::ViewWeek3 => self.current_page = Page::Week3,
            Message::Week1Update(change) => self.week1.update(change),
            Message::Week2Update(change) => self.week2.update(change),
            Message::Week3Update(change) => self.week3.update(change),
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
                Page::Week3 => self.week3.view(),
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

fn main() {
    CryptographyGUI::run(Settings::default());
}
