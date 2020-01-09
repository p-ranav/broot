use {
    id_arena::Id,
    super::{
        bline::BLine,
    },
    std::{
        cmp::{self, Ordering},
    },
};

pub type BId = Id<BLine>;

// a structure making it possible to keep bline references
//  sorted in a binary heap with the line with the smallest
//  score at the top
pub struct SortableBId {
    pub id: BId,
    pub score: i32,
}
impl Eq for SortableBId {}
impl PartialEq for SortableBId {
    fn eq(&self, other: &SortableBId) -> bool {
        self.score == other.score // unused but required by spec of Ord
    }
}
impl Ord for SortableBId {
    fn cmp(&self, other: &SortableBId) -> Ordering {
        if self.score == other.score {
            Ordering::Equal
        } else if self.score < other.score {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}
impl PartialOrd for SortableBId {
    fn partial_cmp(&self, other: &SortableBId) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
