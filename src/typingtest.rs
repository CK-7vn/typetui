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
    pub raw_wpm: i32,
    pub accuracy: i32,
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
            test_text: "tuis are probably the coolest thing since sliced bread".to_string(),
            user_input: String::new(),
            correct_char: 0,
            wpm: 0,
            raw_wpm: 0,
            accuracy: 0,
            time: None,
            word_count: 0,
            start_time: None,
            time_limit: None,
        }
    }
    pub fn handle_typing_input(key: KeyCode, app: &mut TypeTui) {
        let test = &mut app.typing;
        let test_chars: Vec<char> = test.test_text.chars().collect();
        match key {
            KeyCode::Char(c) => test.user_input.push(c),
            KeyCode::Backspace => {
                test.user_input.pop();
            }
            KeyCode::Esc => app.current_screen = Screen::Pause,
            _ => {}
        }
        if let Some(limit_secs) = test.time_limit {
            if test.start_time.is_none() {
                test.start_time = Some(Instant::now());
            }

            if let Some(start) = test.start_time {
                if start.elapsed().as_secs() >= limit_secs as u64 {
                    test.time = Some(start.elapsed());

                    test.correct_char = test_chars
                        .iter()
                        .zip(test.user_input.chars())
                        .filter(|(e, a)| *e == a)
                        .count() as i32;

                    test.calculate_wpm_acc();
                    test.word_count = (test.user_input.len() as i32) / 5;

                    if app.user.is_empty() {
                        app.current_screen = Screen::Login;
                    } else {
                        let time = limit_secs as i32;
                        app.db.add_test(
                            app.user.clone(),
                            test.wpm,
                            test.raw_wpm,
                            test.accuracy,
                            test.word_count,
                            time,
                        );
                        app.history = app.db.get_all_tests().unwrap_or_default();
                        app.current_screen = Screen::Stats;
                    }
                }
            }

            //this appends words if it is a timed test and the user is almost out of words
            let progress = test.user_input.len() as f64 / test.test_text.len() as f64;
            if progress >= 0.75 {
                test.append_words(10);
            }
        } else {
            //start timer on first keystroke
            if test.start_time.is_none() && !test.user_input.is_empty() {
                test.start_time = Some(Instant::now());
            }

            if test.user_input.len() < test.test_text.len() {
                return;
            }

            if let Some(start) = test.start_time {
                test.time = Some(start.elapsed());
            }

            test.correct_char = test_chars
                .iter()
                .zip(test.user_input.chars())
                .filter(|(e, a)| *e == a)
                .count() as i32;

            test.calculate_wpm_acc();
            test.word_count = (test.test_text.len() as i32) / 5;

            if app.user.is_empty() {
                app.current_screen = Screen::Login;
            } else {
                let time = test.time_limit.unwrap_or(0) as i32;
                app.db.add_test(
                    app.user.clone(),
                    test.wpm,
                    test.raw_wpm,
                    test.accuracy,
                    test.word_count,
                    time,
                );
                app.history = app.db.get_all_tests().unwrap_or_default();
                app.current_screen = Screen::Stats;
            }
        }
    }

    fn calculate_wpm_acc(&mut self) {
        let total_typed = self.user_input.chars().count() as i32;
        if total_typed > 0 {
            let pct = (self.correct_char as f64) / (total_typed as f64) * 100.0;
            self.accuracy = pct.round() as i32;
        } else {
            self.accuracy = 0;
        }
        if let Some(duration) = self.time {
            let mins = duration.as_secs_f64() / 60.0;
            if mins > 0.0 {
                self.wpm = ((self.correct_char as f64) / 5.0 / mins).round() as i32;
                self.raw_wpm = ((self.user_input.len() as f64) / 5.0 / mins).round() as i32;
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
