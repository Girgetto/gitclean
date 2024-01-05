mod navigator;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Stylize, Terminal},
    style::{Color, Style},
    widgets::{Block, Borders, Row, Table, TableState},
};
use std::io::{stdout, Result};
use navigator::find_git_repos;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let path = std::env::current_dir().unwrap().to_str().unwrap().to_string();
    let git_dirs = find_git_repos(&path);
    let mut table_state = TableState::default();
    table_state.select(Some(0));
    let mut deleted_dirs: Vec<usize> = Vec::new();

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(frame.size());

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
                .block(Block::default().title("GitClean").borders(Borders::ALL))
                .highlight_style(Style::new().reversed())
                .bg(Color::Black)
                .highlight_symbol(">>");

            frame.render_stateful_widget(table, chunks[0], &mut table_state);
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
