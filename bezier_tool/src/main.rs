use iced::widget::{button, column, text};
use iced::{Alignment, Element, Length};

mod bezier;

#[derive(Default)]
struct Example {
    bezier: bezier::State,
    curves: Vec<bezier::Curve>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    AddCurve(bezier::Curve),
    Clear,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::AddCurve(curve) => {
                self.curves.push(curve);
                self.bezier.request_redraw();
            }
            Message::Clear => {
                self.bezier = bezier::State::default();
                self.curves.clear();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text("Bezeier Tool Example").width(Length::Shrink).size(50),
            self.bezier.view(&self.curves).map(Message::AddCurve),
            button("Clear")
                .style(button::danger)
                .on_press(Message::Clear),
        ]
        .padding(20)
        .spacing(20)
        .align_items(Alignment::Center)
        .into()
    }
}

pub fn main() -> iced::Result {
    iced::program("Bezier Tool - Iced", Example::update, Example::view)
        .antialiasing(true)
        .run()
}
