////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Test printing a layout
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::io;
use tui::{backend::CrosstermBackend, Terminal};

use seahaven::{deck::Deck, game::Table};

pub fn main() {
    let mut d = Deck::new();

    for _ in 0..100 {
        d.shuffle();
    }

    let t = Table::from(&mut d);

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend);
}
