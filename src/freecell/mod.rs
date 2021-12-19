use deckofcards::*;

pub struct Freecell {
    pub homecell: [Hand; 4],
    pub freecell: [Option<Card>; 4],
    pub field: [Hand; 8],
    pub deck: Deck,
    pub curdeal: usize,
    pub minheight: usize,
    pub seed: u64,
}

impl Default for Freecell {
    fn default() -> Freecell {
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
            deck: Deck::new(),
            curdeal: 0,
            minheight: 8,
            seed: 0,
        }
    }   
}

impl Freecell {
    pub fn new() -> Self { Self::default() }

    pub fn deal_one(&mut self) -> bool {
        let n = self.deck.deal_to_hand(&mut self.field[self.curdeal % 8], 1);
        assert!(n < 2);
        if n == 1 {
            self.curdeal += 1;
            true
        } else {
            false
        }
    }

    pub fn get_homecell_topcard(&self, cell: usize) -> Option<&Card> {
        let l = self.homecell[cell].len();
        if l > 0 {
            Some(&self.homecell[cell].cards[l-1])
        } else {
            None
        }
    }

    pub fn get_freecell_card(&self, cell: usize) -> Option<&Card> {
        self.freecell[cell].as_ref()
    }

    pub fn get_field_card_at(&self, field: usize, depth: usize) -> Option<&Card> {
        if self.field[field].len() > depth {
            Some(&self.field[field].cards[depth])
        } else {
            None
        }
    }
}