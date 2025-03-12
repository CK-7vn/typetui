/// what would be really cool is a 1v1 terminal typing race, dont forget this
pub mod app;
pub mod typingtest;
pub mod ui;

use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::{
    crossterm::event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind},
    prelude::{Backend, CrosstermBackend},
};
use std::{error::Error, io};

use ratatui::crossterm::execute;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?; // enable some raws mode
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture);

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;
    let mut app = app::TypeTui::new();
    let res = app::run_app(&mut terminal, &mut app);
    //resotres terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(), //returns a mutable reference to the backend for the
        //terminal
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    let _ = terminal.show_cursor(); //shows the cursor
    if let Ok(do_print) = res {
        if do_print {
            println!("We should figure this out")
        } else if let Err(err) = res {
            println!("{err:?}");
        }
    }
    Ok(())
}
