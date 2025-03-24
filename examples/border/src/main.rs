use iced::widget::{column, container, row, text};
use iced::{Border, Color, Element, Size, Theme, border::Radius};

fn main() -> iced::Result {
    // Settings for the initial app window
    let settings = iced::window::Settings {
        size: Size::new(500.0, 500.0),
        min_size: Some(Size::new(500.0, 500.0)),
        ..Default::default()
    };

    iced::application("border example", App::update, App::view)
        .window(settings)
        .theme(App::theme)
        .run()
}

#[derive(Default)]
// App struct to hold the app state, but we don't need any state in this example
struct App {
    // placeholder
}

#[derive(Debug, Clone)]
// Message enum to handle events in the app, but we don't have any events in this example
enum Message {
    // placeholder
}

impl App {
    // Function to update the app state based on the message received, but we don't have any state to update in this example
    fn update(&mut self, _message: Message) {
        // placeholder
    }

    // Function to define the UI of the app
    // The first border uses the current theme's palette colors, and the second border uses a custom color
    //
    // Remember the border is defined on containers by passing a closure with the container::Style
    // struct to the style method
    fn view(&self) -> Element<Message> {
        let border_1 = container(text("border color using theme.palette()").size(20))
            .padding(10)
            // To use the theme's palette colors, we need to pass the theme to the Style struct in a closure
            .style(|theme: &Theme| container::Style {
                border: Border {
                    color: theme.palette().primary,
                    width: 1.0,
                    radius: Radius::new(3.0),
                },
                ..container::Style::default()
            });

        let border_2 = container(text("border color using custom Color").size(20))
            .padding(10)
            // To use a custom color, we can use the Color struct without passing anything to the closure
            .style(|_| container::Style {
                border: Border {
                    color: Color::from_rgb8(150, 150, 250),
                    width: 2.0,
                    radius: Radius::new(20.0),
                },
                ..container::Style::default()
            });

        container(
            column![
                row![border_1], 
                row![border_2]
            ]  
            .spacing(20),
        )
        .padding(20)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::GruvboxDark
    }
}
