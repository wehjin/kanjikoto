use crate::core::drill_point::DrillPoint;
use crate::views::practice::card::Card;
use rand::prelude::StdRng;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DeckStatus {
    Prompt,
    Learn,
    Check,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Deck {
    pub status: DeckStatus,
    pub top: Card,
    pub learn: Vec<Card>,
    pub review: Vec<Card>,
    pub celebrate: Vec<Card>,
}

impl Deck {
    pub fn new(drills: Vec<DrillPoint>, rng: &mut StdRng) -> Self {
        let mut cards = drills.into_iter().map(Card::new).collect::<Vec<_>>();
        let top = cards.pop().unwrap();
        Self {
            status: DeckStatus::Prompt,
            top,
            learn: cards,
            review: vec![],
            celebrate: vec![],
        }
    }
}
