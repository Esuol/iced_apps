use iced::subscription;
use std::hash::Hash;

pub fn file<I: 'static + Hash + Copy + Send + Sync, T: ToString>(
    id: I,
    url: T,
) -> iced::Subscription<(I, Progress)> {
}

#[derive(Debug, Clone)]
pub enum Progress {
    Started,
    Adnvanced(f32),
    Finished,
    Errored,
}
