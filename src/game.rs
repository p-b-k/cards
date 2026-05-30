////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Create the data structures for the seahaven game
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::{
    cards::{Card, NUM_CARDS, NUM_SUITS, Rank, Suit},
    deck::Deck,
};

#[derive(Clone, Debug)]
pub struct Stack {
    pub cards: Vec<Card>,
}

const NUM_FREE: u8 = 4;
const NUM_COLS: u8 = 10;
const NUM_ROWS: u8 = NUM_CARDS / NUM_COLS;

#[derive(Clone, Debug)]
pub struct Table {
    pub found: [u8; NUM_SUITS as usize],
    pub free: [Option<Card>; NUM_FREE as usize],
    pub blds: [Vec<Card>; NUM_COLS as usize],
}

#[derive(Clone, Debug)]
pub enum Location {
    Found(Suit),
    Free(u8),
    Builds(u8),
}

impl Table {
    pub fn from(deck: &mut Deck) -> Table {
        let mut table = Table {
            found: [0, 0, 0, 0],
            free: [None, None, None, None],
            blds: [
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ],
        };

        for _ in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                table.blds[c as usize].push(deck.deal().unwrap());
            }
        }

        table.free[1] = Some(deck.deal().unwrap());
        table.free[2] = Some(deck.deal().unwrap());

        table
    }

    // Found(Suit),
    // Free(u8),
    // Builds(u8),
    pub fn move_card(&mut self, from: Location, to: Location) -> Option<String> {
        match (from, to) {
            (Location::Builds(c), Location::Found(s)) => {
                let vec = &self.blds[c as usize];

                if vec.is_empty() {
                    Some(format!("Cannot move from an empty column ({c})"))
                } else {
                    let top = vec[vec.len() - 1].clone();
                    if top.suit == s {
                        if self.found[s.index() as usize] == top.rank.index() {
                            self.found[s.index() as usize] = self.found[s.index() as usize] + 1;
                            self.blds[c as usize].pop();
                            None
                        } else {
                            Some(format!(
                                "Card (top:?) does not have the correct rank to be moved to the foundation ({:?})",
                                Rank::from_index(self.found[s.index() as usize])
                            ))
                        }
                    } else {
                        Some(format!(
                            "Card (top:?) does not match foundation suite (s:?)"
                        ))
                    }
                }
            }
            (Location::Builds(cf), Location::Builds(ct)) => {
                let fvec = &self.blds[cf as usize];
                let tvec = &self.blds[ct as usize];

                if fvec.is_empty() {
                    Some(format!("Cannot move from an empty column ({cf})"))
                } else {
                    let ftop = fvec[fvec.len() - 1].clone();
                    if tvec.is_empty() {
                        if ftop.rank == Rank::King {
                            let k = &self.blds[cf as usize].pop().unwrap();
                            let _ = &self.blds[ct as usize].push(k.clone());
                            None
                        } else {
                            Some("Only a King can be moved to an empty column".to_string())
                        }
                    } else {
                        let ttop = tvec[tvec.len() - 1].clone();
                        if ftop.suit == ttop.suit {
                            if ftop.rank.index() + 1 == ttop.rank.index() {
                                let n = &self.blds[cf as usize].pop().unwrap();
                                let _ = &self.blds[ct as usize].push(n.clone());
                                None
                            } else {
                                Some(format!(
                                    "Card ({ftop:?}) is not one below the rank of the target card ({ttop:?})"
                                ))
                            }
                        } else {
                            Some(format!(
                                "Card ({ftop:?}) does not match the suite of it's target card ({ttop:?})"
                            ))
                        }
                    }
                }
            }
            (Location::Builds(c), Location::Free(s)) => {
                let vec = &self.blds[c as usize];

                if vec.is_empty() {
                    Some(format!("Cannot move from an empty column ({c})"))
                } else {
                    match self.free[s as usize] {
                        Some(_) => Some(format!("Cannot move to an occupied Free Cell")),
                        None => {
                            let top = self.blds[c as usize].pop().unwrap();
                            self.free[s as usize] = Some(top);
                            None
                        }
                    }
                }
            }
            (Location::Free(n), Location::Builds(c)) => match self.free[n as usize].clone() {
                None => Some(format!("Cannot move from empty Free Cell")),
                Some(card) => {
                    let vec = &self.blds[c as usize];
                    if vec.is_empty() {
                        if card.rank == Rank::King {
                            let _ = &self.blds[c as usize].push(card);
                            self.free[n as usize] = None;
                            None
                        } else {
                            Some(format!("Only a king can be moved to an empty column"))
                        }
                    } else {
                        let top = vec[vec.len() - 1].clone();
                        if top.suit == card.suit {
                            if top.rank.index() == (card.rank.index() + 1) {
                                let _ = &self.blds[c as usize].push(card);
                                self.free[n as usize] = None;
                                None
                            } else {
                                Some(format!("A card must build on the next highest rank"))
                            }
                        } else {
                            Some(format!("A card must build on the same suite"))
                        }
                    }
                }
            },
            (Location::Free(n), Location::Found(s)) => {
                // Some(format!("TODO: Implement move from free cell to foundation"))
                match self.free[n as usize].clone() {
                    Some(card) => {
                        if card.suit == s {
                            if card.rank.index() == self.found[s.index() as usize] {
                                Some(format!(
                                    "Card (top:?) does not have the correct rank to be moved to the foundation ({:?})",
                                    Rank::from_index(self.found[s.index() as usize])
                                ))
                            } else {
                                self.found[s.index() as usize] = self.found[s.index() as usize] + 1;
                                self.free[n as usize] = None;
                                None
                            }
                        } else {
                            Some("Card (top:?) does not match foundation suite (s:?)".to_string())
                        }
                    }
                    None => Some(format!("Cannot move from empty Free Cell")),
                }
            }
            _ => Some("Unknown Move".to_string()),
        }
    }

    pub fn print(&self) {
        println!(
            "{} {}     {} {} {} {}     {} {}",
            if self.found[0] == 0 {
                format!("[{}]", Suit::Club)
            } else {
                format!(
                    "{}",
                    Card {
                        rank: Rank::from_index(self.found[0] - 1).unwrap(),
                        suit: Suit::Club
                    }
                )
            },
            if self.found[1] == 0 {
                format!("[{}]", Suit::Diamond)
            } else {
                format!(
                    "{}",
                    Card {
                        rank: Rank::from_index(self.found[1] - 1).unwrap(),
                        suit: Suit::Diamond
                    }
                )
            },
            match &self.free[0] {
                Some(c) => format!("{c}"),
                None => "[ ]".to_string(),
            },
            match &self.free[1] {
                Some(c) => format!("{c}"),
                None => "[ ]".to_string(),
            },
            match &self.free[2] {
                Some(c) => format!("{c}"),
                None => "[ ]".to_string(),
            },
            match &self.free[3] {
                Some(c) => format!("{c}"),
                None => "[ ]".to_string(),
            },
            if self.found[2] == 0 {
                format!("[{}]", Suit::Heart)
            } else {
                format!(
                    "{}",
                    Card {
                        rank: Rank::from_index(self.found[2] - 1).unwrap(),
                        suit: Suit::Heart
                    }
                )
            },
            if self.found[3] == 0 {
                format!("[{}]", Suit::Spade)
            } else {
                format!(
                    "{}",
                    Card {
                        rank: Rank::from_index(self.found[3] - 1).unwrap(),
                        suit: Suit::Spade
                    }
                )
            },
        );

        println!();

        let mut all_done = false;
        let mut current_row = 0;

        while !all_done {
            all_done = true;

            for i in 0..NUM_COLS {
                let col = &self.blds[i as usize];
                if current_row < col.len() {
                    print!("{} ", col[current_row]);
                    all_done = false;
                } else {
                    print!("    ");
                }
            }

            println!();

            current_row = current_row + 1;
        }
    }
}
