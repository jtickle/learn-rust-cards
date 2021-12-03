use deckofcards::*;
use rand::prelude::*;
use rand_pcg::Pcg64;

// Seedable RNGs... "A simple number" looks pretty good
// https://rust-random.github.io/book/guide-seeding.html
/// Re-Implemented Cards::shuffle in cards.rs from deckofcards crate to produce a reproducible, seeded shuffle
fn predictable_shuffle(cards: &mut [Card], seed: u64) {
    let mut rng = Pcg64::seed_from_u64(seed);
    // Knuth shuffle
    let l = cards.len();
    for n in 0..l {
        let i = rng.gen_range(0..(l - n));
        cards.swap(i, l - n - 1);
    }
}

fn main() {
    let mut deck = Deck::new();

    predictable_shuffle(deck.mut_cards(), 0);

    while deck.undealt_count() > 0 {
        match deck.deal_one() {
            Ok(v) => println!("{:02} Card: name is '{:?}', short is {}", deck.dealt_count(), v.name(), v.to_str()),
            Err(e) => println!("{:02} ERROR: {:?}", deck.dealt_count(), e),
        }
    }

}
