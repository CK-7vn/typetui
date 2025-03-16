use core::time;
use std::time::{Duration, Instant};

use crossterm::event::KeyCode;

use crate::app::{Screen, TypeTui};

#[derive(Clone, Debug)]
pub struct TypingTest {
    pub test_text: String,
    pub user_input: String,
    pub wpm: u32,
    pub time: Option<Duration>,
    pub start_time: Option<Instant>,
}

impl Default for TypingTest {
    fn default() -> Self {
        Self::new()
    }
}
impl TypingTest {
    pub fn new() -> TypingTest {
        TypingTest {
            test_text: "Here is an example".to_string(),
            user_input: String::new(),
            wpm: 0,
            time: None,
            start_time: None,
        }
    }
    pub fn handle_typing_input(key: KeyCode, app: &mut TypeTui) {
        let test = &mut app.typing;
        if test.user_input.is_empty() {
            test.start_time = Some(Instant::now());
        }
        if test.user_input.len() != test.test_text.len() {
            match key {
                KeyCode::Char(c) => {
                    test.user_input.push(c);
                }
                KeyCode::Backspace => {
                    test.user_input.pop();
                }
                _ => {}
            }
        } else {
            if let Some(start) = test.start_time {
                test.time = Some(start.elapsed());
            }
            app.current_screen = Screen::Stats;
        }
    }
}
