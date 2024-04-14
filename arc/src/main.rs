use std::{f32::consts::PI, time::Instant};

use iced::mouse;
use iced::widget::canvas::{self, stroke, Cache, Canvas, Geometry, Path, Stroke};
use iced::{Element, Length, Point, Rectangle, Renderer, Subscription, Theme};
struct Arc {
    start: Instant,
    cache: Cache,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
}

impl Arc {
    fn update(&mut self, _: Message) {
        self.cache.clear();
    }

    fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(10)).map(|_| Message::Tick)
    }
}

pub fn main() {
    println!("Hello, world!");
}
