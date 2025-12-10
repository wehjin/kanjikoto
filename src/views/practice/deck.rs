use crate::core::drill_point::DrillPoint;
use crate::views::practice::card::{Card, Goal};
use rand::prelude::{SliceRandom, StdRng};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Deck {
    pub rng: StdRng,
    pub top: Card,
    pub cards: Vec<Card>,
    pub mastered: bool,
}

impl Deck {
    pub fn new(drills: Vec<DrillPoint>, mut rng: StdRng) -> Self {
        let mut cards = drills.into_iter().map(Card::new).collect::<Vec<_>>();
        let top = cards.pop().unwrap();
        let deck = Self {
            rng,
            top,
            cards,
            mastered: false,
        };
        deck
    }
    pub fn start(self) -> Self {
        let Self {
            mut rng,
            top,
            mut cards,
            ..
        } = self;
        cards.push(top);
        for card in cards.iter_mut() {
            card.goal = Goal::Learn;
        }
        cards.shuffle(&mut rng);
        let top = cards.pop().unwrap();
        Self {
            rng,
            top,
            cards,
            mastered: false,
        }
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
        let turns_remaining = turns_remaining(&self.cards);
        if turns_remaining == 0 && self.top.goal == Goal::Celebrate {
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

fn turns_remaining(cards: &Vec<Card>) -> usize {
    cards.iter().fold(0, |acc, card| match card.goal {
        Goal::Learn => acc + 2,
        Goal::Review => acc + 1,
        Goal::Celebrate => acc,
    })
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
