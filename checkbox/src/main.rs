use std::default;

use iced::widget::{checkbox, column, container, row, text};
use iced::{Element, Font, Length};

const ICON_FONT: Font = Font::with_name("icons");

#[derive(Default)]
struct Example {
    default: bool,
    styled: bool,
    custom: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DefaultToggled(bool),
    StyledToggled(bool),
    CustomToggled(bool),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::DefaultToggled(default) => {
                self.default = default;
            }
            Message::StyledToggled(styled) => {
                self.styled = styled;
            }
            Message::CustomToggled(custom) => {
                self.custom = custom;
            }
        }
    }
}

pub fn main() {
    print!("hello")
}
