////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Test printing a layout
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

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
    game::{NUM_COLS, NUM_FREE},
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
                // KeyCode::Char('p') => active_menu_item = MenuItem::Pets,
                // KeyCode::Char('a') => {
                //     add_random_pet_to_db().expect("can add new random pet");
                // }
                // KeyCode::Char('d') => {
                //     remove_pet_at_index(&mut pet_list_state).expect("can remove pet");
                // }
                // KeyCode::Down => {
                //     if let Some(selected) = pet_list_state.selected() {
                //         let amount_pets = read_db().expect("can fetch pet list").len();
                //         if selected >= amount_pets - 1 {
                //             pet_list_state.select(Some(0));
                //         } else {
                //             pet_list_state.select(Some(selected + 1));
                //         }
                //     }
                // }
                // KeyCode::Up => {
                //     if let Some(selected) = pet_list_state.selected() {
                //         let amount_pets = read_db().expect("can fetch pet list").len();
                //         if selected > 0 {
                //             pet_list_state.select(Some(selected - 1));
                //         } else {
                //             pet_list_state.select(Some(amount_pets - 1));
                //         }
                //     }
                // }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
