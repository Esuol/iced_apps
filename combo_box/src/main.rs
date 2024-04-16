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
    Other,
}

impl Language {
    const ALL: [Language; 8] = [
        Language::Danish,
        Language::English,
        Language::French,
        Language::German,
        Language::Italian,
        Language::Portuguese,
        Language::Spanish,
        Language::Other,
    ];

    fn hello(&self) -> &str {
        match self {
            Language::Danish => "Halloy!",
            Language::English => "Hello!",
            Language::French => "Salut!",
            Language::German => "Hallo!",
            Language::Italian => "Ciao!",
            Language::Portuguese => "Olá!",
            Language::Spanish => "¡Hola!",
            Language::Other => "... hello?",
        }
    }
}

fn main() {
    println!("Hello, world!");
}
