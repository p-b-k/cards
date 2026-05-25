////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Card Structure
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// *********************************************************************************************************************
// Some Basic Constants
// *********************************************************************************************************************

use std::fmt::{Display, Formatter, Result as FmtResult};

const USE_COLOR: bool = false;

// TODO: Once I get online pull down ansi_term
// use ansi_term::{
//     Color::{Black, Blue, Green, Red, Yellow},
//     Style,
// };

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

    fn display_index(&self) -> u32 {
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
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let c: u32 = match self {
            Suit::Club => 0x2663,
            Suit::Diamond => 0x2662,
            Suit::Heart => 0x2661,
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
    Ace,
}

impl Rank {
    pub fn index(&self) -> u8 {
        match self {
            Rank::C2 => 0,
            Rank::C3 => 1,
            Rank::C4 => 2,
            Rank::C5 => 3,
            Rank::C6 => 4,
            Rank::C7 => 5,
            Rank::C8 => 6,
            Rank::C9 => 7,
            Rank::C10 => 8,
            Rank::Jack => 9,
            Rank::Queen => 10,
            Rank::King => 11,
            Rank::Ace => 12,
        }
    }

    fn display_index(&self) -> u32 {
        match self {
            Rank::Ace => 0,
            _ => {
                let mut extra: u32 = 0;
                if self.index() > Rank::Jack.index() {
                    extra = 1;
                }

                1 + self.index() as u32 + extra
            }
        }
    }

    pub fn from_index(i: u8) -> Result<Rank, String> {
        match i {
            0 => Ok(Rank::C2),
            1 => Ok(Rank::C3),
            2 => Ok(Rank::C4),
            3 => Ok(Rank::C5),
            4 => Ok(Rank::C6),
            5 => Ok(Rank::C7),
            6 => Ok(Rank::C8),
            7 => Ok(Rank::C9),
            8 => Ok(Rank::C10),
            9 => Ok(Rank::Jack),
            10 => Ok(Rank::Queen),
            11 => Ok(Rank::King),
            12 => Ok(Rank::Ace),
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
            _ => write!(f, "{}", self.index() + 2,),
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
            // let mut color = match self.suit {
            //     Suit::Club => Black,
            //     Suit::Spade => Black,
            //     _ => Red,
            // };
            write!(f, "{card_body}")
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
