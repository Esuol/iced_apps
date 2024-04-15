use iced::mouse;
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::{Element, Length, Point, Rectangle, Renderer, Theme};

#[derive(Default)]
pub struct State {
    cache: canvas::Cache,
}

impl State {
    pub fn view<'a>(&'a self, curves: &'a [Curve]) -> Element<'a, Curve> {
        Canvas::new(Bezier {})
    }
}

struct Bezier<'a> {
    state: &'a State,
    curves: &'a [Curve],
}

impl<'a> canvas::Program<Curve> for Bezier<'a> {
    type State = Option<Pending>;
}

#[derive(Debug, Clone, Copy)]
pub struct Curve {
    from: Point,
    to: Point,
    control: Point,
}

impl Curve {
    fn draw_all(curves: &[Curve], frame: &mut Frame) {
        let curves = Path::new(|p| {
            for curve in curves {
                p.move_to(curve.from);
                p.quadratic_curve_to(curve.control, curve.to);
            }
        });

        frame.stroke(&curves, Stroke::default().with_width(2.0));
    }
}
