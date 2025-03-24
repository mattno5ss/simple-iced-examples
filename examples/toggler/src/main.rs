use iced::widget::{container, row, text, toggler};
use iced::{Center, Element, Size, Theme};

pub fn main() -> iced::Result {
    let settings = iced::window::Settings {
        size: Size::new(500.0, 500.0),
        min_size: Some(Size::new(500.0, 500.0)),
        ..Default::default()
    };

    iced::application("toggler example", App::update, App::view)
        .window(settings)
        .theme(App::theme)
        .run()
}

#[derive(Default)]
struct App {
    is_toggled: bool,
}

#[derive(Debug, Clone)]
enum Message {
    TogglerToggled(bool),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::TogglerToggled(is_toggled) => {
                self.is_toggled = is_toggled;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let toggler = toggler(self.is_toggled).label("Toggle me!").on_toggle(Message::TogglerToggled);
        let text = text(if self.is_toggled { "Toggled!" } else { "Not toggled" });
        
        container(row![toggler, text].align_y(Center).spacing(20))
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::GruvboxDark
    }
}
