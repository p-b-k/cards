////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Test printing a layout
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use log::{info, warn};
use tui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
};

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use seahaven::{
    deck::Deck,
    game::{Location, NUM_COLS, NUM_FREE},
    tui::tableau::{Mode, Tableau, TableauWidget},
};

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Initialize deck and table
    let mut deck = Deck::new();
    for _ in 0..100 {
        deck.shuffle_once();
    }
    let mut tableau = Tableau::new(&mut deck);

    // Start setting up input mode and channels
    enable_raw_mode().expect("can run in raw mode");
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(2000);

    // Spawn the input loop
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    // Create the terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Loop over the input and redraw the screen
    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Length(32),
                        Constraint::Min(2),
                        // Constraint::Length(1),
                        // Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(size);

            let tab_widget = TableauWidget {};
            rect.render_stateful_widget(tab_widget, chunks[1], &mut tableau);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }

                KeyCode::Char('h') => match tableau.mode {
                    Mode::Free(i) => {
                        if i > 0 {
                            tableau.mode = Mode::Free(i - 1);
                        }
                    }
                    Mode::Build(i) => {
                        if i > 0 {
                            tableau.mode = Mode::Build(i - 1);
                        }
                    }
                },

                KeyCode::Char('l') => match tableau.mode {
                    Mode::Free(i) => {
                        if i < (NUM_FREE as u8 - 1) {
                            tableau.mode = Mode::Free(i + 1);
                        }
                    }
                    Mode::Build(i) => {
                        if i < (NUM_COLS as u8 - 1) {
                            tableau.mode = Mode::Build(i + 1);
                        }
                    }
                },

                KeyCode::Char('j') => match tableau.mode {
                    Mode::Free(i) => {
                        tableau.mode = Mode::Build(3 + i);
                    }
                    _ => {}
                },

                KeyCode::Char('k') => match tableau.mode {
                    Mode::Build(i) => {
                        if i < 3 {
                            tableau.mode = Mode::Free(0);
                        } else if i > 7 {
                            tableau.mode = Mode::Free(3);
                        } else {
                            tableau.mode = Mode::Free(i - 3);
                        }
                    }
                    _ => {}
                },

                KeyCode::Char(' ') => tableau.tab.retire_all(),

                KeyCode::Char('u') => match tableau.mode {
                    Mode::Free(_) => {}
                    Mode::Build(i) => match tableau.tab.next_free() {
                        Some(f) => {
                            tableau
                                .tab
                                .move_card(Location::Builds(i), Location::Free(f));
                        }
                        None => {
                            info!("No available free slots");
                        }
                    },
                },

                KeyCode::Enter => match tableau.mode {
                    Mode::Free(i) => match &tableau.tab.free[i as usize] {
                        None => {
                            warn!("No card in fee cell {i}");
                        }
                        Some(c) => match tableau.tab.find_build_home(&c) {
                            Some(j) => {
                                tableau
                                    .tab
                                    .move_card(Location::Free(i as u8), Location::Builds(j as u8));
                            }
                            None => {
                                warn!("No build column to move {c:?} to");
                            }
                        },
                    },
                    Mode::Build(i) => match tableau.tab.next_free() {
                        Some(f) => {
                            tableau
                                .tab
                                .move_card(Location::Builds(i), Location::Free(f));
                        }
                        None => {
                            info!("No available free slots");
                        }
                    },
                },

                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
