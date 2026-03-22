use iced::alignment;
use iced::mouse;
use iced::widget::canvas::{self, stroke, Cache, Frame, Geometry, LineCap, Path, Stroke};
use iced::widget::{column, container, row, text};
use iced::{
    Alignment, Color, Degrees, Element, Font, Length, Point, Rectangle, Renderer, Subscription,
    Theme, Vector,
};

pub fn main() -> iced::Result {
    iced::program("Clock App", Clock::update, Clock::view)
        .subscription(Clock::subscription)
        .theme(Clock::theme)
        .antialiasing(true)
        .run()
}

struct Clock {
    now: time::OffsetDateTime,
    clock: Cache,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(time::OffsetDateTime),
}

impl Clock {
    fn update(&mut self, message: Message) {
        match message {
            Message::Tick(now) if now != self.now => {
                self.now = now;
                self.clock.clear();
            }
            Message::Tick(_) => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let time_label = text(format_digital_time(self.now))
            .size(56)
            .font(Font::MONOSPACE);

        let date_label = text(format_date(self.now)).size(24);
        let detail_label = text(format!(
            "{}  |  {}",
            format_weekday(self.now.weekday()),
            format_utc_offset(self.now.offset())
        ))
        .size(16);

        let info_panel = container(
            column![time_label, date_label, detail_label]
                .spacing(8)
                .align_items(Alignment::Center),
        )
        .padding([20, 28])
        .width(Length::Fill)
        .style(|theme| panel_style(theme, 0.22));

        let clock_face = container(
            canvas::Canvas::new(self as &Self)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .padding(24)
        .width(Length::Fill)
        .height(Length::FillPortion(3))
        .style(|theme| panel_style(theme, 0.14));

        container(
            column![
                text("Clock").size(18),
                row![clock_face]
                    .width(Length::Fill)
                    .height(Length::FillPortion(1)),
                info_panel,
            ]
            .spacing(18)
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(24)
        .center_x()
        .center_y()
        .style(|theme| app_style(theme))
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(200)).map(|_| {
            Message::Tick(
                time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
            )
        })
    }

    fn theme(&self) -> Theme {
        Theme::default()
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            now: time::OffsetDateTime::now_local()
                .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
            clock: Cache::default(),
        }
    }
}

impl<Message> canvas::Program<Message> for Clock {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.clock.draw(renderer, bounds.size(), |frame| {
            draw_clock(frame, theme, self.now);
        });

        vec![geometry]
    }
}

fn draw_clock(frame: &mut Frame<Renderer>, theme: &Theme, now: time::OffsetDateTime) {
    let palette = theme.extended_palette();
    let center = frame.center();
    let radius = frame.width().min(frame.height()) * 0.42;

    let outer_ring = Path::circle(center, radius);
    frame.fill(&outer_ring, palette.background.strong.color);

    let ring = Path::circle(center, radius * 0.92);
    frame.fill(&ring, palette.background.base.color);

    for hour in 0..12 {
        let angle = rotation(hour as f32, 12.0);
        draw_tick(frame, center, radius, angle, radius * 0.12, radius / 30.0);
    }

    for minute in 0..60 {
        if minute % 5 == 0 {
            continue;
        }

        let angle = rotation(minute as f32, 60.0);
        draw_tick(frame, center, radius, angle, radius * 0.07, radius / 90.0);
    }

    let second_progress = now.second() as f32 + now.millisecond() as f32 / 1000.0;
    let minute_progress = now.minute() as f32 + second_progress / 60.0;
    let hour_progress = (now.hour() % 12) as f32 + minute_progress / 60.0;

    draw_hand(
        frame,
        center,
        rotation(hour_progress, 12.0),
        radius * 0.5,
        radius / 18.0,
        palette.primary.strong.color,
    );
    draw_hand(
        frame,
        center,
        rotation(minute_progress, 60.0),
        radius * 0.72,
        radius / 28.0,
        palette.secondary.strong.color,
    );
    draw_hand(
        frame,
        center,
        rotation(second_progress, 60.0),
        radius * 0.8,
        radius / 80.0,
        Color::from_rgb8(0xE6, 0x53, 0x4F),
    );

    let center_dot = Path::circle(center, radius / 22.0);
    frame.fill(&center_dot, palette.background.weak.text);

    frame.fill_text(canvas::Text {
        content: format!("{:02}:{:02}", now.hour(), now.minute()),
        size: (radius / 7.0).into(),
        position: Point::new(center.x, center.y + radius * 0.47),
        color: palette.background.base.text,
        horizontal_alignment: alignment::Horizontal::Center,
        vertical_alignment: alignment::Vertical::Center,
        font: Font::MONOSPACE,
        ..canvas::Text::default()
    });
}

fn draw_tick(
    frame: &mut Frame<Renderer>,
    center: Point,
    radius: f32,
    angle: Degrees,
    length: f32,
    width: f32,
) {
    let direction = angle_to_vector(angle);
    let outer = Point::new(center.x + direction.x * radius, center.y + direction.y * radius);
    let inner = Point::new(
        center.x + direction.x * (radius - length),
        center.y + direction.y * (radius - length),
    );

    let tick = Path::line(inner, outer);
    frame.stroke(
        &tick,
        Stroke {
            width,
            style: stroke::Style::Solid(Color::from_rgba8(0xFF, 0xFF, 0xFF, 0.72)),
            line_cap: LineCap::Round,
            ..Stroke::default()
        },
    );
}

fn draw_hand(
    frame: &mut Frame<Renderer>,
    center: Point,
    angle: Degrees,
    length: f32,
    width: f32,
    color: Color,
) {
    let direction = angle_to_vector(angle);
    let tip = Point::new(center.x + direction.x * length, center.y + direction.y * length);
    let hand = Path::line(center, tip);

    frame.stroke(
        &hand,
        Stroke {
            width,
            style: stroke::Style::Solid(color),
            line_cap: LineCap::Round,
            ..Stroke::default()
        },
    );
}

fn angle_to_vector(angle: Degrees) -> Vector {
    let radians = (angle.0 - 90.0).to_radians();

    Vector::new(radians.cos(), radians.sin())
}

fn rotation(value: f32, total: f32) -> Degrees {
    Degrees(360.0 * (value / total))
}

fn format_digital_time(now: time::OffsetDateTime) -> String {
    format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second())
}

fn format_date(now: time::OffsetDateTime) -> String {
    format!("{} {:02}, {}", month_name(now.month()), now.day(), now.year())
}

fn format_weekday(weekday: time::Weekday) -> &'static str {
    match weekday {
        time::Weekday::Monday => "Monday",
        time::Weekday::Tuesday => "Tuesday",
        time::Weekday::Wednesday => "Wednesday",
        time::Weekday::Thursday => "Thursday",
        time::Weekday::Friday => "Friday",
        time::Weekday::Saturday => "Saturday",
        time::Weekday::Sunday => "Sunday",
    }
}

fn month_name(month: time::Month) -> &'static str {
    match month {
        time::Month::January => "January",
        time::Month::February => "February",
        time::Month::March => "March",
        time::Month::April => "April",
        time::Month::May => "May",
        time::Month::June => "June",
        time::Month::July => "July",
        time::Month::August => "August",
        time::Month::September => "September",
        time::Month::October => "October",
        time::Month::November => "November",
        time::Month::December => "December",
    }
}

fn format_utc_offset(offset: time::UtcOffset) -> String {
    let total_seconds = offset.whole_seconds();
    let sign = if total_seconds >= 0 { '+' } else { '-' };
    let total_seconds = total_seconds.abs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;

    format!("UTC{}{:02}:{:02}", sign, hours, minutes)
}

fn app_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        text_color: Some(palette.background.base.text),
        background: Some(palette.background.base.color.into()),
        ..container::Style::default()
    }
}

fn panel_style(theme: &Theme, alpha: f32) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        text_color: Some(palette.background.base.text),
        background: Some(
            Color {
                a: alpha,
                ..palette.primary.base.color
            }
            .into(),
        ),
        border: iced::Border {
            radius: 20.0.into(),
            width: 1.0,
            color: Color::from_rgba8(0xFF, 0xFF, 0xFF, 0.08),
        },
        ..container::Style::default()
    }
}
