use serde::{Deserialize, Serialize};

pub const SESSION_SIZE: usize = 5;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LessonStatus {
    pub lesson_id: i64,
    pub ready: usize,
    pub learned: usize,
}

impl LessonStatus {
    fn sessions(count: usize) -> usize {
        let sessions = count / SESSION_SIZE;
        let remaining = count % SESSION_SIZE;
        sessions + if remaining > 0 { 1 } else { 0 }
    }
    pub fn to_ready_learned(&self) -> (usize, usize) {
        (Self::sessions(self.ready), Self::sessions(self.learned))
    }
}
