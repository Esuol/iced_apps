use iced::subscription;
use std::hash::Hash;

pub fn file<I: 'static + Hash + Copy + Send + Sync, T: ToString>(
    id: I,
    url: T,
) -> iced::Subscription<(I, Progress)> {
    subscription::unfold(id, State::Ready(url.to_string()), move |state| {
        download(id, state)
    })
}

async fn download<I: Copy>(id: I, state: State) -> ((I, Progress), State) {
    match state {
        State::Ready(url) => {
            let response = reqwest::get(&url).await;

            match response {
                Ok(response) => {
                    if let Some(total) = response.content_length() {
                        (
                            (id, Progress::Started),
                            State::Downloading {
                                response,
                                total,
                                downloaded: 0,
                            },
                        )
                    } else {
                        ((id, Progress::Errored), State::Finished)
                    }
                }
                Err(_) => ((id, Progress::Errored), State::Finished),
            }
        }

        State::Downloading {
            mut response,
            total,
            downloaded,
        } => match response.chunk().await {
            Ok(Some(chunk)) => {
                let downloaded = downloaded + chunk.len() as u64;

                let progress = (downloaded as f32 / total as f32) * 100.0;

                (
                    (id, Progress::Adnvanced(progress)),
                    State::Downloading {
                        response,
                        total,
                        downloaded,
                    },
                )
            }
            Ok(None) => ((id, Progress::Finished), State::Finished),
            Err(_) => ((id, Progress::Errored), State::Finished),
        },
        State::Finished => iced::futures::future::pending().await,
    }
}

#[derive(Debug, Clone)]
pub enum Progress {
    Started,
    Adnvanced(f32),
    Finished,
    Errored,
}

pub enum State {
    Ready(String),
    Downloading {
        response: reqwest::Response,
        total: u64,
        downloaded: u64,
    },
    Finished,
}
