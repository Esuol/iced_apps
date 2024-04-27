use iced::keyboard;
use iced::mouse;
use iced::widget::{
    button, canvas, checkbox, column, container, horizontal_space, pick_list, row, scrollable, text,
};
use iced::{
    color, Alignment, Element, Font, Length, Point, Rectangle, Renderer, Subscription, Theme,
};


#[derive(Default, Debug)]
struct Layout {
    example: Example,
    explain: bool,
    theme: Theme
}

#[derive(Debug, Clone)]
enum Message {
    Next,
    Previous,
    ExplainToggked(bool),
    ThemeSelected(Theme),
}

impl Layout {
    
}

fn main() {
    println!("Hello, world!");
}
