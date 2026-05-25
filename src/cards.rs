////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Card Structure
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// *********************************************************************************************************************
// Some Basic Constants
// *********************************************************************************************************************

const NUM_SUITS: u8 = 4;
const NUM_RANKS: u8 = 13;
const NUM_CARDS: u8 = NUM_SUITS * NUM_RANKS;

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
            Rank::C8 => 6,
            Rank::C9 => 7,
            Rank::C10 => 8,
            Rank::Jack => 9,
            Rank::Queen => 10,
            Rank::King => 11,
            Rank::Ace => 12,
        }
    }

    pub fn from_index(i: u8) -> Result<Rank, String> {
        match i {
            0 => Ok(Rank::C2),
            1 => Ok(Rank::C3),
            2 => Ok(Rank::C4),
            3 => Ok(Rank::C5),
            4 => Ok(Rank::C6),
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
