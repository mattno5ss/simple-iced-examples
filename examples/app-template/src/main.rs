use iced::time::{Duration, every};
use iced::widget::{column, container, row};
use iced::{Element, Size, Subscription, Theme};

const VERSION: &str = "v1.0";

fn main() -> iced::Result {
    // Settings for the initial app window
    let settings = iced::window::Settings {
        size: Size::new(500.0, 500.0),
        min_size: Some(Size::new(500.0, 500.0)),
        ..Default::default()
    };

    iced::application(App::title, App::update, App::view)
        .subscription(App::subscription)
        .window(settings)
        .theme(App::theme)
        .run()
}

// App struct to hold the app state
struct App {
    running: bool,
}
impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

#[derive(Debug, Clone)]
// Message enum to handle events in the app
enum Message {
    Tick,
}

impl App {
    // Function to set title of the app
    fn title(&self) -> String {
        format!("Template {VERSION}")
    }
    // Function to create a new instance of the app
    fn new() -> Self {
        // Initial values for the app state
        Self {
            // Set initial values for app state
            running: false,
        }
    }
    // Function to update the app state based on the message received
    fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                // Update app state based on subscription tick
            }
        }
    }
    // Function to define the subscription of the app
    fn subscription(&self) -> Subscription<Message> {
        match self.running {
            true => every(Duration::from_secs(1)).map(|_| Message::Tick),
            false => Subscription::none(),
        }
    }
    // Function to define the UI of the app
    fn view(&self) -> Element<'_, Message> {
        container(column![row![], row![]].spacing(20))
            .padding(20)
            .into()
    }
    // Function to define the theme of the app
    fn theme(&self) -> Theme {
        Theme::CatppuccinFrappe
    }
}
