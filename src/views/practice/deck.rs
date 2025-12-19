use crate::core::data::card::{Card, Goal};
use rand::prelude::{SliceRandom, StdRng};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Stats {
    pub learned: usize,
    pub failed: usize,
    pub repeated: usize,
    pub passed: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Deck {
    pub rng: StdRng,
    pub top: Card,
    cards: Vec<Card>,
    pub stats: Stats,
}

impl Deck {
    pub fn from_cards(mut cards: Vec<Card>, rng: StdRng) -> Self {
        let top = cards.pop().unwrap();
        Self {
            rng,
            top,
            cards,
            stats: Stats {
                learned: 0,
                failed: 0,
                repeated: 0,
                passed: 0,
            },
        }
    }
    pub fn into_cards(self) -> Vec<Card> {
        let mut cards = self.cards;
        cards.push(self.top);
        cards
    }
    pub fn is_all_passed(&self) -> bool {
        self.stats.passed == self.cards.len() + 1 /* for the top card */
    }
    pub fn turns_remaining(&self) -> usize {
        let card_turns = self
            .cards
            .iter()
            .fold(0, |acc, card| acc + card.turns_remaining());
        self.top.turns_remaining() + card_turns
    }
    pub fn learn(mut self) -> Self {
        self.stats.learned += 1;
        self.top.goal = Goal::Learn;
        self.cycle_top()
    }

    pub fn fail(mut self) -> Self {
        self.stats.failed += 1;
        self.top.goal = Goal::Learn;
        self.cycle_top()
    }
    pub fn repeat(mut self) -> Self {
        self.stats.repeated += 1;
        self.top.goal = Goal::Review;
        self.cycle_top()
    }
    pub fn pass(mut self) -> Self {
        self.stats.passed += 1;
        self.top.goal = Goal::Celebrate;
        self.cycle_top()
    }
    fn cycle_top(mut self) -> Self {
        self.cards.shuffle(&mut self.rng);
        if let Some(position) = self
            .cards
            .iter()
            .position(|card| card.goal != Goal::Celebrate)
        {
            let new_top = self.cards.remove(position);
            self.cards.push(self.top);
            self.top = new_top;
        }
        self
    }
}
