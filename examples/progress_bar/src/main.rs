use iced::time::{Duration, every};
use iced::widget::{button, column, container, progress_bar, row, text};
use iced::{Element, Size, Subscription, Theme};

pub fn main() -> iced::Result {
    let settings = iced::window::Settings {
        size: Size::new(500.0, 500.0),
        min_size: Some(Size::new(500.0, 500.0)),
        ..Default::default()
    };

    iced::application(App::default, App::update, App::view)
        .subscription(App::subscription)
        .window(settings)
        .title("Progress Bar Example")
        .theme(App::theme)
        .run()
}

#[derive(Default)]
struct App {
    progress: f32,
    progress_state: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Start,
    Stop,
    Update,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Start => {
                self.progress_state = true;
            }
            Message::Stop => {
                self.progress_state = false;
            }
            Message::Update => {
                if self.progress == 100.0 {
                    self.progress = 0.0
                } else {
                    self.progress += 2.0
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let progress_bar = progress_bar(0.0..=100.0, self.progress);
        let button = match self.progress_state {
            true => button("Stop").on_press(Message::Stop),
            false => button("Start").on_press(Message::Start),
        };
        let percent = text(format!("{}%", self.progress));

        container(column![row![progress_bar, button].spacing(20), row![percent]].spacing(20))
            .padding(20)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.progress_state {
            true => every(Duration::from_millis(100)).map(|_| Message::Update),
            false => Subscription::none(),
        }
    }

    fn theme(&self) -> Theme {
        Theme::GruvboxDark
    }
}
