use iced::widget::{button, column, container};
use iced::window;
use iced::{Alignment, Command, Element, Length};

fn main() {
    println!("Hello, world!");
}

#[derive(Default)]
struct Exit {
    show_confirm: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Confirm,
    Exit,
}

impl Exit {
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Confirm => window::close(window::Id::MAIN),
            Message::Exit => {
                self.show_confirm = true;

                Command::none()
            }
        }
    }
}
