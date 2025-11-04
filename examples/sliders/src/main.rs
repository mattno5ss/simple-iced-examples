use iced::widget::{column, container, row, slider, text, vertical_slider};
use iced::{Center, Element, Size, Theme};

pub fn main() -> iced::Result {
    let settings = iced::window::Settings {
        size: Size::new(500.0, 500.0),
        min_size: Some(Size::new(500.0, 500.0)),
        ..Default::default()
    };

    iced::application("sliders example", App::update, App::view)
        .window(settings)
        .theme(App::theme)
        .run()
}
// App state
#[derive(Default)]
struct App {
    h_slider_value: u8,
    v_slider_value: u8,
}

#[derive(Debug, Clone)]
enum Message {
    HorizontalSliderChanged(u8),
    VerticalSliderChanged(u8),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::HorizontalSliderChanged(value) => {
                self.h_slider_value = value;
            }
            Message::VerticalSliderChanged(value) => {
                self.v_slider_value = value;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let h_slider = slider(
            0..=255,
            self.h_slider_value,
            Message::HorizontalSliderChanged,
        );
        let v_slider =
            vertical_slider(0..=255, self.v_slider_value, Message::VerticalSliderChanged);
        let h_text = text(format!("{}", self.h_slider_value)).size(20);
        let v_text = text(format!("{}", self.v_slider_value)).size(20);

        container(
            column![
                row![column![v_text, v_slider].align_x(Center).spacing(20)],
                row![h_slider, h_text].align_y(Center).spacing(20)
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
