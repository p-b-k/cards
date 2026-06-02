////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Test printing a layout
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{io, thread, time::Duration};

use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders, BorderType},
    layout::{Layout, Constraint, Direction},
    Terminal
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use seahaven::{deck::Deck, game::Table};

pub fn main() {
    let mut d = Deck::new();

    for _ in 0..100 {
        d.shuffle();
    }

    let t = Table::from(&mut d);

    // Set up terminal
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Table")
            .borders(Borders::ALL).border_type(BorderType::Rounded);
        f.render_widget(block, size);
    }).unwrap();

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).unwrap();
    terminal.show_cursor().unwrap();
}
