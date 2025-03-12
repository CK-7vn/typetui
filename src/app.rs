use std::io;

use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::Backend;

#[allow(dead_code)]
use crate::{typingtest::TypingTest, ui};

//to hold the current screen that the user is viewing
pub enum Screen {
    Main { selected_option: usize },
    Typing,
    Login,
    Stats,
    Quit,
}
#[allow(dead_code)]
struct User {
    pub username: String,
    pub pb_wpm: u32,
}

#[allow(dead_code)]
impl User {
    pub fn new() -> User {
        User {
            username: String::new(),
            pb_wpm: 0,
        }
    }
    pub fn update_pb(&mut self, wpm: u32) {
        self.pb_wpm = wpm;
    }
}

#[allow(dead_code)]
pub struct TypeTui {
    user: Option<User>, // wrap the user in an option incase we're doing an anon test
    pub current_screen: Screen,
    pub typing: Option<TypingTest>, //option allows you to model absence of a value
}

impl Default for TypeTui {
    fn default() -> Self {
        Self::new()
    }
}
impl TypeTui {
    pub fn new() -> TypeTui {
        TypeTui {
            user: Some(User {
                username: String::new(),
                pb_wpm: 7,
            }),
            current_screen: Screen::Main { selected_option: 0 },
            typing: None,
        }
    }
}

//generic over backen, we're using crossterm here. also takes a mutable reference to our app
pub fn run_app<B: Backend>(
    terminal: &mut ratatui::Terminal<B>,
    app: &mut TypeTui,
) -> io::Result<bool> {
    loop {
        //terminal is the terminal<backend> that we take as an argument and draw is the
        //ratatui command that draws a Frame to the terminal
        terminal.draw(|f| ui::ui(f, app))?; // this is a closure that tells draw that we want to
                                            // take f:Frame and pass it t our function ui and ui will draw to that Frame
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            if let Some(result) = handle_menu_input(key.code, app) {
                return result;
            }

            match app.current_screen {
                Screen::Quit => match key.code {
                    KeyCode::Char('y') => return Ok(false),
                    KeyCode::Char('n') => {
                        app.current_screen = Screen::Main { selected_option: 0 };
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
// mutable reference to the app to change state, and a keycode
pub fn handle_menu_input(key: KeyCode, app: &mut TypeTui) -> Option<io::Result<bool>> {
    if let Screen::Main {
        ref mut selected_option,
    } = app.current_screen
    {
        let num_options = 4;
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
                _ => {}
            },
            _ => {}
        }
    }
    None
}
