use iced::widget::{container, row, text};
use iced::{Element, Size, Theme};

pub fn main() -> iced::Result {
    let settings = iced::window::Settings {
        size: Size::new(500.0, 500.0),
        min_size: Some(Size::new(500.0, 500.0)),
        ..Default::default()
    };

    iced::application("checkbox example", App::update, App::view)
        .window(settings)
        .theme(App::theme)
        .run()
}

#[derive(Default)]
struct App {}

#[derive(Debug, Clone)]
enum Message {}

impl App {
    fn update(&mut self, message: Message) {}

    fn view(&self) -> Element<Message> {
        let text = text("Just some text.");

        container(row![text].spacing(20)).padding(20).into()
    }

    fn theme(&self) -> Theme {
        Theme::GruvboxDark
    }
}
