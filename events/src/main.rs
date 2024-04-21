use iced::alignment;
use iced::event::{self, Event};
use iced::widget::{button, checkbox, container, text, Column};
use iced::window;
use iced::{Alignment, Command, Element, Length, Subscription};

#[derive(Debug, Default)]
struct Events {
    last: Vec<Event>,
    enabled: bool,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    Toggled(bool),
    Exit,
}

impl Events {
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EventOccurred(event) => {
                self.last.push(event);

                if self.last.len() > 5 {
                    let _ = self.last.remove(0);
                }

                Command::none()
            }

            Message::EventOccurred(event) => {
                if let Event::Window(id, window::Event::CloseRequested) = event {
                    window::close(id)
                } else {
                    Command::none()
                }
            }

            Message::Toggled(enabled) => {
                self.enabled = enabled;
                Command::none()
            }

            Message::Exit => window::close(window::Id::MAIN),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
