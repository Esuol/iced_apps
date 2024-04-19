mod download;

use iced::overlay::menu::State;
use iced::widget::{button, column, container, progress_bar, text, Column};
use iced::{Alignment, Element, Length, Subscription};

#[derive(Debug, Clone)]
struct Example {
    downloads: Vec<Download>,
    last_id: usize,
}

#[derive(Debug)]
pub enum Message {
    Add,
    Download(usize),
    DownloadProgressed((usize, download::Progress)),
}

#[derive(Debug)]
struct Download {
    id: usize,
    state: State,
}

#[derive(Debug)]
enum State {
    Idle,
    Downloading { progress: f32 },
    Finished,
    Errored,
}

impl Download {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            state: State::Idle,
        }
    }

    pub fn start(&mut self) {
        match self.state {
            State::Idle { .. } | State::Finished { .. } | State::Errored { .. } => {
                self.state = State::Downloading { progress: 0.0 };
            }
            State::Downloading { .. } => {}
        }
    }

    pub fn progress(&mut self, new_progress: download::Progress) {
        if let State::Downloading { progress } = &mut self.state() {
            match new_progress {
                download::Progress::Started => {
                    *progress = 0.0;
                }
                download::Progress::Advanced(p) => {
                    *progress = p;
                }
                download::Progress::Finished => {
                    self.state = State::Finished;
                }
                download::Progress::Errored => {
                    self.state = State::Errored;
                }
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Downloading { .. } => {
                download::file(self.id, "https://speed.hetzner.de/100MB.bin?")
                    .map(Message::DownloadProgressed)
            }
            _ => Subscription::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let current_progress = match &self.state {
            State::Idle { .. } => 0.0,
            State::Downloading { progress } => *progress,
            State::Finished { .. } => 100.0,
            State::Errored { .. } => 0.0,
        };

        let progress_bar = progress_bar(0.0..=100.0, current_progress);

        let control: Element<_> = match &self.state {
            State::Idle => button("Start the download!")
                .on_press(Message::Download(self.id))
                .into(),
            State::Finished => column!["Download finished", button("Start again")]
                .spacing(10)
                .align_items(Alignment::Center)
                .into(),
            State::Downloading { .. } => {
                text(format!("Downloading... {current_progress:.2}%")).into()
            }
            State::Errored => column![
                "Something went wrong :(",
                button("Try again").on_press(Message::Download(self.id)),
            ]
            .spacing(10)
            .align_items(Alignment::Center)
            .into(),
        };

        Column::new()
            .spacing(10)
            .padding(10)
            .align_items(Alignment::Center)
            .push(progress_bar)
            .push(control)
            .into()
    }
}

fn main() {
    println!("Hello, world!");
}
