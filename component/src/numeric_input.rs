use iced::alignment::{self, Alignment};
use iced::widget::{button, component, row, text, text_input, Component};
use iced::{Element, Length, Size};

pub struct NumericInput<Message> {
    value: Option<u32>,
    on_change: Box<dyn Fn(Option<u32>) -> Message>,
}

pub fn numeric_input<Message>(
    value: Option<u32>,
    on_change: impl Fn(Option<u32>) -> Message + 'static,
) -> NumericInput<Message> {
    NumericInput::new(value, on_change)
}
