////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Main
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use seahaven::{
    cards::{Card, NUM_RANKS, NUM_SUITS},
    deck::Deck,
};

pub fn main() {
    let mut d = Deck::new();

    d.shuffle_once()
        .shuffle_once()
        .shuffle_once()
        .shuffle_once()
        .shuffle_once()
        .shuffle_once()
        .shuffle_once();

    for s in 0..NUM_SUITS {
        for r in 0..NUM_RANKS {
            let c = Card::from_index(d.cards[(NUM_RANKS * s + r) as usize]).unwrap();
            print!("{c} ");
        }
        println!();
    }
}
