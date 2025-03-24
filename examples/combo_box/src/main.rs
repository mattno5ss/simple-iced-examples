use iced::widget::{column, combo_box, container, text};
use iced::{Element, Size, Theme};

pub fn main() -> iced::Result {
    let settings = iced::window::Settings {
        size: Size::new(500.0, 500.0),
        min_size: Some(Size::new(500.0, 500.0)),
        ..Default::default()
    };

    iced::application("combo box example", App::update, App::view)
        .window(settings)
        .theme(App::theme)
        .run()
}

struct App {
    options: combo_box::State<String>,
    selected_option: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    Selected(String),
}

impl App {
    fn new() -> Self {
        let items = vec![
            "item 1".to_string(),
            "item 2".to_string(),
            "item 3".to_string(),
        ];
        Self {
            options: combo_box::State::new(items),
            selected_option: None,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Selected(item) => {
                self.selected_option = Some(item);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let combo_box = combo_box(
            &self.options,
            "Choose an item...",
            self.selected_option.as_ref(),
            Message::Selected,
        )
        .width(300);

        let selected_item =
            text("Selected item: ".to_string() + self.selected_option.as_deref().unwrap_or("None"));

        container(column![selected_item, combo_box].spacing(20))
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::GruvboxDark
    }
}
