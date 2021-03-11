#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::*;
use tui::layout::*;


fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10)
                ].as_ref()
            )
            .split(f.size());

        let block = Block::default()
            .title("New")
            .borders(Borders::ALL);
        f.render_widget(block, chunks[0]);

        let block = Block::default()
            .title("Open")
            .borders(Borders::ALL);
        f.render_widget(block, chunks[1]);

        let block = Block::default()
            .title("Settings")
            .borders(Borders::ALL);
        f.render_widget(block, chunks[2]);

    })
}
