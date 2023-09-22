mod freecell;
mod user;

use deckofcards::*;
use crate::freecell::Freecell;

/// Re-Implemented Cards::shuffle in cards.rs from deckofcards crate to
/// produce a reproducible, seeded shuffle

fn main()  {
    // Create a new Freecell board
    let mut board = Freecell::new();
    board.seed = 42 as u64;

    // Shuffle according to random seed
    // TODO: prompt the user
    board.deck.seeded_shuffle(board.seed);

    /*while deck.cards().len() > 0 {
        board.deal_one(&mut deck);
        print_freecell_board(&board)
    }*/

    crate::user::run(board);
}
