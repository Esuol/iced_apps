use iced::gradient;
use iced::widget::{checkbox, column, container, horizontal_space, row, slider, text};
use iced::Program;
use iced::{Alignment, Color, Element, Length, Radians, Theme};

#[derive(Debug, Clone, Copy)]
struct Gradient {
    start: Color,
    end: Color,
    angle: Radians,
    transparent: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    StartColorChanged(Color),
    EndColorChanged(Color),
    AngleChanged(Radians),
    TransparentToggled(bool),
}

impl Gradient {
    fn new() -> Self {
        Self {
            start: Color::BLACK,
            end: Color::new(0.0, 0.0, 1.0, 1.0),
            angle: Radians(0.0),
            transparent: false,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
