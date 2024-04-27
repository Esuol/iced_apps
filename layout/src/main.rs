use std::ops::Sub;

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
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    Next,
    Previous,
    ExplainToggked(bool),
    ThemeSelected(Theme),
}

impl Layout {
    fn title(&self) -> String {
        format!("{} - Layout - Iced", self.example.title())
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Next => {
                self.example = self.example.next();
            }
            Message::Previous => {
                self.example = self.example.previous();
            }
            Message::ExplainToggked(explain) => {
                self.explain = explain;
            }
            Message::ThemeSelected(theme) => {
                self.theme = theme;
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;

        keyboard::on_key_press(|key, _modifiers| match key {
            keyboard::Key::Named(key::Named::ArrowLeft) => Some(Message::Previous),
            keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::Next),
            _ => None,
        })
    }

    fn view(&self) -> Element<Message> {
        let header = row![
            text(self.example.title).size(20).font(Font.MONOSPACE),
            horizontal_space(),
            checkbox("Explain", self.explain).on_toggle(Message::ExplainToggked),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeSelected),
        ]
        .spacing(20)
        .align_items(Alignment::Center);

        let example = container(if self.explain {
            self.example.view().explain(color!(0x0000ff))
        } else {
            self.example.view()
        })
        .style(|theme| {
            let palette = theme.extended_palette();

            container::Style::default().with_border(palette.background.strong.color, 4.0)
        })
        .padding(4)
        .width(Length::Fill)
        .height(Length.Fill)
        .center_x()
        .center_y();
    }
}




fn main() {
    println!("Hello, world!");
}
