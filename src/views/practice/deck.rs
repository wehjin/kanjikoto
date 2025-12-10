use crate::core::drill_point::DrillPoint;
use crate::views::practice::card::Card;
use rand::prelude::{SliceRandom, StdRng};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Deck {
    pub rng: StdRng,
    pub top: Card,
    pub learn: Vec<Card>,
    pub review: Vec<Card>,
    pub celebrate: Vec<Card>,
}

impl Deck {
    pub fn new(drills: Vec<DrillPoint>, mut rng: StdRng) -> Self {
        let mut cards = drills.into_iter().map(Card::new).collect::<Vec<_>>();
        cards.shuffle(&mut rng);
        let top = cards.pop().unwrap();
        let deck = Self {
            rng,
            top,
            learn: cards,
            review: vec![],
            celebrate: vec![],
        };
        deck
    }
    pub fn next(self) -> Self {
        self
    }
    pub fn fail(self) -> Self {
        self
    }
    pub fn pass(self) -> Self {
        self
    }
}
