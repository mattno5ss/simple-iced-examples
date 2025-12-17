use iced::widget::{checkbox, container, row, text};
use iced::{Element, Size, Theme};

pub fn main() -> iced::Result {
    let settings = iced::window::Settings {
        size: Size::new(500.0, 500.0),
        min_size: Some(Size::new(500.0, 500.0)),
        ..Default::default()
    };

    iced::application(App::default, App::update, App::view)
        .window(settings)
        .title("Checkbox Example")
        .theme(App::theme)
        .run()
}

#[derive(Default)]
struct App {
    is_checked: bool,
}

#[derive(Debug, Clone)]
enum Message {
    CheckboxToggled(bool),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::CheckboxToggled(is_checked) => {
                self.is_checked = is_checked;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let checkbox = checkbox(self.is_checked)
            .label("Check Me!")
            .on_toggle(Message::CheckboxToggled);
        let text = match self.is_checked {
            true => text("Checked!"),
            false => text("Not checked!"),
        };
        container(row![checkbox, text].spacing(20))
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::GruvboxDark
    }
}
