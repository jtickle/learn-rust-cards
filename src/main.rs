mod freecell;
mod user;

use deckofcards::*;
use rand::prelude::*;
use rand_pcg::Pcg64;
use crate::freecell::Freecell;

// Seedable RNGs... "A simple number" looks pretty good
// https://rust-random.github.io/book/guide-seeding.html
/// Re-Implemented Cards::shuffle in cards.rs from deckofcards crate to
/// produce a reproducible, seeded shuffle
fn predictable_shuffle(cards: &mut [Card], seed: u64) {
    let mut rng = Pcg64::seed_from_u64(seed);
    // Knuth shuffle
    let l = cards.len();
    for n in 0..l {
        let i = rng.gen_range(0..(l - n));
        cards.swap(i, l - n - 1);
    }
}

fn main()  {
    // Create a new Freecell board
    let mut board = Freecell::new();
    board.seed = 42 as u64;

    // Shuffle according to random seed
    // TODO: prompt the user
    predictable_shuffle(board.deck.mut_cards(), board.seed);

    /*while deck.cards().len() > 0 {
        board.deal_one(&mut deck);
        print_freecell_board(&board)
    }*/

    crate::user::run(board);
}
