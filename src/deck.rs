////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Model a deck of cards
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use rand::{self, random_bool};

use crate::cards::{Card, NUM_CARDS};

const HALF_DECK: u16 = NUM_CARDS / 2;

#[derive(Clone)]
pub struct Deck {
    pub next: u16,
    pub cards: [u8; NUM_CARDS as usize],
}

impl Deck {
    pub fn new() -> Deck {
        Deck {
            cards: [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
                44, 45, 46, 47, 48, 48, 50, 51,
            ],
            next: 0,
        }
    }

    pub fn deal(&mut self) -> Option<Card> {
        if self.next < NUM_CARDS {
            let c = self.cards[self.next as usize];
            self.next = self.next + 1;
            Some(Card::from_index(c).unwrap())
        } else {
            None
        }
    }

    pub fn shuffle(&mut self, times: usize) -> &mut Deck {
        for _ in 0..times {
            self.shuffle_once();
        }

        self
    }

    pub fn shuffle_once(&mut self) -> &mut Deck {
        let mut l: Vec<u8> = Vec::new();
        let mut r: Vec<u8> = Vec::new();

        for i in 0..(HALF_DECK) {
            l.push(self.cards[i as usize]);
            r.push(self.cards[(i + (NUM_CARDS / 2)) as usize]);
        }

        let mut lx: usize = 0;
        let mut rx: usize = 0;

        for i in 0..NUM_CARDS {
            match (lx < HALF_DECK as usize, rx < HALF_DECK as usize) {
                (true, true) => {
                    if random_bool(0.5) {
                        self.cards[i as usize] = l.get(lx).unwrap().clone();
                        lx = lx + 1;
                    } else {
                        self.cards[i as usize] = r.get(rx).unwrap().clone();
                        rx = rx + 1;
                    }
                }
                (true, false) => {
                    self.cards[i as usize] = l.get(lx).unwrap().clone();
                    lx = lx + 1;
                }
                (false, true) => {
                    self.cards[i as usize] = r.get(rx).unwrap().clone();
                    rx = rx + 1;
                }
                _ => {
                    // Can't get here
                }
            }
        }

        self
    }
}
