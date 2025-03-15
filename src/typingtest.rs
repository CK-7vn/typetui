use std::time::Instant;

pub struct TypingTest {
    pub test_text: String,
    pub user_input: String,
    pub wpm: u32,
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
        }
    }
}
