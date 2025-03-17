use core::time;
use std::{
    fs,
    time::{Duration, Instant},
};

use crossterm::event::KeyCode;
use rand::seq::IndexedRandom;

use crate::app::{Screen, TypeTui};

#[derive(Clone, Debug)]
pub struct TypingTest {
    pub test_text: String,
    pub user_input: String,
    pub correct_char: i32,
    pub wpm: i32,
    pub time: Option<Duration>,
    pub start_time: Option<Instant>,
    pub time_limit: Option<u16>,
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
            correct_char: 0,
            wpm: 0,
            time: None,
            start_time: None,
            time_limit: None,
        }
    }

    pub fn handle_typing_input(key: KeyCode, app: &mut TypeTui) {
        let test = &mut app.typing;
        let test_text: Vec<char> = test.test_text.chars().collect();
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
            for (i, c) in test.user_input.chars().enumerate() {
                if c == test_text[i] {
                    test.correct_char += 1
                }
            }
            test.calculate_wpm_acc();
            app.current_screen = Screen::Stats;
        }
    }

    fn calculate_wpm_acc(&mut self) {
        if let Some(time) = self.time {
            let secs = time.as_secs() as f64;
            let minutes = secs / 60.0;
            if minutes > 0.0 {
                // calculate WPM as (correct characters / 5) divided by elapsed minutes
                // this is monkeytypes way of calculating wpm
                self.wpm = ((self.correct_char as f64) / 5.0 / minutes).round() as i32;
            } else {
                self.wpm = 0;
            }
        }
    }
    pub fn get_words(&mut self, num_words: usize) {
        let contents = fs::read_to_string("./20k.txt").expect("can't get words from file");
        let words: Vec<&str> = contents
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();
        let mut rng = rand::rng();
        let chosen: Vec<&str> = words
            .choose_multiple(&mut rng, num_words.min(words.len()))
            .cloned()
            .collect();
        self.test_text = chosen.join(" ");
    }
}

pub fn count_chars(input: &str) -> usize {
    input
        .chars()
        .take_while(|ch| ch.is_whitespace() || ch.is_alphabetic() && *ch != '\n')
        .count()
}
