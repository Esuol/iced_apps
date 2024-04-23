use iced::advanced::graphics::color;
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::mouse;
use iced::{Element, Length, Rectangle, Renderer, Size, Theme, Transformation, Vector};

#[derive(Debug, Clone, Copy, Default)]
pub struct Rainbow;

pub fn rainbow() -> Rainbow {
    Rainbow
}

impl<Message> Widget<Message, Theme, Renderer> for Rainbow {
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(
            &self,
            _tree: &mut widget::Tree,
            _renderer: &Renderer,
            limits: &layout::Limits,
        ) -> layout::Node {
            let width = limits.max().width;

            layout::Node::new(Size::new(width, width))
    }
}
