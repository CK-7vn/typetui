use std::{error::Error, io};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::{
    app::{AppResult, Screen, TestOptsFocus, TypeTui},
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
        .alignment(ratatui::layout::Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, chunk);
    Ok(())
}

pub fn render_menu(frame: &mut Frame, chunk: Rect, selected_option: usize) -> AppResult<()> {
    let options = ["Test", "Login", "History", "Quit", "TestOpts"];

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

pub fn render_history(
    frame: &mut Frame,
    area: Rect,
    history: &[(String, i32, i32, i32)],
    state: &mut ListState,
) -> AppResult<()> {
    let items: Vec<ListItem> = history
        .iter()
        .map(|(uname, wpm, word_count, time)| {
            let label = if *time > 0 {
                format!("{}s — {} WPM — {}", time, wpm, uname)
            } else {
                format!("{} words — {} WPM — {}", word_count, wpm, uname)
            };
            ListItem::new(label)
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .title("History")
        .title_alignment(Alignment::Center);

    let list = List::new(items)
        .block(block)
        .highlight_symbol(">> ")
        .highlight_style(Style::default().fg(Color::Yellow));

    frame.render_stateful_widget(list, area, state);
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

pub fn ui(f: &mut Frame, app: &mut TypeTui) -> crate::app::AppResult<()> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(90),
            Constraint::Percentage(10),
        ])
        .split(f.area());
    match app.current_screen {
        Screen::Main { selected_option: _ } => {}
        _ => {
            render_title(f, chunks[0])?;
        }
    }
    match app.current_screen {
        Screen::Main { selected_option } => {
            let main_panes = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(chunks[1]);
            render_splash(f, main_panes[0]);
            render_menu(f, main_panes[1], selected_option)?;
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
        Screen::History => {
            let area = centered_rect(50, 20, f.area());
            let _ = render_history(f, area, &app.history, &mut app.stats_list_state);
        }
        Screen::Stats => {
            render_stats(f, &app.typing)?;
        }
        Screen::Login => {
            let area = centered_rect(50, 20, f.area());
            render_login(f, area, &app.login_input)?;
        }
        Screen::Pause => {
            render_pause_menu(f, chunks[1], app.pause_selected)?;
        }
    }
    match app.current_screen {
        Screen::Typing => render_legend(f, chunks[2], "ESC to pause"),
        Screen::TestOpts => render_legend(f, chunks[2], "ESC to return to Main Menu q to quit"),
        Screen::Main { selected_option: _ } => render_legend(
            f,
            chunks[2],
            " \u{2191}/\u{2193} to move  •  Enter to select",
        ),
        _ => render_legend(
            f,
            chunks[2],
            "ESC to return to test * q to quit \u{2191}/\u{2193} to move  •  Enter to select ",
        ),
    };
    Ok(())
}
fn render_legend(frame: &mut Frame, area: Rect, text: &str) {
    let p = Paragraph::new(text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    frame.render_widget(p, area);
}
fn render_splash(frame: &mut Frame, area: Rect) {
    //let lines = [
    //    "▄▄▄█████▓▓██   ██▓ ██▓███  ▓█████ ▄▄▄█████▓ █    ██  ██▓",
    //    "▓  ██▒ ▓▒ ▒██  ██▒▓██░  ██▒▓█   ▀ ▓  ██▒ ▓▒ ██  ▓██▒▓██▒",
    //    "▒ ▓██░ ▒░  ▒██ ██░▓██░ ██▓▒▒███   ▒ ▓██░ ▒░▓██  ▒██░▒██▒",
    //    "░ ▓██▓ ░   ░ ▐██▓░▒██▄█▓▒ ▒▒▓█  ▄ ░ ▓██▓ ░ ▓▓█  ░██░░██░",
    //    "  ▒██▒ ░   ░ ██▒▓░▒██▒ ░  ░░▒████▒  ▒██▒ ░ ▒▒█████▓ ░██░",
    //    "  ▒ ░░      ██▒▒▒ ▒▓▒░ ░  ░░░ ▒░ ░  ▒ ░░   ░▒▓▒ ▒ ▒ ░▓  ",
    //    "    ░     ▓██ ░▒░ ░▒ ░      ░ ░  ░    ░    ░░▒░ ░ ░  ▒ ░",
    //    "  ░       ▒ ▒ ░░  ░░          ░     ░       ░░░ ░ ░  ▒ ░",
    //    "          ░ ░                 ░  ░            ░      ░  ",
    //    "          ░ ░                                            ",
    //];
    let lines = [
        "                                         ,ggggggggggggggg                  ",
        "   I8                                   dP\"\"\"\"\"\"88\"\"\"\"\"\"\"                  ",
        "   I8                                   Yb,_    88                         ",
        "88888888                                 `\"\"    88                     gg  ",
        "   I8                                           88                     \"\"  ",
        "   I8    gg     gg  gg,gggg,     ,ggg,          88        gg      gg   gg  ",
        "   I8    I8     8I  I8P\"  \"Yb   i8\" \"8i         88        I8      8I   88  ",
        "  ,I8,   I8,   ,8I  I8'    ,8i  I8, ,8I   gg,   88        I8,    ,8I   88  ",
        " ,d88b, ,d8b, ,d8I ,I8 _  ,d8'  `YbadP'    \"Yb,,8P       ,d8b,  ,d8b,_,88,_",
        " 8P\"\"Y8 P\"\"Y88P\"888PI8 YY88888P888P\"Y888     \"Y8P'       8P'\"Y88P\"`Y88P\"\"Y8",
        "              ,d8I' I8                                                     ",
        "            ,dP'8I  I8                                                     ",
        "           ,8\"  8I  I8                                                     ",
        "           I8   8I  I8                                                     ",
        "           `8, ,8I  I8                                                     ",
        "            `Y8P\"   I8                                                     ",
    ];
    let splash = lines.join("\n");
    let p = Paragraph::new(Text::raw(splash))
        .style(Style::default().fg(Color::Blue))
        .alignment(Alignment::Center);
    frame.render_widget(p, area);
}
pub fn render_pause_menu(frame: &mut Frame, area: Rect, selected: usize) -> AppResult<()> {
    let options = ["Restart Test", "New Test", "Main Menu", "Quit"];
    let items: Vec<ListItem> = options.iter().map(|&s| ListItem::new(s)).collect();
    let block = Block::default()
        .title("Paused")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center);
    let list = List::new(items)
        .block(block)
        .highlight_symbol("» ")
        .highlight_style(Style::default().fg(Color::LightBlue));
    let mut state = ListState::default();
    state.select(Some(selected));
    frame.render_stateful_widget(list, area, &mut state);
    Ok(())
}

pub fn render_login(frame: &mut Frame, area: Rect, user_input: &str) -> AppResult<()> {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Login")
        .title_alignment(Alignment::Center);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Length(1)])
        .margin(1)
        .split(area);

    let prompt = Paragraph::new(Text::raw("Enter your username")).alignment(Alignment::Left);

    frame.render_widget(block.clone(), area);

    let user_line = Paragraph::new(user_input)
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center);
    frame.render_widget(prompt, chunks[0]);
    frame.render_widget(user_line, chunks[1]);

    Ok(())
}

pub fn render_test_opts(frame: &mut ratatui::Frame, app: &TypeTui) -> AppResult<()> {
    let popup_area = centered_rect(60, 50, frame.area());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(popup_area);
    let words_border_style = if let TestOptsFocus::Words = app.test_opts.focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    // top block: "How many words"
    let words_block = Block::default()
        .title("How many words")
        .borders(Borders::ALL)
        .border_style(words_border_style);

    // render the input string inside the block.
    let words_paragraph = Paragraph::new(app.test_opts.word_input.as_str())
        .block(words_block)
        .alignment(ratatui::layout::Alignment::Center)
        .wrap(Wrap { trim: true });
    frame.render_widget(words_paragraph, chunks[0]);

    //conditional rendering here based on whatt he user has selected
    let seconds_border_style = if let TestOptsFocus::Seconds = app.test_opts.focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    // bottom block: "Seconds" options
    let seconds_block = Block::default()
        .title("Seconds")
        .borders(Borders::ALL)
        .border_style(seconds_border_style);
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
