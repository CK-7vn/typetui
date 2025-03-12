use std::time::Instant;

#[allow(dead_code)]
pub struct TypingTest {
    pub user_input: String,
    pub test_text: String,
}

impl Default for TypingTest {
    fn default() -> Self {
        Self::new()
    }
}
impl TypingTest {
    pub fn new() -> TypingTest {
        TypingTest {
            user_input: String::new(),
            test_text: "How about this test i guess".to_string(),
        }
    }
}
