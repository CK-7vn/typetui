use std::{cmp, io};

use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::Backend;

use crate::{typingtest::TypingTest, ui};
pub enum Menu {
    Main,
    Type,
    Login,
    Stats,
    Quit,
}
//to hold the current screen that the user is viewing
pub enum CurrentScreen {
    Main,
    Typing,
    Exiting,
}
struct User {
    pub username: String,
    pub pb_wpm: u32,
}

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

pub struct TypeTui {
    user: Option<User>, // wrap the user in an option incase we're doing an anon test
    pub current_screen: CurrentScreen,
    pub typing: Option<TypingTest>, //option allows you to model absence of a value
    pub menu: Menu,
    pub selected_option: usize, //holds index of menu option
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
            current_screen: CurrentScreen::Main,
            typing: None,
            menu: Menu::Main,
            selected_option: 2,
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

            handle_menu_input(key.code, app);
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    //Match to the main screen
                    KeyCode::Char('m') => {
                        //Matches to the e
                        //character keypress and sets current screen to
                        //editing and currently editing to key
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                        //return Ok(false);
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(false);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                // if the user is editing, are they editing a value or key, match
                // on that and then perform the right actions
                _ => {}
            }
        }
    }
}
// mutable reference to the app to change state, and a keycode
pub fn handle_menu_input(key: KeyCode, app: &mut TypeTui) {
    if let Menu::Main = app.menu {
        match key {
            KeyCode::Up => {
                if app.selected_option > 0 {
                    app.selected_option -= 1;
                } else if app.selected_option == 0 {
                    app.selected_option = 3;
                }
            }
            KeyCode::Down => {
                if app.selected_option < 3 {
                    app.selected_option += 1;
                } else if app.selected_option == 3 {
                    app.selected_option = 0;
                }
            }
            //when the user presses enter we'll check the selected options value and
            //then switch menu's based on that
            KeyCode::Enter => match app.selected_option {
                0 => app.menu = Menu::Type,
                1 => app.menu = Menu::Login,
                2 => app.menu = Menu::Stats,
                3 => app.menu = Menu::Quit,
                _ => {}
            },
            _ => {}
        }
    }
}
