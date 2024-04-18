use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::mouse;
use iced::{Border, Color, Element, Length, Rectangle, Size};

pub struct Circle {
    radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Circle { radius }
    }
}

pub fn circle(radius: f32) -> Circle {
    Circle::new(radius)
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Circle
where
  Renderer: renderer::Renderer,
{
  fn size(&self) -> Size<Length> {
    Size {
      width: Length::Shrink,
      height: Length::Shrink
    }
  }
}
