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

    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (event::Status, Option<Curve>) {
        let Some(cursor_position) = cursor.position_in(bounds) else {
            return (event::Status::Ignored, None);
        };

        match event {
            Event::Mouse(mouse_event) => {
                let message = match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => match *state {
                        None => {
                            *state = Some(Pending::One {
                                from: cursor_position,
                            });

                            None
                        }
                        Some(Pending::One { from }) => {
                            *state = Some(Pending::Two {
                                from,
                                to: cursor_position,
                            });

                            None
                        }
                        Some(Pending::Two { from, to }) => {
                            *state = None;

                            Some(Curve {
                                from,
                                to,
                                control: cursor_position,
                            })
                        }
                    },
                    _ => None,
                };

                (event::Status::Captured, message)
            }
            _ => (event::Status::Ignored, None),
        }
    }
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

#[derive(Debug, Clone, Copy)]
enum Pending {
    One { from: Point },
    Two { from: Point, to: Point },
}

impl Pending {
    fn draw(&self, renderer: &Renderer, bounds: Rectangle, cursor: mouse::Cursor) -> Geometry {
        let mut frame = Frame::new(renderer, bounds.size());

        if let Some(cursor_position) = cursor.position_in(bounds) {
            match *self {
                Pending::One { from } => {
                    let line = Path::line(from, cursor_position);
                    frame.stroke(&line, Stroke::default().with_width(2.0));
                }
                Pending::Two { from, to } => {
                    let curve = Curve {
                        from,
                        to,
                        control: cursor_position,
                    };

                    Curve::draw_all(&[curve], &mut frame);
                }
            };
        }
        frame.into_geometry()
    }
}
