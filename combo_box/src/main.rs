use iced::widget::{column, combo_box, container, scrollable, text, vertical_space};
use iced::{Alignment, Element, Length};

struct Example {
    languages: combo_box::State<Language>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    Danish,
    #[default]
    English,
    French,
    German,
    Italian,
    Portuguese,
    Spanish,
    Other
}

fn main() {
    println!("Hello, world!");
}
