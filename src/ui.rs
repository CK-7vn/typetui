use std::{error::Error, io};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::{
    app::{AppResult, Screen, TypeTui},
    event::AppEventHandler,
    typingtest::TypingTest,
};

//our ui struct will handle all of the terminal stuff,
//getting us into raw, and taking us back out
#[derive(Debug)]
pub struct UI {
    pub app: TypeTui,
    terminal: Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    pub events: AppEventHandler,
}

impl UI {
    pub fn new(
        term: Terminal<CrosstermBackend<io::Stdout>>,
        events: AppEventHandler,
        app: TypeTui,
    ) -> Self {
        Self {
            app,
            terminal: term,
            events,
        }
    }
    pub async fn init(&mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        let _ = execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture);

        //initializes our async run loop
        let res = TypeTui::run_app(&mut self.terminal, &mut self.app, &mut self.events).await;

        //resotres terminal
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(), //returns a mutable reference to the backend for the
            //terminal
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        let _ = self.terminal.show_cursor(); //shows the cursor
        res.map(|_| ()).map_err(|e| e.into())
    }
}

pub fn render_title(frame: &mut Frame, chunk: Rect) -> AppResult<()> {
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
    Ok(())
}

pub fn render_typing_test(frame: &mut Frame, chunk: Rect, typing: &TypingTest) -> AppResult<()> {
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
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::QuadrantInside),
        )
        .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(paragraph, chunk);
    Ok(())
}

pub fn render_menu(frame: &mut Frame, chunk: Rect, selected_option: usize) -> AppResult<()> {
    let options = ["Test", "Login", "Stats", "Quit", "TestOpts"];

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
    Ok(())
}

pub fn render_stats(frame: &mut Frame, test: &TypingTest) -> AppResult<()> {
    let wpm = test.wpm;
    let popup_block = Block::default()
        .borders(Borders::ALL)
        .title("Stats")
        .title_alignment(ratatui::layout::Alignment::Center);

    let fmt_wpm = format!("{} WPM", wpm);

    let wpm_span = Text::styled(fmt_wpm, Style::default().fg(Color::LightMagenta));

    let paragraph = Paragraph::new(wpm_span)
        .block(popup_block)
        .alignment(ratatui::layout::Alignment::Center);

    let area = centered_rect(60, 25, frame.area());
    frame.render_widget(paragraph, area);
    Ok(())
}

pub fn render_quit(frame: &mut Frame) -> AppResult<()> {
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
    Ok(())
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

pub fn ui(f: &mut Frame, app: &TypeTui) -> crate::app::AppResult<()> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(90),
            Constraint::Percentage(10),
        ])
        .split(f.area());
    render_title(f, chunks[0])?;
    match app.current_screen {
        Screen::Main { selected_option } => {
            render_menu(f, chunks[1], selected_option)?;
        }
        Screen::TestOpts => {
            let _ = render_test_opts(f, app);
        }
        Screen::Typing => {
            render_typing_test(f, chunks[1], &app.typing)?;
        }
        Screen::Quit => {
            render_quit(f)?;
        }
        Screen::Stats => {
            render_stats(f, &app.typing)?;
        }
        _ => {}
    }
    Ok(())
}

pub fn render_test_opts(frame: &mut ratatui::Frame, app: &TypeTui) -> AppResult<()> {
    // Split the available area into two parts.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());

    // Left block: "How many words"
    let words_block = Block::default()
        .title("How many words")
        .borders(Borders::ALL);
    // Render the input string inside the block.
    let words_paragraph = Paragraph::new(app.test_opts.word_input.as_str())
        .block(words_block)
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(words_paragraph, chunks[0]);

    // Right block: "Seconds" options
    let seconds_block = Block::default().title("Seconds").borders(Borders::ALL);
    let seconds_options: Vec<ListItem> = app
        .test_opts
        .seconds_options
        .iter()
        .map(|sec| ListItem::new(format!("{} seconds", sec)))
        .collect();
    let seconds_list = List::new(seconds_options)
        .block(seconds_block)
        .highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol("-> ");
    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(app.test_opts.seconds_selected));
    frame.render_stateful_widget(seconds_list, chunks[1], &mut list_state);

    Ok(())
}

#[allow(dead_code)]
pub fn center_text(text: &str, width: u16) -> String {
    let effective_width = width.saturating_sub(3);
    let text_width = text.len() as u16;
    if text_width >= effective_width {
        return text.to_string();
    }
    let padding = (effective_width - text_width) / 2;
    let pad = " ".repeat(padding as usize);
    format!("{}{}{}", pad, text, pad)
}
