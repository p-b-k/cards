////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Seahaven Tableeau TUI
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use log::error;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::StatefulWidget,
};

use crate::{
    cards::{Card, NUM_SUITS, Rank, Suit},
    deck::Deck,
    game::{NUM_COLS, NUM_FREE, Table},
};

#[derive(Clone, Debug, PartialEq)]
pub enum Mode {
    Free(u8),
    Build(u8),
}

#[derive(Clone, Debug)]
pub struct Tableau {
    pub tab: Table,
    pub mode: Mode,
}

impl Tableau {
    pub fn new(deck: &mut Deck) -> Tableau {
        // let mut deck = Deck::new();

        Tableau {
            mode: Mode::Build(5),
            tab: Table::from(deck),
        }
    }
}

pub struct TableauWidget {}

const BOARD_MARGIN_LEFT: u16 = 1;
const BOARD_MARGIN_RIGHT: u16 = 1;
const BOARD_MARGIN_TOP: u16 = 1;
const BOARD_MARGIN_BOTTOM: u16 = 1;
const CARD_WIDTH: u16 = 3;
const CARD_GAP: u16 = 1;
const CARD_SPAN: u16 = CARD_WIDTH + CARD_GAP;

enum DrawResult {
    Ok,
    Err(String),
}

impl StatefulWidget for TableauWidget {
    type State = Tableau;

    fn render(self, area: Rect, buff: &mut Buffer, state: &mut Self::State) {
        // Set bg
        let default_style = Style::default()
            .bg(Color::Rgb(0, 43, 0))
            .fg(Color::Rgb(0xf0, 0xe4, 0))
            .add_modifier(Modifier::BOLD);

        for r in 0..area.height {
            for c in 0..(BOARD_MARGIN_LEFT + (NUM_COLS * CARD_SPAN) + BOARD_MARGIN_RIGHT) {
                buff.get_mut(c, r).set_style(default_style);
            }
        }

        match draw_free_cells(area, buff, state) {
            DrawResult::Err(s) => {
                error!("Error: {s}");
                // panic!("Unable to draw free cells");
            }
            DrawResult::Ok => {}
        };

        match draw_found_cells(area, buff, state) {
            DrawResult::Err(s) => {
                error!("Error: {s}");
                // panic!("Unable to draw free cells");
            }
            DrawResult::Ok => {}
        };

        match draw_build_cells(area, buff, state) {
            DrawResult::Err(s) => {
                error!("Error: {s}");
                // panic!("Unable to draw free cells");
            }
            DrawResult::Ok => {}
        };

        match draw_hrule_cells(area, buff, state) {
            DrawResult::Err(s) => {
                error!("Error: {s}");
                // panic!("Unable to draw free cells");
            }
            DrawResult::Ok => {}
        };

        match draw_free_border_cells(area, buff, state) {
            DrawResult::Err(s) => {
                error!("Error: {s}");
                // panic!("Unable to draw free cells");
            }
            DrawResult::Ok => {}
        };

        match draw_table_border_cells(area, buff, state) {
            DrawResult::Err(s) => {
                error!("Error: {s}");
                // panic!("Unable to draw free cells");
            }
            DrawResult::Ok => {}
        };
    }
}

fn draw_card_at(
    buff: &mut Buffer,
    card: &Option<Card>,
    row: u16,
    col: u16,
    empty: &str,
) -> DrawResult {
    match card {
        None => {
            buff.get_mut(col + 0, row)
                .set_char(empty.chars().nth(0).expect("String Back To Short (0)"));
            buff.get_mut(col + 1, row)
                .set_char(empty.chars().nth(1).expect("String Back To Short (1)"));
            buff.get_mut(col + 2, row)
                .set_char(empty.chars().nth(2).expect("String Back To Short (2)"));
        }
        Some(c) => {
            let fg = match c.suit {
                Suit::Spade => Color::Black,
                Suit::Club => Color::Black,
                _ => Color::Red,
            };

            for i in 0..3 {
                buff.get_mut(col + i, row).set_style(
                    tui::style::Style::default()
                        .bg(Color::White)
                        .fg(fg)
                        .add_modifier(Modifier::BOLD),
                );
            }

            buff.get_mut(col + 0, row).set_char(c.suit.as_char());
            buff.get_mut(col + 1, row).set_char(match c.rank {
                Rank::C10 => '1',
                _ => ' ',
            });
            buff.get_mut(col + 2, row).set_char(match c.rank {
                Rank::Ace => 'A',
                Rank::C2 => '2',
                Rank::C3 => '3',
                Rank::C4 => '4',
                Rank::C5 => '5',
                Rank::C6 => '6',
                Rank::C7 => '7',
                Rank::C8 => '8',
                Rank::C9 => '9',
                Rank::C10 => '0',
                Rank::Jack => 'J',
                Rank::Queen => 'Q',
                Rank::King => 'K',
            });
        }
    }
    DrawResult::Ok
}

fn draw_free_cells(area: Rect, buff: &mut Buffer, state: &mut Tableau) -> DrawResult {
    for i in 0..NUM_FREE {
        let row = BOARD_MARGIN_TOP + 1;
        let col = BOARD_MARGIN_LEFT + (3 * CARD_SPAN) + (i as u16 * CARD_SPAN);

        draw_card_at(buff, &state.tab.free[i as usize], row, col, "[ ]");
    }
    DrawResult::Ok
}
fn draw_found_cells(area: Rect, buff: &mut Buffer, state: &mut Tableau) -> DrawResult {
    for s in 0..NUM_SUITS {
        let cnt = if s < 2 { 0 } else { 6 };
        let row = BOARD_MARGIN_TOP + 1;
        let col = BOARD_MARGIN_LEFT + ((s + cnt) * CARD_SPAN);

        if state.tab.found[s as usize] > 0 {
            let c = Card::from_index(state.tab.found[s as usize] - 1)
                .expect("Unable to get card from index");
            draw_card_at(buff, &Some(c), row, col, "***");
        } else {
            buff.get_mut(col, row).set_char('[');
            buff.get_mut(col + 1, row)
                .set_char(Suit::from_index(s as u8).unwrap().as_char());
            buff.get_mut(col + 2, row).set_char(']');
        }
    }

    DrawResult::Err("draw_found_cells: Not Implemented".to_string())
}
fn draw_build_cells(area: Rect, buff: &mut Buffer, state: &mut Tableau) -> DrawResult {
    DrawResult::Err("draw_build_cells: Not Implemented".to_string())
}
fn draw_hrule_cells(area: Rect, buff: &mut Buffer, state: &mut Tableau) -> DrawResult {
    DrawResult::Err("draw_hrule_cells: Not Implemented".to_string())
}
fn draw_free_border_cells(area: Rect, buff: &mut Buffer, state: &mut Tableau) -> DrawResult {
    DrawResult::Err("draw_free_border_cells: Not Implemented".to_string())
}
fn draw_table_border_cells(area: Rect, buff: &mut Buffer, state: &mut Tableau) -> DrawResult {
    DrawResult::Err("draw_table_border_cells: Not Implemented".to_string())
}
