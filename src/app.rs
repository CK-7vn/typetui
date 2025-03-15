use std::io;

use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::Backend;

#[allow(dead_code)]
use crate::{typingtest::TypingTest, ui};

//to hold the current screen that the user is viewing
pub enum Screen {
    Main { selected_option: usize },
    Typing, //return string or bec of char's
    Login,
    Stats,
    Quit,
}

#[allow(dead_code)]
enum InputMode {
    Normal,
    Editing,
}

//current screen event handler for each screen, input mode, normal, or enter click ready
//switch the user input mode and iunput handler checking if its enter then when they type push the
//characters into the input buffer
//start the tui, render the ui, access users input
//index of cursor x if thats the typing test than handle_typetest_input
//do everything on the app...don't make it more difficult than it has to be
#[allow(dead_code)]
pub struct TypeTui {
    pub current_screen: Screen,
    pub cursor: (usize, usize),
    pub typing: TypingTest, //option allows you to model absence of a value
    input_mode: InputMode,
    character_index: usize,
    //wpm here?
    //of type tui_input buffer with input and cursor.
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
        }
    }
    //depend on position of cursor, and input mode being input or normal,
    //screen, input mode,
    pub fn handle_input(&mut self) {}

    //set screen reset the cursor
    pub fn handle_change_screen(&mut self) {}

    //move cursor up

    // move cusor down
    fn handle_typing_input(key: KeyCode, app: &mut TypeTui) {
        let test = &mut app.typing;
        match key {
            KeyCode::Char(c) => {
                test.user_input.push(c);
            }
            KeyCode::Backspace => {
                test.user_input.pop();
            }
            _ => {}
        }
    }

    //generic over backend, we're using crossterm here. also takes a mutable reference to our app
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

                std::thread::spawn(move || -> io::Result<bool> {
                    loop {
                        if event::poll(std::time::Duration::from_millis(100))? {
                            if let Event::Key(key_event) = event::read()? {
                                if key_event.code == KeyCode::Char('c')
                                    && key_event
                                        .modifiers
                                        .contains(crossterm::event::KeyModifiers::CONTROL)
                                {
                                    return Ok(false);
                                }
                            }
                        }
                    }
                });
                //method on app called handle input

                match app.current_screen {
                    Screen::Main { .. } => {
                        if let Some(result) = Self::handle_menu_input(key.code, app) {
                            return result;
                        }
                    }
                    Screen::Quit => match key.code {
                        KeyCode::Char('y') => return Ok(false),
                        KeyCode::Char('n') => {
                            app.current_screen = Screen::Main { selected_option: 0 };
                        }
                        _ => {}
                    },
                    Screen::Typing => {
                        Self::handle_typing_input(key.code, app);
                    }
                    _ => {}
                }
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
}
