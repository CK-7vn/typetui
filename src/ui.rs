use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{self, Event};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::app::{handle_menu_input, CurrentScreen, Menu, TypeTui};

pub fn ui(frame: &mut Frame, app: &TypeTui) {
    // we now have 3 chunks a title chunk at chunk[0], main chunk at chunk[1] and footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(70),
            Constraint::Percentage(20),
        ])
        .split(frame.area());

    render_title(frame, chunks[0]);
    match app.menu {
        Menu::Main => render_menu(frame, chunks[1], app),
        Menu::Login => todo!(),
        Menu::Stats => todo!(),
        Menu::Type => todo!(),
        Menu::Quit => render_quit(frame, app),
    }
}

pub fn render_menu(frame: &mut Frame, chunk: Rect, app: &TypeTui) {
    let options = ["Test", "Login", "Stats", "Quit"];
    let items: Vec<ListItem> = options.iter().map(|&s| ListItem::new(s)).collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Main Menu"))
        .highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol("-> ");
    let mut list_state = ListState::default();
    list_state.select(Some(app.selected_option));
    frame.render_stateful_widget(list, chunk, &mut list_state);
}

pub fn render_quit(frame: &mut Frame, app: &TypeTui) {
    let popup_block = Block::default()
        .title("y/n")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::LightRed));

    let exit_text = Text::styled(
        "Would you like to quit (y/n)",
        Style::default().fg(Color::Red),
    );

    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    let area = centered_rect(60, 25, frame.area());
    frame.render_widget(exit_paragraph, area);
}
pub fn render_title(frame: &mut Frame, chunk: Rect) {
    let title_chunk = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "TypeTui",
        Style::default().fg(Color::LightGreen),
    ))
    .block(title_chunk)
    .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(title.clone(), chunk);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    //Splitting into width wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
