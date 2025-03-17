use std::io;

use crossterm::event::KeyCode;

use crate::{typingtest::TypingTest, ui};

pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Copy, Debug)]
pub enum Screen {
    Main { selected_option: usize },
    Typing, //return string or bec of char's
    TestOpts,
    Login,
    Stats,
    Quit,
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
//current screen event handler for each screen, input mode, normal, or enter click ready
//switch the user input mode and iunput handler checking if its enter then when they type push the
//characters into the input buffer
//start the tui, render the ui, access users input
//index of cursor x if thats the typing test than handle_typetest_input
//do everything on the app...don't make it more difficult than it has to be
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TypeTui {
    pub current_screen: Screen,
    pub cursor: (usize, usize),
    pub typing: TypingTest, //option allows you to model absence of a value
    input_mode: InputMode,
    character_index: usize,
    pub test_opts: TestOpts,
}

impl Default for TypeTui {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeTui {
    pub fn new() -> TypeTui {
        TypeTui {
            current_screen: Screen::Main { selected_option: 0 },
            typing: TypingTest::new(),
            cursor: (0, 0),
            input_mode: InputMode::Normal,
            character_index: 0,
            test_opts: TestOpts::new(),
        }
    }
    //depend on position of cursor, and input mode being input or normal,
    //screen, input mode,
    pub fn handle_input(&mut self) {}

    //set screen reset the cursor
    pub fn handle_change_screen(&mut self) {}

    //move cursor up

    // move cusor down
    pub fn handle_stats(&mut self) {}

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
                            crate::app::Screen::TestOpts => match app.test_opts.focus {
                                TestOptsFocus::Words => match key_event.code {
                                    KeyCode::Char(c) => app.test_opts.word_input.push(c),
                                    KeyCode::Backspace => {
                                        app.test_opts.word_input.pop();
                                    }
                                    KeyCode::Enter => {
                                        if let Ok(n) =
                                            app.test_opts.word_input.trim().parse::<usize>()
                                        {
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
                                        if app.test_opts.seconds_selected
                                            < app.test_opts.seconds_options.len() - 1
                                        {
                                            app.test_opts.seconds_selected += 1;
                                        }
                                    }
                                    KeyCode::Enter => {
                                        let chosen_seconds = app.test_opts.seconds_options
                                            [app.test_opts.seconds_selected];
                                        app.load_random_words(50);
                                        app.typing.time_limit = Some(chosen_seconds);
                                        app.current_screen = Screen::Typing;
                                    }
                                    KeyCode::Tab => {
                                        app.test_opts.focus = TestOptsFocus::Words;
                                    }
                                    _ => {}
                                },
                            },
                            crate::app::Screen::Typing => {
                                TypingTest::handle_typing_input(key_event.code, app);
                            }
                            crate::app::Screen::Stats => {
                                if key_event.code == KeyCode::Char('q') {
                                    return Ok(false);
                                }
                            }
                            crate::app::Screen::Quit => {
                                if key_event.code == KeyCode::Char('y') {
                                    return Ok(false);
                                } else if key_event.code == KeyCode::Char('n') {
                                    app.current_screen =
                                        crate::app::Screen::Main { selected_option: 0 };
                                }
                            }
                            _ => {}
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
    pub fn handle_test_ops(key: KeyCode) {}

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
                    0 => app.current_screen = Screen::Typing,
                    1 => app.current_screen = Screen::Login,
                    2 => app.current_screen = Screen::Stats,
                    3 => {
                        app.current_screen = Screen::Quit;
                        return Some(Ok(false));
                    }
                    4 => {
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
