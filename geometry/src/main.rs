mod rainbow;

use iced::widget::{column, container, scrollable};
use iced::{Element, Length};
use rainbow::rainbow;

pub fn main() -> iced::Result {
    iced::run("Custom 2D Geometry - Iced", |_: &mut _, _| {}, view)
}

fn view(_state: &()) -> Element<'_, ()> {
    let content = column![
        rainbow(),
        "In this example we draw a custom widget Rainbow, using \
                 the Mesh2D primitive. This primitive supplies a list of \
                 triangles, expressed as vertices and indices.",
        "Move your cursor over it, and see the center vertex \
                 follow you!",
        "Every Vertex2D defines its own color. You could use the \
                 Mesh2D primitive to render virtually any two-dimensional \
                 geometry for your widget.",
    ]
    .padding(20)
    .spacing(20)
    .max_width(500);

    let scrollable = scrollable(container(content).width(Length::Fill).center_x());

    container(scrollable)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .into()
}
