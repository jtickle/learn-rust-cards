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

    pub fn from_deck(deck: &mut Deck) -> Self {
        let mut board: Self = Self::new();
        board.deal_from(deck);
        board
    }

    pub fn deal_from(&mut self, deck: &mut Deck) {
        for
        //TODO loop over deck and deal to fields
    }

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

    pub fn show_field(&self, i: usize, j: usize) -> Option<String> {
        if self.field[i].len() > j {
            Some(self.field[i].cards[j].to_str().clone())
        } else {
            None
        }
    }
}

fn print_freecell_board(board: &Freecell) {

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

    let mut maxlen: usize = 1;

    for i in 0..8 {
        let l = board.field[i].len();
        if l > maxlen {
            maxlen = l
        }
    }
    
    for i in 0..maxlen {
        println!(" {}  {}  {}  {}  {}  {}  {}  {} ",
            board.show_field(0, i).unwrap_or(String::from("  ")),
            board.show_field(1, i).unwrap_or(String::from("  ")),
            board.show_field(2, i).unwrap_or(String::from("  ")),
            board.show_field(3, i).unwrap_or(String::from("  ")),
            board.show_field(4, i).unwrap_or(String::from("  ")),
            board.show_field(5, i).unwrap_or(String::from("  ")),
            board.show_field(6, i).unwrap_or(String::from("  ")),
            board.show_field(7, i).unwrap_or(String::from("  ")),
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
        println!("{:02} Card: name is '{:?}', short is {}", i, card.name(), card.to_str());
    }

    let mut board = Freecell::new();

    print_freecell_board(&board)
}
