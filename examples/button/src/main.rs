use iced::widget::{button, container, row, text};
use iced::{Center, Element, Size, Theme};

pub fn main() -> iced::Result {
    let settings = iced::window::Settings {
        size: Size::new(500.0, 500.0),
        min_size: Some(Size::new(500.0, 500.0)),
        ..Default::default()
    };

    iced::application("button example", App::update, App::view)
        .window(settings)
        .theme(App::theme)
        .run()
}

#[derive(Default)]
struct App {
    pressed_count: u32,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {
                self.pressed_count += 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let button = button("Press Me!").on_press(Message::ButtonPressed);
        let text = text(format!("Button pressed {} times!", self.pressed_count));
        
        container(row![button, text].align_y(Center).spacing(20))
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::GruvboxDark
    }
}
