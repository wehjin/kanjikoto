use crate::components::hint;
use crate::components::hint::Hint;
use crate::core::api::DrillPoint;

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
            answer: Answer::new(index, hint::get_hints(&drill)),
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
