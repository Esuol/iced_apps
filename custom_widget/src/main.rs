use iced::widget::{column, container, slider, text};
use iced::{Alignment, Element, Length};

mod circle;
use circle::circle;

struct Example {
    radius: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadiusChanged(f32),
}

impl Example {
    fn new() -> Self {
        Example { radius: 50.0 }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::RadiusChanged(radius) => {
                self.radius = radius;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            circle(self.radius),
            text(format!("Radius: {:.2}", self.radius)),
            slider(1.0..=100.0, self.radius, Message::RadiusChanged).step(0.01),
        ]
        .padding(20)
        .spacing(20)
        .max_width(500)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl Default for Example {
    fn default() -> Self {
        Self::new()
    }
}

fn main() -> iced::Result {
    iced::run("Custom widget", Example::update, Example::view)
}
