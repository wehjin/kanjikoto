use crate::core::drill_point::DrillPoint;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Goal {
    Learn,
    Review,
    Celebrate,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Card {
    pub goal: Goal,
    pub front: CardFront,
    pub back: CardBack,
}

impl Card {
    pub fn new(drill: DrillPoint) -> Self {
        let front = CardFront {
            kanji: drill.kanji.to_string(),
        };
        let back = CardBack {
            yomi: drill.yomi.to_string(),
            meaning: drill.meaning.to_string(),
        };
        Card {
            goal: Goal::Learn,
            front,
            back,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CardFront {
    pub kanji: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CardBack {
    pub yomi: String,
    pub meaning: String,
}
