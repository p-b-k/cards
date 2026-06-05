////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Card Structure
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// *********************************************************************************************************************
// Some Basic Constants
// *********************************************************************************************************************

use std::fmt::{Display, Formatter, Result as FmtResult};

const USE_COLOR: bool = true;

// TODO: Once I get online pull down ansi_term
use ansi_term::{
    Color::{Black, Red, White},
    Style,
};

pub const NUM_SUITS: u8 = 4;
pub const NUM_RANKS: u8 = 13;
pub const NUM_CARDS: u8 = NUM_SUITS * NUM_RANKS;

const LTE: &str = "\u{2264}";
const LT: &str = "<";

// *********************************************************************************************************************
// Suit Definition
// *********************************************************************************************************************

#[derive(Clone, Debug, PartialEq)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Suit {
    pub fn index(&self) -> u8 {
        match self {
            Suit::Club => 0,
            Suit::Diamond => 1,
            Suit::Heart => 2,
            Suit::Spade => 3,
        }
    }

    fn _display_index(&self) -> u32 {
        ((NUM_SUITS as u32) - 1) - self.index() as u32
    }

    pub fn from_index(i: u8) -> Result<Suit, String> {
        match i {
            0 => Ok(Suit::Club),
            1 => Ok(Suit::Diamond),
            2 => Ok(Suit::Heart),
            3 => Ok(Suit::Spade),
            _ => Err(format!(
                "Suit value ({i}) is out of range [0 {LTE} i {LT} {NUM_CARDS}]"
            )),
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Suit::Spade => char::from_u32(0x2660).expect("Couldn't get Spade char"),
            Suit::Heart => char::from_u32(0x2665).expect("Couldn't get Heart char"),
            Suit::Diamond => char::from_u32(0x2666).expect("Couldn't get Diamond char"),
            Suit::Club => char::from_u32(0x2663).expect("Couldn't get Club char"),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let c: u32 = match self {
            Suit::Club => 0x2663,
            Suit::Diamond => 0x2666,
            Suit::Heart => 0x2665,
            Suit::Spade => 0x2660,
        };

        write!(f, "{}", char::from_u32(c).unwrap())
    }
}

// *********************************************************************************************************************
// Rank Definition
// *********************************************************************************************************************

#[derive(Clone, Debug, PartialEq)]
pub enum Rank {
    Ace,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn index(&self) -> u8 {
        match self {
            Rank::Ace => 0,
            Rank::C2 => 1,
            Rank::C3 => 2,
            Rank::C4 => 3,
            Rank::C5 => 4,
            Rank::C6 => 5,
            Rank::C7 => 6,
            Rank::C8 => 7,
            Rank::C9 => 8,
            Rank::C10 => 9,
            Rank::Jack => 10,
            Rank::Queen => 11,
            Rank::King => 12,
        }
    }

    pub fn from_index(i: u8) -> Result<Rank, String> {
        match i {
            0 => Ok(Rank::Ace),
            1 => Ok(Rank::C2),
            2 => Ok(Rank::C3),
            3 => Ok(Rank::C4),
            4 => Ok(Rank::C5),
            5 => Ok(Rank::C6),
            6 => Ok(Rank::C7),
            7 => Ok(Rank::C8),
            8 => Ok(Rank::C9),
            9 => Ok(Rank::C10),
            10 => Ok(Rank::Jack),
            11 => Ok(Rank::Queen),
            12 => Ok(Rank::King),
            _ => Err(format!(
                "Rank value ({i}) is out of range [0 {LTE} i {LT} {NUM_RANKS}]"
            )),
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Rank::Ace => write!(f, "A"),
            Rank::King => write!(f, "K"),
            Rank::Queen => write!(f, "Q"),
            Rank::Jack => write!(f, "J"),
            Rank::C10 => write!(f, "T"),
            _ => write!(f, "{}", self.index() + 1,),
        }
    }
}

// *********************************************************************************************************************
// Card Definition
// *********************************************************************************************************************

#[derive(Clone, Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn index(&self) -> u8 {
        (13 * self.suit.index()) + self.rank.index()
    }

    pub fn from_index(i: u8) -> Result<Card, String> {
        if i < 52 {
            let suit = Suit::from_index(i / NUM_RANKS).unwrap();
            let rank = Rank::from_index(i % NUM_RANKS).unwrap();
            Ok(Card { suit, rank })
        } else {
            Err(format!(
                "Card value ({i}) is out of range [0 {LTE} i {LT} 52]"
            ))
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // let s = self.suit.display_index();
        // let r = self.rank.display_index();

        // let c: u32 = 0x1F0A1 + 0x10 * s + r;

        // write!(f, "{}", char::from_u32(c).unwrap())
        if USE_COLOR {
            let card_body = format!("{}:{}", self.rank, self.suit);
            let color = match self.suit {
                Suit::Club => Black,
                Suit::Spade => Black,
                _ => Red,
            };
            // write!(f, "{}", White. color.paint(card_body))
            write!(
                f,
                "{}",
                Style::new().bold().on(White).fg(color).paint(card_body)
            )
        } else {
            write!(f, "{}:{}", self.rank, self.suit)
        }
    }
}

// *********************************************************************************************************************
// Unit Tests
// *********************************************************************************************************************

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_test() {
        let s_2 = Card {
            suit: Suit::Spade,
            rank: Rank::C2,
        };

        let h_k = Card {
            suit: Suit::Heart,
            rank: Rank::King,
        };

        let d_10 = Card {
            suit: Suit::Diamond,
            rank: Rank::C10,
        };

        assert_eq!(39, s_2.index());
        assert_eq!(37, h_k.index());
        assert_eq!(21, d_10.index());
    }

    #[test]
    fn from_index_test() {
        let s_2 = Card::from_index(39).unwrap();
        let h_k = Card::from_index(37).unwrap();
        let d_10 = Card::from_index(21).unwrap();

        assert_eq!(
            Card {
                suit: Suit::Spade,
                rank: Rank::C2
            },
            s_2
        );
        assert_eq!(
            Card {
                suit: Suit::Heart,
                rank: Rank::King
            },
            h_k
        );
        assert_eq!(
            Card {
                suit: Suit::Diamond,
                rank: Rank::C10
            },
            d_10
        );
    }
}
