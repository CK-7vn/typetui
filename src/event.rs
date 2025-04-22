use crossterm::event::{self, Event, KeyEvent};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum AppEvent {
    Tick,
    Key(KeyEvent),
}

#[derive(Debug)]
pub struct AppEventHandler {
    receiver: tokio::sync::mpsc::Receiver<AppEvent>,
}

impl AppEventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = tokio::sync::mpsc::channel(10);
        let send_event = sender.clone();

        let _handler = tokio::spawn(async move {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(tick_rate);

                if event::poll(timeout).expect("poll failed") {
                    match event::read().expect("read failed") {
                        Event::Key(key) => {
                            if send_event.send(AppEvent::Key(key)).await.is_err() {
                                break;
                            }
                        }
                        Event::FocusLost
                        | Event::FocusGained
                        | Event::Paste(_)
                        | Event::Mouse(_)
                        | Event::Resize(_, _) => {
                            if send_event.send(AppEvent::Tick).await.is_err() {
                                break;
                            }
                        }
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    if send_event.send(AppEvent::Tick).await.is_err() {
                        break;
                    }
                    last_tick = Instant::now();
                }
            }
        });

        Self { receiver }
    }

    pub async fn next(&mut self) -> Option<AppEvent> {
        self.receiver.recv().await
    }
}
