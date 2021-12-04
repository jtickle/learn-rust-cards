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

    // Create a new deck of cards
    let mut deck = Deck::new();

    // Shuffle according to random seed
    // TODO: prompt the user
    predictable_shuffle(deck.mut_cards(), 42);

    for (i, card) in deck.cards().iter().enumerate() {
        println!("{:02} Card: name is '{:?}', short is {}", i, card.name(), card.to_str());
    }
}
