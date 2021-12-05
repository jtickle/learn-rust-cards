use deckofcards::*;
use rand::prelude::*;
use rand_pcg::Pcg64;

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

    //pub fn from_deck(deck: &mut Deck) -> Self {
    //    let mut board: Self = Self::new();
    //    board.deal_from(deck);
    //    board
    //}
//
    //pub fn deal_from(&mut self, deck: &mut Deck) {
    //    for
    //    //TODO loop over deck and deal to fields
    //}

    pub fn get_homecell_topcard(&self, cell: usize) -> Option<Card> {
        let l = self.homecell[cell].len();
        if l > 0 {
            Some(self.homecell[cell].cards[l-1])
        } else {
            None
        }
    }

    pub fn get_freecell_card(&self, cell: usize) -> Option<Card> {
        return self.freecell[cell]
    }

    pub fn get_field_card_at(&self, field: usize, depth: usize) -> Option<Card> {
        if self.field[field].len() > depth {
            Some(self.field[field].cards[depth])
        } else {
            None
        }
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

fn card_to_str_or(maybe: Option<Card>, def: &str) -> String{
    match maybe {
        Some(card) => card.to_str().clone(),
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
    format!("{}{}", card.rank.to_char(), suit_to_uc_fill(&card.suit))
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

    print_freecell_board(&board)
}
