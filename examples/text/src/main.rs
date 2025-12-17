use iced::widget::{container, row, text};
use iced::{Element, Size, Theme};

pub fn main() -> iced::Result {
    let settings = iced::window::Settings {
        size: Size::new(500.0, 500.0),
        min_size: Some(Size::new(500.0, 500.0)),
        ..Default::default()
    };

    iced::application(App::default, App::update, App::view)
        .window(settings)
        .title("Text Example")
        .theme(App::theme)
        .run()
}

#[derive(Default)]
struct App {}

#[derive(Debug, Clone)]
enum Message {}

impl App {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<'_, Message> {
        let text = text("Just some text.");

        container(row![text].spacing(20)).padding(20).into()
    }

    fn theme(&self) -> Theme {
        Theme::GruvboxDark
    }
}
