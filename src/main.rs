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

pub struct Freecell {
    pub homecell: [Hand; 4],
    pub freecell: [Option<Card>; 4],
    pub field: [Hand; 8],
}

impl Default for Freecell {
    fn default() -> Self {
        Self {
            homecell: [
                Hand::new(),
                Hand::new(),
                Hand::new(),
                Hand::new(),
            ],
            freecell: [
                None,
                None,
                None,
                None,
            ],
            field: [
                Hand::new(),
                Hand::new(),
                Hand::new(),
                Hand::new(),
                Hand::new(),
                Hand::new(),
                Hand::new(),
                Hand::new(),
            ],
        }
    }
}

impl Freecell {
    pub fn new() -> Self { Self::default() }

    pub fn show_homecell(&self, i: usize) -> Option<String> {
        if self.homecell[i].len() > 0 {
            Some(self.homecell[i].cards[0].to_str().clone())
        } else {
            None
        }
    }

    pub fn show_freecell(&self, i: usize) -> Option<String> {
        match self.freecell[i] {
            Some(x) => Some(x.to_str().clone()),
            None => None,
        }
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

    let mut board = Freecell::new();

    println!("  {} {} {} {}  👑  {} {} {} {}  ", 
        board.show_homecell(0).unwrap_or(String::from("[]")),
        board.show_homecell(1).unwrap_or(String::from("[]")),
        board.show_homecell(2).unwrap_or(String::from("[]")),
        board.show_homecell(3).unwrap_or(String::from("[]")),
        board.show_freecell(0).unwrap_or(String::from("[]")),
        board.show_freecell(1).unwrap_or(String::from("[]")),
        board.show_freecell(2).unwrap_or(String::from("[]")),
        board.show_freecell(3).unwrap_or(String::from("[]")),
    );
    //println!(" {}  {}  {}  {}  {}  {}  {}  {} ")
}
