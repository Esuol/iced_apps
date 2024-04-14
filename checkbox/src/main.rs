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

pub fn main() {
    print!("hello")
}
