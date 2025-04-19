pub mod app;
pub mod db;
pub mod event;
pub mod typingtest;
pub mod ui;

use event::AppEventHandler;
#[allow(dead_code)]
use ratatui::prelude::CrosstermBackend;
use std::{error::Error, io};
use ui::UI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let terminal = ratatui::Terminal::new(backend)?;

    let app = app::TypeTui::new();
    let mut ui = UI::new(terminal, AppEventHandler::new(100), app);
    ui.init().await?;

    Ok(())
}
