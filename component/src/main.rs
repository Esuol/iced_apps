use iced::widget::container;
use iced::{Element, Length};

mod numeric_input;
pub use numeric_input::numeric_input;

#[derive(Default)]
struct Component {
    value: Option<u32>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    NumericInputChanged(Option<u32>),
}

impl Component {
    fn update(&mut self, message: Message) {
        match message {
            Message::NumericInputChanged(value) => {
                self.value = value;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        container(numeric_input(self.value, Message::NumericInputChanged))
            .padding(20)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

fn main() -> iced::Result {
    iced::run("Component - Iced", Component::update, Component::view)
}
