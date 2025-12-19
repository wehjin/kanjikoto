use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Goal {
    Learn,
    Review,
    Celebrate,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub id: i64,
    pub goal: Goal,
    pub front: CardFront,
    pub back: CardBack,
}

impl Card {
    pub fn turns_remaining(&self) -> usize {
        match self.goal {
            Goal::Learn => 1,
            Goal::Review => 1,
            Goal::Celebrate => 0,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CardFront {
    pub kanji: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CardBack {
    pub yomi: String,
    pub meaning: String,
}
