use iced::widget::{column, container, text, text_input};
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
struct App {
    input: String,
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::TextInputChanged(input) => {
                self.input = input;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let text_input =
            text_input("Type something...", &self.input).on_input(Message::TextInputChanged);
        let text = text(format!("You entered: {}", &self.input));

        container(column![text_input, text].spacing(20))
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::GruvboxDark
    }
}
