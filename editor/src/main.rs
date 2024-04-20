use iced::highlighter::{self, highlighter};
use iced::keyboard;
use iced::widget::{
    button, column, container, horizotal_scroll, pick_list, row, text, text_editor, tooltip
}
use iced::{Alignment, Command, Element, Font, Length, Subscription, Theme};

use std::ffi;
use std::io;
use std::{Path, PathBuf};
use std::sync::Arc;

struct Editor {
    file: Option<PathBuf>,
    content: text_editor::Content,
    theme: highlighter::Theme,
    is_loading: bool,
    is_dirty: bool,
}

#[derive(Debug, Clone)]
enum Message {
    ActionPerformed(text_editor::Action),
    ThemeSelected(highlighter::Theme),
    NewFile,
    OpenFile,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    SaveFile,
    FileSaved(Result<PathBuf, Error>)
}

impl Editor {
    fn new() -> Self {
        Self {
            file: None,
            content: text_editor::Content::new(),
            theme: highlighter::Theme::SolarizedDark,
            is_loading: true,
            is_dirty: false,
        }
    }

    fn load() -> Command<Message> {
        Command::perform(
            load_file(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR"))),
            Message::FileOpened,
        )
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ActionPerformed(action) => {
                self.is_dirty = self.is_dirty || action.is_edit();

                self.content.perform(action);

                Command::none()
            }

            Message::NewFile => {
                if !self.is_loading {
                    self.file = None;
                    self.content = text_editor::Content::new();
                }

                Command::none()
            }

            Message::OpenFile => {
                if self.is_loading {
                   Command::none()
                } else {
                    self.is_loading = true;

                    Command::perform(open_file(), Message::FileOpened)
                }
            }

            Message::FileOpened(result) => {
                self.is_loading = false;
                self.is_dirty = false;

                if let Ok((path, contents)) = result {
                    self.file = Some(path);
                    self.content = text_editor::Content::with_text(&contents);
                }

                Command::none()
            }

            Message::SaveFile => {
                if self.is_loading {
                    Command::none()
                } else {
                    self.is_loading = true;

                    Command::perform(
                        save_file(self.file.clone(), self.content.text()),
                        Message::FileOpened
                    )
                }
            }

            Message::FileSaved(result) => {
                self.is_loading = false;

                if let Ok(path) = result {
                    self.file = Some(path);
                    self.is_dirty = false;
                }

                Command::none()
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
