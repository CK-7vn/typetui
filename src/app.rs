use crate::io;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::TableState;

use crate::{
    db::{self, DB},
    typingtest::TypingTest,
    ui,
};

pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Copy, Debug)]
pub enum Screen {
    Main { selected_option: usize },
    Typing, //return string or bec of char's
    TestOpts,
    Login,
    Stats,
    History,
    Quit,
    Pause,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum InputMode {
    Normal,
    Editing,
}

#[derive(Clone, Debug)]
pub struct TestOpts {
    pub focus: TestOptsFocus,
    pub word_input: String,
    pub seconds_options: Vec<u16>,
    pub seconds_selected: usize,
}

#[derive(Clone, Debug)]
pub enum TestOptsFocus {
    Words,
    Seconds,
}

impl Default for TestOpts {
    fn default() -> Self {
        Self {
            focus: TestOptsFocus::Words,
            word_input: String::new(),
            seconds_options: vec![15, 30, 60],
            seconds_selected: 0,
        }
    }
}
impl TestOpts {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct TypeTui {
    pub current_screen: Screen,
    pub cursor: (usize, usize),
    pub typing: TypingTest, //option allows you to model absence of a value
    input_mode: InputMode,
    character_index: usize,
    pub test_opts: TestOpts,
    pub db: db::DB,
    pub user: String,
    pub login_input: String,
    pub stats_list_state: ratatui::widgets::TableState,
    pub history: Vec<(String, i32, i32, i32, i32, i32)>,
    pub pause_selected: usize,
}

impl Default for TypeTui {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeTui {
    pub fn new() -> TypeTui {
        let mut state = TableState::default();
        state.select(Some(0));
        TypeTui {
            current_screen: Screen::Main { selected_option: 0 },
            typing: TypingTest::new(),
            cursor: (0, 0),
            input_mode: InputMode::Normal,
            character_index: 0,
            test_opts: TestOpts::new(),
            db: Result::expect(DB::new(), "error creating db"),
            user: "".to_string(),
            login_input: String::new(),
            stats_list_state: state,
            history: Vec::new(),
            pause_selected: 0,
        }
    }
    pub fn load_random_words(&mut self, num_words: usize) {
        self.typing.get_words(num_words);
    }

    pub async fn run_app<B: ratatui::prelude::Backend>(
        terminal: &mut ratatui::Terminal<B>,
        app: &mut TypeTui,
        event_handler: &mut crate::event::AppEventHandler,
    ) -> io::Result<bool> {
        loop {
            terminal.draw(|f| {
                ui::ui(f, app).unwrap();
            })?;

            if let Some(event) = event_handler.next().await {
                match event {
                    crate::event::AppEvent::Key(key_event) => {
                        if key_event.code == KeyCode::Char('c')
                            && key_event
                                .modifiers
                                .contains(crossterm::event::KeyModifiers::CONTROL)
                        {
                            return Ok(false);
                        }
                        match app.current_screen {
                            crate::app::Screen::Main { .. } => {
                                let _ = TypeTui::handle_menu_input(key_event.code, app);
                            }
                            crate::app::Screen::TestOpts => {
                                TypeTui::handle_test_ops(app, key_event);
                            }
                            crate::app::Screen::Typing => {
                                TypingTest::handle_typing_input(key_event.code, app);
                            }
                            crate::app::Screen::History => match key_event.code {
                                KeyCode::Up => {
                                    let i = app.stats_list_state.selected().unwrap_or(0);
                                    let len = app.history.len();
                                    let new = if i == 0 { len.saturating_sub(1) } else { i - 1 };
                                    app.stats_list_state.select(Some(new));
                                }
                                KeyCode::Down => {
                                    let i = app.stats_list_state.selected().unwrap_or(0);
                                    let len = app.history.len();
                                    let new = if i + 1 >= len { 0 } else { i + 1 };
                                    app.stats_list_state.select(Some(new));
                                }
                                KeyCode::Esc => {
                                    app.current_screen = Screen::Main { selected_option: 0 }
                                }
                                KeyCode::Char('q') => {
                                    return Ok(false);
                                }
                                _ => {}
                            },
                            crate::app::Screen::Stats => match key_event.code {
                                KeyCode::Char('q') => {
                                    return Ok(false);
                                }
                                KeyCode::Esc => {
                                    app.current_screen = Screen::Main { selected_option: 0 }
                                }
                                _ => {}
                            },
                            crate::app::Screen::Quit => {
                                if key_event.code == KeyCode::Char('y') {
                                    return Ok(false);
                                } else if key_event.code == KeyCode::Char('n') {
                                    app.current_screen =
                                        crate::app::Screen::Main { selected_option: 0 };
                                }
                            }
                            Screen::Login => match key_event.code {
                                KeyCode::Char(c) => {
                                    app.login_input.push(c);
                                }
                                KeyCode::Backspace => {
                                    app.login_input.pop();
                                }
                                KeyCode::Enter => {
                                    app.confirm_login();
                                }
                                KeyCode::Esc => {
                                    app.current_screen = Screen::Main { selected_option: 0 };
                                }
                                _ => {}
                            },
                            Screen::Pause => match key_event.code {
                                KeyCode::Up => {
                                    if app.pause_selected == 0 {
                                        app.pause_selected = 3;
                                    } else {
                                        app.pause_selected -= 1;
                                    }
                                }
                                KeyCode::Down => {
                                    app.pause_selected = (app.pause_selected + 1) % 4;
                                }
                                KeyCode::Enter => match app.pause_selected {
                                    0 => {
                                        app.typing.user_input = "".to_string();
                                        app.typing.time = None;
                                        app.typing.start_time = None;
                                        app.current_screen = Screen::Typing;
                                    }
                                    1 => {
                                        app.current_screen = Screen::TestOpts;
                                    }
                                    2 => {
                                        app.current_screen = Screen::Main { selected_option: 0 };
                                    }
                                    3 => {
                                        return Ok(false);
                                    } // quit
                                    _ => {}
                                },
                                KeyCode::Esc => {
                                    // resume
                                    app.current_screen = Screen::Typing;
                                }
                                _ => {}
                            },
                        }
                    }
                    crate::event::AppEvent::Tick => {}
                }
            }

            if let crate::app::Screen::Quit = app.current_screen {
                return Ok(false);
            }
        }
    }
    pub fn reset_test(&mut self) {
        self.typing.user_input.clear();
        self.typing.correct_char = 0;
        self.typing.wpm = 0;
        self.typing.start_time = None;
        self.typing.time = None;
        self.typing.time_limit = None;
        self.typing.word_count = 0;
        self.typing.test_text.clear();
    }

    pub fn confirm_login(&mut self) {
        let uname = self.login_input.trim().to_string();
        if uname.is_empty() {
            return;
        }
        self.user = uname.to_string();
        self.login_input.clear();
        if self.typing.wpm != 0 {
            let word_count = if self.typing.time_limit.is_some() {
                (self.typing.user_input.len() as i32) / 5
            } else {
                (self.typing.test_text.len() as i32) / 5
            };
            self.typing.word_count = word_count;
            self.db.add_test(
                uname.clone(),
                self.typing.wpm,
                self.typing.raw_wpm,
                self.typing.accuracy,
                word_count,
                self.typing.time_limit.unwrap_or(0) as i32,
            );
            self.history = self.db.get_all_tests().expect("error getting all tests");
            self.current_screen = Screen::Stats;
        } else {
            self.current_screen = Screen::Main { selected_option: 0 }
        }
    }
    pub fn handle_test_ops(app: &mut TypeTui, key_event: KeyEvent) {
        match app.test_opts.focus {
            TestOptsFocus::Words => match key_event.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    app.current_screen = Screen::Quit;
                }
                KeyCode::Esc => {
                    app.current_screen = Screen::Main { selected_option: 0 };
                }
                KeyCode::Char(c) => app.test_opts.word_input.push(c),
                KeyCode::Backspace => {
                    app.test_opts.word_input.pop();
                }
                KeyCode::Enter => {
                    app.reset_test();
                    if let Ok(n) = app.test_opts.word_input.trim().parse::<usize>() {
                        app.load_random_words(n);
                        app.current_screen = Screen::Typing;
                    }
                }
                KeyCode::Tab => {
                    app.test_opts.focus = TestOptsFocus::Seconds;
                }
                _ => {}
            },
            TestOptsFocus::Seconds => match key_event.code {
                KeyCode::Up => {
                    if app.test_opts.seconds_selected > 0 {
                        app.test_opts.seconds_selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if app.test_opts.seconds_selected < app.test_opts.seconds_options.len() - 1 {
                        app.test_opts.seconds_selected += 1;
                    }
                }
                KeyCode::Enter => {
                    app.reset_test();
                    let chosen_seconds =
                        app.test_opts.seconds_options[app.test_opts.seconds_selected];
                    app.load_random_words(50);
                    app.typing.time_limit = Some(chosen_seconds);
                    app.current_screen = Screen::Typing;
                }
                KeyCode::Tab => {
                    app.test_opts.focus = TestOptsFocus::Words;
                }
                KeyCode::Esc => app.current_screen = Screen::Main { selected_option: 0 },
                KeyCode::Char('q') | KeyCode::Char('Q') => app.current_screen = Screen::Quit,
                _ => {}
            },
        }
    }

    pub fn refresh_history(&mut self) {
        match self.db.get_all_tests() {
            Ok(rows) => {
                self.history = rows;
                self.stats_list_state.select(Some(0));
            }
            Err(e) => {
                eprintln!("DB Error loading history: {}", e);
                self.history.clear();
            }
        }
    }

    //handle input takes the screen and then the app
    // mutable reference to the app to change state, and a keycode
    pub fn handle_menu_input(key: KeyCode, app: &mut TypeTui) -> Option<io::Result<bool>> {
        if let Screen::Main {
            ref mut selected_option,
        } = app.current_screen
        {
            let num_options = 5;
            match key {
                KeyCode::Up => {
                    if *selected_option == 0 {
                        *selected_option = num_options - 1;
                    } else {
                        *selected_option -= 1;
                    }
                }
                KeyCode::Down => {
                    if *selected_option == num_options - 1 {
                        *selected_option = 0;
                    } else {
                        *selected_option += 1;
                    }
                }
                //when the user presses enter we'll check the selected options value and
                //then switch menu's based on that
                KeyCode::Enter => match *selected_option {
                    0 => {
                        //app.reset_test();
                        //const DEFAULT_WORD_COUNT: usize = 50;
                        //app.typing.get_words(DEFAULT_WORD_COUNT);
                        //app.typing.time_limit = Some(15);
                        app.current_screen = Screen::Typing
                    }
                    1 => app.current_screen = Screen::Login,
                    2 => {
                        app.refresh_history();
                        app.current_screen = Screen::History
                    }
                    3 => {
                        app.current_screen = Screen::Quit;
                        return Some(Ok(false));
                    }
                    4 => {
                        app.reset_test();
                        app.current_screen = Screen::TestOpts;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        None
    }
}
