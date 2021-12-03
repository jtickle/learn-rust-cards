use deckofcards::*;

fn main() {
    let mut deck = Deck::new();

    while deck.undealt_count() > 0 {
        match deck.deal_one() {
            Ok(v) => println!("Card: {:?} {}", v, deck.undealt_count()),
            Err(e) => println!("ERROR: {:?} {}", e, deck.undealt_count()),
        }
    }

}
