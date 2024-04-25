use iced::widget::{checkbox, column, container, horizontal_space, row, slider, text};
use iced::Program;
use iced::{gradient, program};
use iced::{Alignment, Color, Element, Length, Radians, Theme};

#[derive(Debug, Clone, Copy)]
struct Gradient {
    start: Color,
    end: Color,
    angle: Radians,
    transparent: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    StartColorChanged(Color),
    EndColorChanged(Color),
    AngleChanged(Radians),
    TransparentToggled(bool),
}

impl Gradient {
    fn new() -> Self {
        Self {
            start: Color::BLACK,
            end: Color::new(0.0, 0.0, 1.0, 1.0),
            angle: Radians(0.0),
            transparent: false,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::StartChanged(color) => self.start = color,
            Message::EndChanged(color) => self.end = color,
            Message::AngleChanged(angle) => self.angle = angle,
            Message::TransparentToggled(transparent) => {
                self.transparent = transparent;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let Self {
            start,
            end,
            angle,
            transparent,
        } = *self;

        let gradient_box = container(horizontal_space())
            .style(move |_theme| {
                let gradient = gradient::Linear::new(angle)
                    .add_stop(0.0, start)
                    .add_stop(1.0, end);

                gradient.into()
            })
            .width(Length::Fill)
            .height(Length::Fill);

        let angle_picker = row![
            text("Angle").width(64),
            slider(Radians::RANGE, self.angle, Message::AngleChanged).step(0.01)
        ]
        .spacing(8)
        .padding(8)
        .align_items(Alignment::Center);

        let transparency_toggle = iced::widget::Container::new(
            checkbox("Transparent window", transparent).on_toggle(Message::TransparentToggled),
        )
        .padding(8);

        column![
            color_picker("Start", self.start).map(Message::StartChanged),
            color_picker("End", self.end).map(Message::EndChanged),
            angle_picker,
            transparency_toggle,
            gradient_box,
        ]
        .into()
    }

    fn styke(&self, theme: &Theme) -> program::Appearance {
        use program::DefaultStyle;

        if self.transparent {
            program::Appearance {
                background_color: Color::TRANSPARENT,
                text_color: theme.palette().text,
            }
        } else {
            Theme::default_style(theme)
        }
    }
}

impl Default for Gradient {
    fn default() -> Self {
        Self::new()
    }
}

fn color_picker(label: &str, color: Color) -> Element<'_, Color> {
    row![
        text(label).width(64),
        slider(0.0..=1.0, color.r, move |r| { Color { r, ..color } }).step(0.01),
        slider(0.0..=1.0, color.g, move |g| { Color { g, ..color } }).step(0.01),
        slider(0.0..=1.0, color.b, move |b| { Color { b, ..color } }).step(0.01),
        slider(0.0..=1.0, color.a, move |a| { Color { a, ..color } }).step(0.01),
    ]
    .spacing(8)
    .padding(8)
    .align_items(Alignment::Center)
    .into()
}

fn main() {
    println!("Hello, world!");
}
