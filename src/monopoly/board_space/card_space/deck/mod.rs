use super::{
	card::Card, kind::CardKind
};

#[derive(Debug)]
pub struct Deck {
    used: Vec<Card>,
    unused: Vec<Card>,
}

impl Deck {
    pub fn new(kind: CardKind) -> Self {
		Self { used: vec![], unused: CardKind::make_cards() }
    }
}

