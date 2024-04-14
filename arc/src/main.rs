use std::{f32::consts::PI, time::Instant};

use iced::mouse;
use iced::widget::canvas::{self, stroke, Cache, Canvas, Geometry, Path, Stroke};
use iced::{Element, Length, Point, Rectangle, Renderer, Subscription, Theme};

struct Arc {
    start: Instant,
    cache: Cache,
}


pub fn main() {
    println!("Hello, world!");
}
