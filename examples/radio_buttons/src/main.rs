use iced::widget::{column, container, radio, row, text};
use iced::{Element, Size, Theme};

pub fn main() -> iced::Result {
    let settings = iced::window::Settings {
        size: Size::new(500.0, 500.0),
        min_size: Some(Size::new(500.0, 500.0)),
        ..Default::default()
    };

    iced::application("radio buttons example", App::update, App::view)
        .window(settings)
        .theme(App::theme)
        .run()
}
// App state
#[derive(Default)]
struct App {
    radio_choice: Option<RadioChoice>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RadioChoice {
    A,
    B,
    C,
}

#[derive(Debug, Clone)]
enum Message {
    RadioChoiceChanged(RadioChoice),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::RadioChoiceChanged(choice) => {
                self.radio_choice = Some(choice);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let radio_a = radio(
            "A",
            RadioChoice::A,
            self.radio_choice,
            Message::RadioChoiceChanged,
        );
        let radio_b = radio(
            "B",
            RadioChoice::B,
            self.radio_choice,
            Message::RadioChoiceChanged,
        );
        let radio_c = radio(
            "C",
            RadioChoice::C,
            self.radio_choice,
            Message::RadioChoiceChanged,
        );
        let text = match self.radio_choice {
            Some(RadioChoice::A) => text("You selected: A"),
            Some(RadioChoice::B) => text("You selected: B"),
            Some(RadioChoice::C) => text("You selected: C"),
            None => text("Select a radio button."),
        };
        container(column![row![text], row![radio_a, radio_b, radio_c].spacing(20)].spacing(20))
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::GruvboxDark
    }
}
