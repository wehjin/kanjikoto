use crate::core::data::card::{Card, Goal};
use rand::prelude::StdRng;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Deck {
    pub rng: StdRng,
    pub top: Card,
    pub cards: Vec<Card>,
    pub mastered: bool,
}

impl Deck {
    pub fn from_cards(mut cards: Vec<Card>, rng: StdRng) -> Self {
        let top = cards.pop().unwrap();
        Self {
            rng,
            top,
            cards,
            mastered: false,
        }
    }
    pub fn turns_remaining(&self) -> usize {
        let card_turns = self
            .cards
            .iter()
            .fold(0, |acc, card| acc + card.turns_remaining());
        self.top.turns_remaining() + card_turns
    }
    pub fn next(self) -> Self {
        self.fail()
    }

    pub fn fail(mut self) -> Self {
        self.top.goal = Goal::Learn;
        self.cycle_top()
    }
    pub fn pass(mut self) -> Self {
        self.top.goal = match self.top.goal {
            Goal::Learn => Goal::Review,
            Goal::Review => Goal::Celebrate,
            Goal::Celebrate => Goal::Celebrate,
        };
        let turns_remaining = self.turns_remaining();
        if turns_remaining == 0 {
            self.mastered = true;
            self
        } else {
            self.cycle_top()
        }
    }
    fn cycle_top(self) -> Self {
        let Self {
            rng, top, cards, ..
        } = self;
        let (mut cards, new_top) = find_top(cards);
        match new_top {
            Some(new_top) => {
                cards.push(top);
                Self {
                    rng,
                    top: new_top,
                    cards,
                    mastered: false,
                }
            }
            None => Self {
                rng,
                top,
                cards,
                mastered: false,
            },
        }
    }
}

fn find_top(cards: Vec<Card>) -> (Vec<Card>, Option<Card>) {
    let (cards, learn) = remove_card_with_goal(cards, Goal::Learn);
    if let Some(learn) = learn {
        return (cards, Some(learn));
    }

    let (cards, review) = remove_card_with_goal(cards, Goal::Review);
    if let Some(review) = review {
        return (cards, Some(review));
    }

    let (cards, celebrate) = remove_card_with_goal(cards, Goal::Celebrate);
    if let Some(celebrate) = celebrate {
        return (cards, Some(celebrate));
    }
    (cards, None)
}

fn remove_card_with_goal(mut cards: Vec<Card>, goal: Goal) -> (Vec<Card>, Option<Card>) {
    if let Some(position) = cards.iter().position(|card| card.goal == goal) {
        let new_top = cards.remove(position);
        return (cards, Some(new_top));
    }
    (cards, None)
}
