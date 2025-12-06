use crate::core::drill_point::DrillPoint;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Lesson {
    pub index: usize,
    pub prompt: String,
    pub answer: Answer,
}

impl Lesson {
    pub fn new(index: usize, drill: DrillPoint) -> Self {
        Self {
            index,
            prompt: drill.kanji.clone(),
            answer: Answer::new(index, get_hints(&drill)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Answer {
    pub lesson_index: usize,
    pub hints: Vec<Hint>,
    pub visible: bool,
}

impl Answer {
    pub fn new(lesson_index: usize, hints: Vec<Hint>) -> Self {
        Self {
            lesson_index,
            hints,
            visible: false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hint {
    pub style: HintStyle,
    pub text: String,
}
impl Hint {
    pub fn definition(text: String) -> Self {
        Self {
            style: HintStyle::Definition,
            text,
        }
    }
    pub fn reading(text: String) -> Self {
        Self {
            style: HintStyle::Reading,
            text,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum HintStyle {
    Definition,
    Reading,
}

fn get_hints(drill: &DrillPoint) -> Vec<Hint> {
    let definitions = drill
        .to_meanings()
        .into_iter()
        .map(Hint::definition)
        .collect::<Vec<_>>();
    let yomi = Hint::reading(drill.yomi.clone());
    vec![yomi]
        .into_iter()
        .chain(definitions)
        .collect::<Vec<_>>()
}
