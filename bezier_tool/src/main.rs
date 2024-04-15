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

fn main() {
    println!("Hello, world!");
}
