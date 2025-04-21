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
    pub word_count: i32,
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
            word_count: 0,
            start_time: None,
            time_limit: None,
        }
    }
    pub fn handle_typing_input(key: KeyCode, app: &mut TypeTui) {
        let test = &mut app.typing;
        let test_chars: Vec<char> = test.test_text.chars().collect();

        // Timed test branch: if time_limit is set.
        if let Some(time_limit) = test.time_limit {
            // Start timer on first input.
            if test.start_time.is_none() {
                test.start_time = Some(Instant::now());
            }

            // Check if time is up.
            if let Some(start) = test.start_time {
                if start.elapsed().as_secs() >= time_limit as u64 {
                    test.time = Some(start.elapsed());
                    for (i, c) in test.user_input.chars().enumerate() {
                        if c == test_chars[i] {
                            test.correct_char += 1;
                        }
                    }
                    test.calculate_wpm_acc();
                    let mut test_time: u16 = 0;
                    if let Some(time_maybe) = test.time_limit {
                        test_time = time_maybe;
                    }
                    app.db.add_test(
                        app.user.clone(),
                        test.wpm,
                        test.word_count,
                        test_time.into(),
                    );
                    app.current_screen = Screen::Stats;
                }
            }
            // Process key press.
            match key {
                KeyCode::Char(c) => {
                    test.user_input.push(c);
                }
                KeyCode::Backspace => {
                    test.user_input.pop();
                }
                KeyCode::Esc => app.current_screen = Screen::Pause,
                _ => {}
            }

            // If the user has typed 75% (or more) of the current text,
            // append additional words (for example, 10 words).
            let typed_len = test.user_input.len();
            let total_len = test.test_text.len();
            if total_len > 0 && (typed_len as f64 / total_len as f64) >= 0.75 {
                test.append_words(10);
            }
        } else {
            // Non-timed test branch: wait until the user finishes the test text.
            if test.user_input.is_empty() {
                test.start_time = Some(Instant::now());
            }
            if test.user_input.len() < test.test_text.len() {
                match key {
                    KeyCode::Char(c) => {
                        test.user_input.push(c);
                    }
                    KeyCode::Backspace => {
                        test.user_input.pop();
                    }
                    KeyCode::Esc => app.current_screen = Screen::Pause,
                    _ => {}
                }
            } else {
                if let Some(start) = test.start_time {
                    test.time = Some(start.elapsed());
                }
                for (i, c) in test.user_input.chars().enumerate() {
                    if c == test_chars[i] {
                        test.correct_char += 1;
                    }
                }
                test.calculate_wpm_acc();
                let mut test_time: u16 = 0;
                if let Some(time_maybe) = test.time_limit {
                    test_time = time_maybe;
                }

                if app.user.is_empty() {
                    app.current_screen = Screen::Login;
                } else {
                    let text_length = test.test_text.len() as i32;
                    test.word_count = text_length / 5;
                    app.db.add_test(
                        app.user.clone(),
                        test.wpm,
                        test.word_count,
                        test_time.into(),
                    );
                    test.test_text = "".to_string();
                    test.user_input = "".to_string();
                    app.current_screen = Screen::Stats;
                    app.history = app.db.get_all_tests().unwrap();
                }
            }
        }
    }

    fn calculate_wpm_acc(&mut self) {
        if let Some(time) = self.time {
            let secs = time.as_secs() as f64;
            let minutes = secs / 60.0;
            if secs > 0.0 {
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
    pub fn append_words(&mut self, num_words: usize) {
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

        if !self.test_text.is_empty() {
            self.test_text.push(' ');
        }
        self.test_text.push_str(&chosen.join(" "));
    }
}
//counting chars helper function
pub fn count_chars(input: &str) -> usize {
    input
        .chars()
        .take_while(|ch| ch.is_whitespace() || ch.is_alphabetic() && *ch != '\n')
        .count()
}
