use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::mouse;
use iced::{Border, Color, Element, Length, Rectangle, Shadow, Size};

pub struct CustomQuad {
    size: f32,
    radius: [f32; 4],
    border_width: f32,
    shadow: Shadow,
}

impl CustomQuad {
    pub fn new(size: f32, radius: [f32; 4], border_width: f32, shadow: Shadow) -> Self {
        Self {
            size,
            radius,
            border_width,
            shadow,
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for CustomQuad
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(self.size, self.size))
    }

    fn draw(
        &self,
        _state: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
      
    }
}
