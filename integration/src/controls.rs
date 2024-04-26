use iced::Command;
use iced_wgpu::Renderer;
use iced_widget::{column, container, row, slider, text, text_input};
use iced_winit::core::alignment;
use iced_winit::core::{Color, Element, Length, Theme};
use iced_winit::runtime::{Command, Program};

pub struct Controls {
    background_color: Color,
    input: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    BackgroundColorChanged(Color),
    InputChanged(String),
}

impl Controls {
    pub fn new() -> Controls {
        Controls {
            background_color: Color::BLACK,
            input: String::default(),
        }
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }
}

impl Program for Controls {
    type Theme = Theme;
    type Message = Message;
    type Renderer = Renderer;

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::BackgroundColorChanged(color) => {
                self.background_color = color;
            }
            Message::InputChanged(input) => {
                self.input = input;
            }
        }

        Command::none()
    }
}
