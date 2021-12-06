use deckofcards::*;

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
}