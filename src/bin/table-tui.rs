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
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
};

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use seahaven::{deck::Deck, game::Table as Tableau};

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize deck and table
    let mut deck = Deck::new();

    for _ in 0..100 {
        deck.shuffle();
    }

    let t = Tableau::from(&mut deck);

    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
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

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = vec!["New", "Retire", "Retire All", "Build", "Undo", "Quit"];

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let tableau = Paragraph::new("Seahaven Towers")
                .style(Style::default().fg(Color::Yellow))
                .style(Style::default().bg(Color::Rgb(0, 43, 0)))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Tableau")
                        .border_type(BorderType::Rounded),
                );

            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .block(Block::default().borders(Borders::NONE))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);
            // let pets_chunks = Layout::default()
            //     .direction(Direction::Horizontal)
            //     .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            //     .split(chunks[1]);
            // let (left, right) = render_pets(&deck);
            // rect.render_stateful_widget(left, pets_chunks[0], &mut pet_list_state);
            // rect.render_widget(right, pets_chunks[1]);
            rect.render_widget(tableau, chunks[1]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }

                // KeyCode::Char('h') => active_menu_item = MenuItem::Home,
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

// fn render_pets<'a>(tableau: &Tableau) {
//     let pets = Block::default()
//         .borders(Borders::ALL)
//         .style(Style::default().fg(Color::White))
//         .title("Pets")
//         .border_type(BorderType::Plain);

//     let selected_pet = pet_list
//         .get(
//             pet_list_state
//                 .selected()
//                 .expect("there is always a selected pet"),
//         )
//         .expect("exists")
//         .clone();

//     let list = List::new(items).block(pets).highlight_style(
//         Style::default()
//             .bg(Color::Yellow)
//             .fg(Color::Black)
//             .add_modifier(Modifier::BOLD),
//     );

//     let pet_detail = Table::new(vec![Row::new(vec![
//         Cell::from(Span::raw(selected_pet.id.to_string())),
//         Cell::from(Span::raw(selected_pet.name)),
//         Cell::from(Span::raw(selected_pet.category)),
//         Cell::from(Span::raw(selected_pet.age.to_string())),
//         Cell::from(Span::raw(selected_pet.created_at.to_string())),
//     ])])
//     .header(Row::new(vec![
//         Cell::from(Span::styled(
//             "ID",
//             Style::default().add_modifier(Modifier::BOLD),
//         )),
//         Cell::from(Span::styled(
//             "Name",
//             Style::default().add_modifier(Modifier::BOLD),
//         )),
//         Cell::from(Span::styled(
//             "Category",
//             Style::default().add_modifier(Modifier::BOLD),
//         )),
//         Cell::from(Span::styled(
//             "Age",
//             Style::default().add_modifier(Modifier::BOLD),
//         )),
//         Cell::from(Span::styled(
//             "Created At",
//             Style::default().add_modifier(Modifier::BOLD),
//         )),
//     ]))
//     .block(
//         Block::default()
//             .borders(Borders::ALL)
//             .style(Style::default().fg(Color::White))
//             .title("Detail")
//             .border_type(BorderType::Plain),
//     )
//     .widths(&[
//         Constraint::Percentage(5),
//         Constraint::Percentage(20),
//         Constraint::Percentage(20),
//         Constraint::Percentage(5),
//         Constraint::Percentage(20),
//     ]);

//     (list, pet_detail)
// }
