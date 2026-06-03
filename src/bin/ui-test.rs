////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Try a second layout
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{io, thread, time::{Instant, Duration}};

use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders, BorderType},
    layout::{Layout, Constraint, Direction},
    Terminal
};

use std::sync::{mpsc, mpsc::{Sender, Receiver }};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use seahaven::{deck::Deck, game::Table};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up deck
    let mut d = Deck::new();

    for _ in 0..100 {
        d.shuffle();
    }

    let t = Table::from(&mut d);

    // Set up terminal
    enable_raw_mode().unwrap();
    let (tx, rx) : (Sender<CEvent>, Receiver<CEvent>) = mpsc::channel();
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

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

        //     let copyright = Paragraph::new("pet-CLI 2020 - all rights reserved")
        //         .style(Style::default().fg(Color::LightCyan))
        //         .alignment(Alignment::Center)
        //         .block(
        //             Block::default()
        //                 .borders(Borders::ALL)
        //                 .style(Style::default().fg(Color::White))
        //                 .title("Copyright")
        //                 .border_type(BorderType::Plain),
        //         );

        //     let menu = menu_titles
        //         .iter()
        //         .map(|t| {
        //             let (first, rest) = t.split_at(1);
        //             Spans::from(vec![
        //                 Span::styled(
        //                     first,
        //                     Style::default()
        //                         .fg(Color::Yellow)
        //                         .add_modifier(Modifier::UNDERLINED),
        //                 ),
        //                 Span::styled(rest, Style::default().fg(Color::White)),
        //             ])
        //         })
        //         .collect();

        //     let tabs = Tabs::new(menu)
        //         .select(active_menu_item.into())
        //         .block(Block::default().title("Menu").borders(Borders::ALL))
        //         .style(Style::default().fg(Color::White))
        //         .highlight_style(Style::default().fg(Color::Yellow))
        //         .divider(Span::raw("|"));

        //     rect.render_widget(tabs, chunks[0]);
        //     match active_menu_item {
        //         MenuItem::Home => rect.render_widget(render_home(), chunks[1]),
        //         MenuItem::Pets => {
        //             let pets_chunks = Layout::default()
        //                 .direction(Direction::Horizontal)
        //                 .constraints(
        //                     [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
        //                 )
        //                 .split(chunks[1]);
        //             let (left, right) = render_pets(&pet_list_state);
        //             rect.render_stateful_widget(left, pets_chunks[0], &mut pet_list_state);
        //             rect.render_widget(right, pets_chunks[1]);
        //         }
        //     }
        //     rect.render_widget(copyright, chunks[2]);
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
    
    // // Set up terminal
    // enable_raw_mode().unwrap();
    // let mut stdout = io::stdout();
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    // let backend = CrosstermBackend::new(stdout);
    // let mut terminal = Terminal::new(backend).unwrap();

    // terminal.draw(|f| {
    //     let size = f.size();
    //     let block = Block::default()
    //         .title("Table")
    //         .borders(Borders::ALL).border_type(BorderType::Rounded);
    //     f.render_widget(block, size);
    // }).unwrap();

    // thread::sleep(Duration::from_millis(5000));

    // // restore terminal
    // disable_raw_mode().unwrap();
    // execute!(
    //     terminal.backend_mut(),
    //     LeaveAlternateScreen,
    //     DisableMouseCapture
    // ).unwrap();
    // terminal.show_cursor().unwrap();
}

