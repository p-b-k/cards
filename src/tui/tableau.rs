////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Seahaven Tableeau TUI
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use log::{debug, error};
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

const FREE_INDICATOR_CHARS: [char; 3] = [
    ' ',
    char::from_u32(0x25c9).expect("Unable to get indicator char"),
    ' ',
];

const BOARD_INDICATOR_CHARS: [char; 3] = [
    ' ',
    char::from_u32(0x25c9).expect("Unable to get indicator char"),
    ' ',
];

fn bg_color() -> Color {
    Color::Rgb(0x33, 0xA3, 0x48)
}

fn fg_color() -> Color {
    Color::Rgb(0xd0, 0xc4, 0x33)
}

fn dl_color() -> Color {
    Color::Rgb(0xdf, 0x83, 0x12)
}

fn hl_color() -> Color {
    Color::Rgb(0xFF, 0xe0, 0x12)
}

fn cr_color() -> Color {
    Color::Rgb(0xf3, 0x30, 0x22)
}

impl StatefulWidget for TableauWidget {
    type State = Tableau;

    fn render(self, area: Rect, buff: &mut Buffer, state: &mut Self::State) {
        // debug!("Render: Mode = {:?}", state.mode);
        // Set bg
        let default_style = Style::default()
            .bg(bg_color())
            .fg(fg_color())
            .add_modifier(Modifier::BOLD);

        let tab_height =
            BOARD_MARGIN_BOTTOM + BOARD_MARGIN_TOP + 4 as u16 + state.tab.max_build() as u16;

        for r in 0..tab_height {
            for c in 0..(BOARD_MARGIN_LEFT + (NUM_COLS * CARD_SPAN) + BOARD_MARGIN_RIGHT - CARD_GAP)
            {
                buff.get_mut(c, r as u16).set_style(default_style);
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

fn draw_free_cells(_area: Rect, buff: &mut Buffer, state: &mut Tableau) -> DrawResult {
    for i in 0..NUM_FREE {
        let row = BOARD_MARGIN_TOP;
        let col = BOARD_MARGIN_LEFT + (3 * CARD_SPAN) + (i as u16 * CARD_SPAN);

        draw_card_at(buff, &state.tab.free[i as usize], row, col, "[ ]");
    }
    DrawResult::Ok
}

fn draw_found_cells(_area: Rect, buff: &mut Buffer, state: &mut Tableau) -> DrawResult {
    for s in 0..NUM_SUITS {
        let cnt = if s < 2 { 0 } else { 6 };
        let row = BOARD_MARGIN_TOP;
        let col = BOARD_MARGIN_LEFT + ((s + cnt) * CARD_SPAN);

        if state.tab.found[s as usize] > 0 {
            // let c = Card::from_index(state.tab.found[s as usize] - 1)
            //     .expect("Unable to get card from index");
            let c = Card {
                rank: Rank::from_index(state.tab.found[s as usize] - 1).unwrap(),
                suit: Suit::from_index(s as u8).unwrap(),
            };
            draw_card_at(buff, &Some(c), row, col, "***");
        } else {
            let suit = Suit::from_index(s as u8).unwrap();
            let suit_color = match suit {
                Suit::Club => Color::Black,
                Suit::Spade => Color::Black,
                _ => Color::Red,
            };
            buff.get_mut(col, row).set_char('[');
            buff.get_mut(col + 1, row).set_char(suit.as_char());
            buff.get_mut(col + 1, row)
                .set_style(Style::default().bg(bg_color()).fg(suit_color));
            buff.get_mut(col + 2, row).set_char(']');
        }
    }

    DrawResult::Ok
}

fn draw_build_cells(_area: Rect, buff: &mut Buffer, state: &mut Tableau) -> DrawResult {
    for column in 0..NUM_COLS {
        for c in 0..state.tab.blds[column as usize].len() {
            let row = BOARD_MARGIN_TOP + 3 as u16 + c as u16;
            let col = BOARD_MARGIN_LEFT + (column * CARD_SPAN);
            draw_card_at(
                buff,
                &Some(state.tab.blds[column as usize][c].clone()),
                row,
                col,
                "    ",
            );
        }
    }

    DrawResult::Ok
}

fn draw_hrule_cells(_area: Rect, buff: &mut Buffer, state: &mut Tableau) -> DrawResult {
    let row = BOARD_MARGIN_TOP + 2;
    let border_char = char::from_u32(0x2501).expect("Unable to get HR char");
    let mut style = Style::default().bg(bg_color()).fg(dl_color());
    let mut cursor: Option<u16> = None;

    match state.mode {
        Mode::Build(i) => {
            // style.fg(hl_color());
            style = Style::default().bg(bg_color()).fg(hl_color());
            cursor = Some(i as u16);
        }
        _ => {}
    }

    for i in 0..((NUM_COLS * CARD_SPAN) - CARD_GAP) {
        buff.get_mut(BOARD_MARGIN_LEFT + i, row).set_style(style);
        buff.get_mut(BOARD_MARGIN_LEFT + i, row)
            .set_char(border_char);
    }

    match cursor {
        Some(i) => {
            let style = Style::default().fg(cr_color());
            buff.get_mut(BOARD_MARGIN_LEFT + (i * CARD_SPAN), row)
                .set_char(BOARD_INDICATOR_CHARS[0]);
            buff.get_mut(BOARD_MARGIN_LEFT + (i * CARD_SPAN), row)
                .set_style(style);
            buff.get_mut(BOARD_MARGIN_LEFT + (i * CARD_SPAN) + 1, row)
                .set_char(BOARD_INDICATOR_CHARS[1]);
            buff.get_mut(BOARD_MARGIN_LEFT + (i * CARD_SPAN + 1), row)
                .set_style(style);
            buff.get_mut(BOARD_MARGIN_LEFT + (i * CARD_SPAN) + 2, row)
                .set_char(BOARD_INDICATOR_CHARS[2]);
            buff.get_mut(BOARD_MARGIN_LEFT + (i * CARD_SPAN + 2), row)
                .set_style(style);
        }
        None => {}
    }

    DrawResult::Ok
}

fn draw_free_border_cells(_area: Rect, buff: &mut Buffer, state: &mut Tableau) -> DrawResult {
    let row = BOARD_MARGIN_TOP + 1;
    let border_char = char::from_u32(0x2501).expect("Unable to get HR char");
    let mut style = Style::default().bg(bg_color()).fg(dl_color());
    let mut cursor: Option<u16> = None;

    match state.mode {
        Mode::Free(i) => {
            // style.fg(hl_color());
            style = Style::default().bg(bg_color()).fg(hl_color());
            cursor = Some(i as u16);
        }
        _ => {}
    }

    for i in 0..((NUM_FREE * CARD_SPAN) - CARD_GAP) {
        buff.get_mut(BOARD_MARGIN_LEFT + (CARD_SPAN * 3) + i, row)
            .set_style(style);
        buff.get_mut(BOARD_MARGIN_LEFT + (CARD_SPAN * 3) + i, row)
            .set_char(border_char);
    }

    match cursor {
        Some(i) => {
            let style = Style::default().fg(cr_color());

            buff.get_mut(BOARD_MARGIN_LEFT + (3 * CARD_SPAN) + (i * CARD_SPAN), row)
                .set_char(FREE_INDICATOR_CHARS[0]);
            buff.get_mut(BOARD_MARGIN_LEFT + (3 * CARD_SPAN) + (i * CARD_SPAN), row)
                .set_style(style);
            buff.get_mut(
                BOARD_MARGIN_LEFT + (3 * CARD_SPAN) + (i * CARD_SPAN) + 1,
                row,
            )
            .set_char(FREE_INDICATOR_CHARS[1]);
            buff.get_mut(
                BOARD_MARGIN_LEFT + (3 * CARD_SPAN) + (i * CARD_SPAN) + 1,
                row,
            )
            .set_style(style);
            buff.get_mut(
                BOARD_MARGIN_LEFT + (3 * CARD_SPAN) + (i * CARD_SPAN) + 2,
                row,
            )
            .set_char(FREE_INDICATOR_CHARS[2]);
            buff.get_mut(
                BOARD_MARGIN_LEFT + (3 * CARD_SPAN) + (i * CARD_SPAN) + 2,
                row,
            )
            .set_style(style);
        }
        None => {}
    }

    DrawResult::Ok
}
