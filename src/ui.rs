use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::{
    app::{Screen, TypeTui},
    typingtest::TypingTest,
};

pub fn ui(frame: &mut Frame, app: &TypeTui) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(70),
            Constraint::Percentage(20),
        ])
        .split(frame.area());

    render_title(frame, chunks[0]);
    match app.current_screen {
        Screen::Main { selected_option } => {
            let popup_area = centered_rect(60, 50, frame.area());
            render_menu(frame, popup_area, selected_option);
        }
        Screen::Login => todo!(),
        Screen::Stats => todo!(),
        Screen::Typing => render_typing_test(frame, chunks[1], &app.typing),
        Screen::Quit => render_quit(frame, app),
    }
}
pub fn render_menu(frame: &mut Frame, chunk: Rect, selected_option: usize) {
    let options = ["Test", "Login", "Stats", "Quit"];

    let items: Vec<ListItem> = options.iter().map(|&s| ListItem::new(s)).collect();

    let popup_block = Block::default()
        .borders(Borders::ALL)
        .title("Main Menu")
        .title_alignment(ratatui::layout::Alignment::Center);
    let list = List::new(items)
        .block(popup_block)
        .highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol("-> ");
    let mut list_state = ListState::default();
    list_state.select(Some(selected_option));
    frame.render_stateful_widget(list, chunk, &mut list_state);
}

pub fn render_quit(frame: &mut Frame, app: &TypeTui) {
    let popup_block = Block::default()
        .title("y/n")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::LightRed));

    let exit_text = Text::styled(
        "Would you like to quit (y/n)",
        Style::default().fg(Color::Gray),
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
pub fn render_typing_test(frame: &mut Frame, chunk: Rect, typing: &TypingTest) {
    let test_chars: Vec<char> = typing.test_text.chars().collect();
    let input_chars: Vec<char> = typing.user_input.chars().collect();

    let spans: Vec<Span> = test_chars
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            if i < input_chars.len() {
                if input_chars[i] == c {
                    Span::styled(c.to_string(), Style::default().fg(Color::Green))
                } else {
                    Span::styled(c.to_string(), Style::default().fg(Color::Red))
                }
            } else {
                Span::raw(c.to_string())
            }
        })
        .collect();

    let paragraph = Paragraph::new(Line::from(spans))
        .block(Block::default().borders(Borders::ALL))
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(paragraph, chunk);
}

#[allow(dead_code)]
fn center_text(text: &str, width: u16) -> String {
    let effective_width = width.saturating_sub(3);
    let text_width = text.len() as u16;
    if text_width >= effective_width {
        return text.to_string();
    }
    let padding = (effective_width - text_width) / 2;
    let pad = " ".repeat(padding as usize);
    format!("{}{}{}", pad, text, pad)
}
