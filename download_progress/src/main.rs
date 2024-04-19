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
        match  self.state {
            State::Idle { .. }
                | State::Finished {..}
                | State::Errored {..} => {
                    self.state = State::Downloading { progress: 0.0 };
                }
                State::Downloading { .. } => {}
        }
    }
}

fn main() {
    println!("Hello, world!");
}
