use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyEvent};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AppEvent {
    Tick,
    Key(KeyEvent),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct AppEventHandler {
    sender: tokio::sync::mpsc::Sender<AppEvent>,
    receiver: tokio::sync::mpsc::Receiver<AppEvent>,
    handler: tokio::task::JoinHandle<()>,
}
//same calculation start at i32 min, and every iteration if its greater than the last one every
//tick.
#[allow(dead_code)]
impl AppEventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = tokio::sync::mpsc::channel(10);
        let send_event = sender.clone();
        let handler = {
            tokio::spawn(async move {
                let sender = send_event.clone();
                let last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("no events available") {
                        match event::read().expect("Unable to read event") {
                            Event::Key(e) => sender.send(AppEvent::Key(e)),
                            Event::FocusLost => sender.send(AppEvent::Tick),
                            Event::FocusGained => sender.send(AppEvent::Tick),
                            Event::Paste(_s) => sender.send(AppEvent::Tick),
                            Event::Mouse(_) => sender.send(AppEvent::Tick),
                            Event::Resize(_, _) => sender.send(AppEvent::Tick),
                        }
                        .await
                        .expect("Failed to send terminal event")
                    }
                    if last_tick.elapsed() >= tick_rate {
                        sender
                            .send(AppEvent::Tick)
                            .await
                            .expect("Failed to send tick event");
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }
    pub async fn next(&mut self) -> Option<AppEvent> {
        self.receiver.recv().await
    }
}
