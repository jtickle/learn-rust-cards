mod freecell;

use deckofcards::*;
use rand::prelude::*;
use rand_pcg::Pcg64;
use crate::freecell::*;
use colored::*;

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

fn suit_to_uc_outline(suit: &Suit) -> char {
    match suit {
        deckofcards::Suit::Hearts => '♥',
        deckofcards::Suit::Spades => '♠',
        deckofcards::Suit::Diamonds => '♦',
        deckofcards::Suit::Clubs => '♣',
    }
}

fn suit_to_uc_fill(suit: &Suit) -> char {
    match suit {
        deckofcards::Suit::Hearts => '♥',
        deckofcards::Suit::Spades => '♠',
        deckofcards::Suit::Diamonds => '♦',
        deckofcards::Suit::Clubs => '♣',
    }
}

fn show_card(card: &Card) -> String {
    // TODO: cli display configuration
    let disp = format!("{}{}", card.rank.to_char(), suit_to_uc_fill(&card.suit));
    if card.is_hearts() || card.is_diamonds() {
        disp.red().to_string()
    } else {
        disp
    }
}

fn card_to_str_or(maybe: Option<Card>, def: &str) -> String{
    match maybe {
        Some(card) => show_card(&card),
        None => String::from(def)
    }
}

fn print_freecell_board(board: &Freecell) {

    println!(" {} {} {} {}  👑  {} {} {} {} ", 
        card_to_str_or(board.get_homecell_topcard(0), "[]"),
        card_to_str_or(board.get_homecell_topcard(1), "[]"),
        card_to_str_or(board.get_homecell_topcard(2), "[]"),
        card_to_str_or(board.get_homecell_topcard(3), "[]"),
        card_to_str_or(board.get_freecell_card(0), "[]"),
        card_to_str_or(board.get_freecell_card(1), "[]"),
        card_to_str_or(board.get_freecell_card(2), "[]"),
        card_to_str_or(board.get_freecell_card(3), "[]"),
    );

    let mut maxlen: usize = 1;

    for i in 0..8 {
        let l = board.field[i].len();
        if l > maxlen {
            maxlen = l
        }
    }
    
    for i in 0..maxlen {
        println!(" {}  {}  {}  {}  {}  {}  {}  {} ",
            card_to_str_or(board.get_field_card_at(0, i), "  "),
            card_to_str_or(board.get_field_card_at(1, i), "  "),
            card_to_str_or(board.get_field_card_at(2, i), "  "),
            card_to_str_or(board.get_field_card_at(3, i), "  "),
            card_to_str_or(board.get_field_card_at(4, i), "  "),
            card_to_str_or(board.get_field_card_at(5, i), "  "),
            card_to_str_or(board.get_field_card_at(6, i), "  "),
            card_to_str_or(board.get_field_card_at(7, i), "  "),
        )
    }
}

fn main() {

    // Create a new deck of cards
    let mut deck = Deck::new();

    // Shuffle according to random seed
    // TODO: prompt the user
    predictable_shuffle(deck.mut_cards(), 42);

    for (i, card) in deck.cards().iter().enumerate() {
        println!("{:02} Card: name is '{}', short is {}", i, card.name(), show_card(card));
    }

    let mut board = Freecell::new();

    while deck.cards().len() > 0 {
        board.deal_one(&mut deck);
        print_freecell_board(&board)
    }
}
