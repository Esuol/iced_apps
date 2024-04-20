use iced::highlighter::{self, highlighter};
use iced::keyboard;
use iced::widget::{
    button, column, container, horizotal_scroll, pick_list, row, text, text_editor, tooltip
}
use iced::{Alignment, Command, Element, Font, Length, Subscription, Theme};

use core::error;
use std::{ffi, path};
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

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}

async fn open_file() -> Result<(PathBuf, Arc<String>), Error> {
    let picked_file = rfd::AsyncFileDialog::new()
    .set_title("Open a text File")
    .pick_file()
    .await
    .ok_or(Error::DialogClosed)?;

    load_file(picked_file).await
}

async fn load_file(path: impl Into<PathBuf>) -> Result<(PathBuf, Arc<String>), Error> {
    let path = path.into();

    let contents = tokio::fs::read_to_string(&path)
    .await
    .map(Arc::new)
    .map_err(|error| Error::IoError(error.kind()))?;

    Ok((path, contents))
}

async fn save_file(
    path: Option<PathBuf>,
    contents: String
) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
        .save_file()
        .await
        .as_ref()
        .map(rfd::FileHandle::path)
        .map(Path::to_owned)
        .ok_or(Error::DialogClosed)?
    };

    tokio::fs::write(&path, contents)
    .await
    .map_err(|error| Error::IoError(error.kind()))?;

    Ok(path)
}

fn main() {
    println!("Hello, world!");
}
