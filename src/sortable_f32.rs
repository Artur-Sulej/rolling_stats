use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct SortableF32(pub f32);

impl Eq for SortableF32 {}

impl PartialOrd for SortableF32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SortableF32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}
