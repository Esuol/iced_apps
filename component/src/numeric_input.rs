use iced::alignment::{self, Alignment};
use iced::widget::{button, component, row, text, text_input, Component};
use iced::{Element, Length, Size};
pub struct NumericInput<Message> {
    value: Option<u32>,
    on_change: Box<dyn Fn(Option<u32>) -> Message>,
}

pub fn numeric_input<Message>(
    value: Option<u32>,
    on_change: impl Fn(Option<u32>) -> Message + 'static,
) -> NumericInput<Message> {
    NumericInput::new(value, on_change)
}

#[derive(Debug, Clone)]
pub enum Event {
    InputChanged(String),
    IncrementPressed,
    DecrementPressed,
}

impl<Message> NumericInput<Message> {
    pub fn new(value: Option<u32>, on_change: impl Fn(Option<u32>) -> Message + 'static) -> Self {
        Self {
            value,
            on_change: Box::new(on_change),
        }
    }
}

impl<Message, Theme> Component<Message, Theme> for NumericInput<Message>
where
    Theme: text::Catalog + button::Catalog + text_input::Catalog + 'static,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::IncrementPressed => Some((self.on_change)(Some(
                self.value.unwrap_or_default().saturating_add(1),
            ))),
            Event::DecrementPressed => Some((self.on_change)(Some(
                self.value.unwrap_or_default().saturating_sub(1),
            ))),
            Event::InputChanged(value) => {
                if value.is_empty() {
                    Some((self.on_change)(None))
                } else {
                    value
                        .parse()
                        .ok()
                        .map(Some)
                        .map(self.on_change.as_ref())
                }
            }
        }
    }
}
