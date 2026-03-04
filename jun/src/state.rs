use iced::widget;

use crate::{Language, Message};

pub struct Dependencies {}
pub struct App {
    pub deps: Dependencies,
    pub t: Language,
}

impl App {
    pub fn new(deps: Dependencies, t: Language) -> Self {
        Self { deps, t }
    }

    pub fn update(&mut self, _: Message) -> iced::Task<Message> {
        iced::Task::none()
    }

    pub fn title(&self) -> String {
        self.t.t_title().into()
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        widget::button("-").into()
    }
}
