////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Seahaven Tableeau TUI
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use log::error;
use tui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};

use crate::{
    deck::Deck,
    game::{NUM_FREE, Table},
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
        let mut deck = Deck::new();

        Tableau {
            mode: Mode::Build(5),
            tab: Table::from(&mut deck),
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

fn draw_free_cells(area: Rect, buff: &mut Buffer, state: &mut Tableau) -> DrawResult {
    for i in 0..NUM_FREE {
        for j in 0..3 {
            match &state.tab.free[i as usize] {
                None => {
                    buff.get_mut(
                        j + BOARD_MARGIN_LEFT + (3 * CARD_SPAN) + (i as u16 * CARD_SPAN),
                        BOARD_MARGIN_TOP + 1,
                    )
                    .set_symbol("*");
                }
                Some(c) => {
                    buff.get_mut(
                        j + BOARD_MARGIN_LEFT + (3 * CARD_SPAN) + (i as u16 * CARD_SPAN),
                        BOARD_MARGIN_TOP + 1,
                    )
                    .set_symbol("C");
                }
            }
        }
    }
    // buff.get_mut(0, 0).set_symbol("a");
    DrawResult::Err("draw_free_cells: Not Implemented".to_string())
}
fn draw_found_cells(area: Rect, buff: &mut Buffer, state: &mut Tableau) -> DrawResult {
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
