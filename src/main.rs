mod navigator;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use navigator::find_git_repos;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Stylize, Terminal},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Row, Table, TableState},
};
use std::io::{stdout, Result};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let path = Arc::new(
        std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    );
    let git_dirs = Arc::new(Mutex::new(Vec::new()));

    let path_clone = Arc::clone(&path);
    let git_dirs_clone = Arc::clone(&git_dirs);

    thread::spawn(move || {
        *git_dirs_clone.lock().unwrap() = find_git_repos(&*path_clone);
    });

    while git_dirs.lock().unwrap().is_empty() {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(frame.size());

            let text = Text::styled("Reading Paths...", Style::default().fg(Color::White));

            let paragraph = Paragraph::new(text)
                .block(Block::default())
                .style(Style::default().fg(Color::White))
                .alignment(ratatui::layout::Alignment::Center);

            frame.render_widget(paragraph, chunks[0]);
        })?;

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    let git_dirs = Arc::try_unwrap(git_dirs).unwrap().into_inner().unwrap();

    let mut table_state = TableState::default();
    table_state.select(Some(0));
    let mut deleted_dirs: Vec<usize> = Vec::new();

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(frame.size());

            let letters = vec![
                "_____ _ _    _____ _                  ",
                "/ ____(_) |  / ____| |                 ",
                "| |  __ _| |_| |    | | ___  __ _ _ __  ",
                "| | |_ | | __| |    | |/ _ \\/ _` | '_ \\ ",
                "| |__| | | |_| |____| |  __/ (_| | | | |",
                "\\_____|_|\\__|\\_____|_|\\___|\\__,_|_| |_|",
            ];

            frame.render_widget(
                Paragraph::new(Text::from(letters.join("\n")))
                    .block(Block::default())
                    .style(Style::default().fg(Color::White))
                    .alignment(ratatui::layout::Alignment::Center),
                chunks[0],
            );

            let text = Text::styled(
                "q: quit | d: delete | up/down: navigate",
                Style::default().fg(Color::White),
            );

            let paragraph = Paragraph::new(text)
                .block(Block::default())
                .style(Style::default().fg(Color::White))
                .alignment(ratatui::layout::Alignment::Center);

            frame.render_widget(paragraph, chunks[2]);

            let rows = git_dirs
                .iter()
                .map(|path| {
                    Row::new(vec![if deleted_dirs
                        .contains(&git_dirs.iter().position(|x| x == path).unwrap())
                    {
                        format!("[deleted] {}", path.display())
                    } else {
                        format!("{}", path.display())
                    }])
                })
                .collect::<Vec<_>>();

            let widths = [Constraint::Percentage(100)].as_ref();

            let table = Table::new(rows, widths)
                .column_spacing(1)
                .style(Style::new().white())
                .header(
                    Row::new(vec!["Directories"])
                        .style(Style::new().bold())
                        .bottom_margin(1),
                )
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(Style::new().reversed())
                .bg(Color::Black)
                .highlight_symbol(">>");

            frame.render_stateful_widget(table, chunks[1], &mut table_state);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                let current_index = table_state.selected().unwrap();
                match (key.kind, key.code) {
                    (KeyEventKind::Press, KeyCode::Char('q')) => {
                        break;
                    }
                    (KeyEventKind::Press, KeyCode::Char('d')) => {
                        let selected = git_dirs.get(current_index).unwrap();
                        let _ = std::fs::remove_dir_all(selected);

                        deleted_dirs.push(current_index);
                    }
                    (KeyEventKind::Press, KeyCode::Down) => {
                        if current_index < git_dirs.len() - 1 {
                            table_state.select(Some(current_index + 1));
                        }
                    }
                    (KeyEventKind::Press, KeyCode::Up) => {
                        if current_index > 0 {
                            table_state.select(Some(current_index - 1));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
