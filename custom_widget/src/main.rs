use iced::widget::{column, container, slider, text};
use iced::{Alignment, Element, Length};

mod circle;
use circle::Circle;

struct Example {
    radius: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadiusChanged(f32),
}

impl Example {
    
}

fn main() {
    println!("Hello, world!");
}
