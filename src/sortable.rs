use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct Sortable<T: PartialOrd + Copy>(pub T);

impl<T: PartialOrd + Copy> Eq for Sortable<T> {}

impl<T: PartialOrd + Copy> PartialOrd for Sortable<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: PartialOrd + Copy> Ord for Sortable<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}
