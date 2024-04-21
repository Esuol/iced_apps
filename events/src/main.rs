use iced::alignment;
use iced::event::{self, Event};
use iced::widget::{button, checkbox, container, text, Column};
use iced::window;
use iced::{Alignment, Command, Element, Length, Subscription};

#[derive(Debug, Default)]
struct Events {
    last: Vec<Event>,
    enabled: bool,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    Toggled(bool),
    Exit,
}

fn main() {
    println!("Hello, world!");
}
