use iced::widget::container;
use iced::{Element, Length};

mod numeric_input;

#[derive(Default)]
struct Component {
    value: Option<u32>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    NumericInputChanged(Option<u32>),
}

impl Component {
    
}

fn main() {
    println!("Hello, world!");
}
