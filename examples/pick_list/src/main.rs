use iced::widget::{container, pick_list, row};
use iced::{Element, Size, Theme};

pub fn main() -> iced::Result {
    let settings = iced::window::Settings {
        size: Size::new(500.0, 500.0),
        min_size: Some(Size::new(500.0, 500.0)),
        ..Default::default()
    };

    iced::application("pick list example", App::update, App::view)
        .window(settings)
        .theme(App::theme)
        .run()
}

struct App {
    options: Vec<String>,
    selection: Option<String>,
}
impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    PickListSelected(String),
}

impl App {
    fn new() -> Self {
        let options = vec![
            "Option 1".to_string(),
            "Option 2".to_string(),
            "Option 3".to_string(),
        ];
        Self {
            options,
            selection: None,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PickListSelected(selected) => {
                self.selection = Some(selected);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let pick_list = pick_list(
            self.options.clone(),
            self.selection.clone(),
            Message::PickListSelected,
        )
        .placeholder("Pick an option...");

        container(row![pick_list].spacing(20)).padding(20).into()
    }

    fn theme(&self) -> Theme {
        Theme::GruvboxDark
    }
}
